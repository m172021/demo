use super::*;

// ==== Interface

pub enum MsgTo_Node_UrlMap {
    Insert(Vec<Url>),
    Get(Vec<Url256>),
}

pub enum MsgFrom_Node_UrlMap {
    Insert_Res(Vec<Result<(Url, Url256), Node_UrlMap_Conflict>>),
    Get_Res(Vec<(Url256, Option<Url>)>),
}

// ==== Impl

pub struct Node_UrlMap {
    data: HashMap<Url256, Url>,
    shard_group: Node_UrlMap_SG,
}

impl Node_Config_T for Node_UrlMap {
    type MsgTo = MsgTo_Node_UrlMap;
    type MsgFrom = MsgFrom_Node_UrlMap;

    fn process(&mut self, msg: Self::MsgTo) -> Self::MsgFrom {
        match msg {
            MsgTo_Node_UrlMap::Insert(v) => {
                let mut out = Vec::with_capacity(v.len());
                for x in v.into_iter() {
                    let x256 = x.to_256();
                    match self.data.get(&x256) {
                        None => {
                            self.data.insert(x256, x.clone());
                            out.push(Ok((x, x256)));
                        }
                        Some(url_old) => out.push(Err(Node_UrlMap_Conflict {
                            url_old: url_old.clone(),
                            url_new: x,
                            url256: x256,
                        })),
                    }
                }
                MsgFrom_Node_UrlMap::Insert_Res(out)
            }
            MsgTo_Node_UrlMap::Get(v) => {
                let out = v.iter().map(|x| (*x, self.data.get(&x).cloned()));
                MsgFrom_Node_UrlMap::Get_Res(out.collect::<Vec<_>>())
            }
        }
    }
}

pub struct Node_UrlMap_SG(ShardGroup<1>);

impl Node_UrlMap {
    pub fn new(shard_group: Node_UrlMap_SG) -> Node_UrlMap {
        Node_UrlMap {
            data: HashMap::new(),
            shard_group,
        }
    }
}

pub struct Node_UrlMap_Conflict {
    pub(crate) url_old: Url,
    pub(crate) url_new: Url,
    pub(crate) url256: Url256,
}
