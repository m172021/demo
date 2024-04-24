use super::*;

// ==== Interface

pub enum MsgTo_Node_CheckFetchStatus {
    Query(Vec<Url256>),
    MarkDone(Vec<Url256>),
}

pub enum MsgFrom_Node_CheckFetchStatus {
    Query_Res(Vec<Url256>), // returns only the ones that need to be fetched
    MarkDone_Res,
}

// ==== Impl

pub struct Node_CheckFetchStatus {
    done: HashSet<Url256>,
    // this is out-dated in one direction
    //   in hashset -> definitely fetched
    //   not in hashset -> may or may not be fetched
    shard_group: Node_CheckFetchStatus_SG,
}

impl Node_Config_T for Node_CheckFetchStatus {
    type MsgTo = MsgTo_Node_CheckFetchStatus;
    type MsgFrom = MsgFrom_Node_CheckFetchStatus;

    fn process(&mut self, msg: Self::MsgTo) -> Self::MsgFrom {
        match msg {
            MsgTo_Node_CheckFetchStatus::MarkDone(v) => {
                v.iter().for_each(|x| drop(self.done.insert(*x)));
                Self::MsgFrom::MarkDone_Res
            }
            MsgTo_Node_CheckFetchStatus::Query(v) => {
                let out = v.into_iter().filter(|x| !self.done.contains(x));
                Self::MsgFrom::Query_Res(out.collect::<Vec<_>>())
            }
        }
    }
}

pub struct Node_CheckFetchStatus_SG(ShardGroup<2>);

// Node

impl Node_CheckFetchStatus {
    pub fn new(shard_group: Node_CheckFetchStatus_SG) -> Node_CheckFetchStatus {
        Node_CheckFetchStatus {
            done: Default::default(),
            shard_group: shard_group,
        }
    }
}
