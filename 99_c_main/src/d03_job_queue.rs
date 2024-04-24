use super::*;

// ==== Interface

pub enum MsgTo_Node_JobQueue {
    Enqueue(Vec<Url256>),
    TakeJobs(usize),
    Done(Vec<Url256>),
    Tick_10m(), // call this once every 10 mins to check on stalled jobs
}

pub enum MsgFrom_Node_JobQueue {
    Enqueue_Res(),
    TakeJobs_Res(Vec<Url256>),
    Done_Res(),
    Tick_10m_Res(),
}

// ==== Impl

pub struct Node_JobQueue {
    need_to_fetch: VecDeque<Url256>,
    fetching_cur: HashSet<Url256>, // currently in flight
    fetching_old: HashSet<Url256>, // stuff that might have stalled
    all: HashSet<Url256>,          // union of need_to_fetch and fetching
    ttl_insert: u64,               // need this to check for when job queue is empty
    shard_group: Node_JobQueue_SG,
}

impl Node_Config_T for Node_JobQueue {
    type MsgTo = MsgTo_Node_JobQueue;
    type MsgFrom = MsgFrom_Node_JobQueue;

    fn process(&mut self, msg: Self::MsgTo) -> Self::MsgFrom {
        match msg {
            MsgTo_Node_JobQueue::Enqueue(v) => {
                v.iter().for_each(|x| self.push(x));
                Self::MsgFrom::Enqueue_Res()
            }
            MsgTo_Node_JobQueue::TakeJobs(n) => {
                let out = self.need_to_fetch.drain(0..n).collect::<Vec<_>>();
                out.iter().for_each(|x| drop(self.fetching_cur.insert(*x)));
                Self::MsgFrom::TakeJobs_Res(out)
            }
            MsgTo_Node_JobQueue::Done(v) => {
                v.iter().for_each(|x| self.mark_done(x));
                Self::MsgFrom::Done_Res()
            }
            MsgTo_Node_JobQueue::Tick_10m() => {
                let cur = std::mem::replace(&mut self.fetching_cur, HashSet::new());
                let old = std::mem::replace(&mut self.fetching_old, cur);
                for x in old.into_iter() {
                    // these jobs have not finished in 10 mins
                    // we assume stall and re-que them
                    self.need_to_fetch.push_back(x);
                }
                Self::MsgFrom::Tick_10m_Res()
            }
        }
    }
}

pub struct Node_JobQueue_SG(ShardGroup<2>);

impl Node_JobQueue {
    pub fn mark_done(&mut self, x: &Url256) {
        self.fetching_cur.remove(&x);
        self.all.remove(&x);
    }

    pub fn push(&mut self, x: &Url256) {
        if !self.all.contains(x) {
            self.need_to_fetch.push_back(*x);
            self.all.insert(*x);
        }
    }

    pub fn new(shard_group: Node_JobQueue_SG) -> Node_JobQueue {
        Node_JobQueue {
            need_to_fetch: VecDeque::new(),
            all: HashSet::new(),
            fetching_cur: HashSet::new(),
            shard_group: shard_group,
            fetching_old: HashSet::default(),
            ttl_insert: 0,
        }
    }
}

pub enum JobQueue_Res {
    Fetched(Url256),
    NeedToFetch(Url256),
}
