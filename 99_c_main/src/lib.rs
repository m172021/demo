#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

// This is a DRAFT. I have NEVER written a crawler before.

// Known Flaws
//
//   * Heap alloc all over the place.
//       Out of memory = crash.
//       Maybe alloc all memory up front, return error before OOM ?
//
//   * Clone/Copy all over the place.
//       Can definitely reduce copying.
//
//   * 1 Job = Fetch (Http Get) & Extract (extract URL) are entertwined
//       Should probably split into Fetch (network IO heavy)
//       and extract (CPU heavy)
//
//   * No idea how good lihnk extractor is.
//       Should we be using selenium  ?
//

// Design Goals:

// 1. Skipped URL = Error. Duplicate Work = Inefficient.
// 2. Assume everything can fail.
// 3. Parallelize network delays via async. Try to operate at limits of Network IO

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hasher;

use x_my_proc_macro::*;

mod d00_url;
mod d01_url_map;
mod d02_check_fetched;
mod d03_job_queue;
mod d04_fetcher_async;
mod d99_cluster_manager;
pub use d00_url::*;
pub use d01_url_map::*;
pub use d02_check_fetched::*;
pub use d03_job_queue::*;
pub use d04_fetcher_async::*;
pub use d99_cluster_manager::*;

pub struct Node_Addr {
    ipv4: std::net::Ipv4Addr,
    port: u32,
}

pub trait Node_Config_T {
    type MsgTo;
    type MsgFrom;

    fn process(&mut self, msg: Self::MsgTo) -> Self::MsgFrom;
}

pub struct Boss_UrlStatus {
    queued: HashSet<Url>,
    fetched: HashSet<Url>,
}

pub enum Job_Preference {
    Fetch, // Worker wants Fetcn because it has more NetworkIO than CPU
    Parse, // Worker wants Parse because it has more CPU than NetworkIO
}

pub enum MsgTo_Worker {
    Fetch { target_utl: String },
}

pub enum MsgTo_Storage {
    Store { target_url: String, data: Vec<u8> },
}

pub struct MsgTo_Parser {}

pub enum MsgTo_KeyCache {
    StoredUrl { target_url: String },
}

pub enum Msg_Global {
    Fetcher(MsgTo_Worker),
    Storage(MsgTo_Storage),
    Parser(MsgTo_Parser),
    KeyCache(MsgTo_KeyCache),
}

#[test]
fn test_00() {}
