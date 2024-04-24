use super::*;

pub struct ClusterManager {
    check_fetched: HashMap<Node_CheckFetchStatus_SG, std::net::Ipv4Addr>,
    urlmap: HashMap<Node_UrlMap_SG, std::net::Ipv4Addr>,
}
