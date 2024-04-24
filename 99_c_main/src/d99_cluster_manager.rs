use super::*;

pub struct ClusterManager {
    check_fetched: HashMap<Node_CheckFetchStatus_SG, Node_Addr>,
    urlmap: HashMap<Node_UrlMap_SG, Node_Addr>,
}
