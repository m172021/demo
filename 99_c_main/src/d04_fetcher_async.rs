use super::*;
use std::cell::Cell;

use std::sync::{Arc, Mutex};
// use std::time::Duration;

// ==== Interface

// ==== Impl

#[derive(Clone)]
pub struct UrlQueue {
    job_queue: Arc<Mutex<VecDeque<Url>>>,
    reqs_in_flight: Arc<Mutex<usize>>,
}

impl UrlQueue {
    pub fn take(&self) -> Option<Url> {
        let v = {
            let mut t = self.job_queue.lock().unwrap();
            t.pop_front()
        };
        v
    }

    pub fn new() -> UrlQueue {
        UrlQueue {
            job_queue: Arc::new(Mutex::new(VecDeque::new())),
            reqs_in_flight: Arc::new(Mutex::new(0)),
        }
    }

    pub fn len(&self) -> usize {
        self.job_queue.lock().unwrap().len()
    }

    pub fn len_plus_inflight(&self) -> usize {
        let t0 = self.job_queue.lock().unwrap();
        let t1 = self.reqs_in_flight.lock().unwrap();
        t0.len() + *t1
    }

    pub fn req_n(&self, n: usize) {
        // send msg requesting n jobs
        let mut t = self.reqs_in_flight.lock().unwrap();
        *t = *t + n;
    }

    pub fn push_queue(&self, jobs: Vec<Url>) {
        let mut t0 = self.job_queue.lock().unwrap();
        let mut t1 = self.reqs_in_flight.lock().unwrap();
        *t1 = *t1 - jobs.len();
        t0.extend(jobs);
    }
}

pub struct Node_Fetcher {
    job_queue: UrlQueue,

    num_tasks: usize,
}

impl Node_Config_T for Node_Fetcher {
    type MsgTo = ();
    type MsgFrom = ();

    fn process(&mut self, msg: Self::MsgTo) -> Self::MsgFrom {
        // match msg {}
    }
}

pub struct Node_Fetcher_SG(ShardGroup<2>);

pub struct Document {
    url: Url,
    text: String,
    links: Vec<String>,
}

// Node

impl Node_Fetcher {
    pub fn new(num_tasks: usize) -> Node_Fetcher {
        let out = Node_Fetcher {
            job_queue: UrlQueue::new(),
            num_tasks,
        };

        for _ in 0..num_tasks {
            let jq = out.job_queue.clone();
            let t = async move {
                loop {
                    match jq.take() {
                        None => {
                            // ugly hack: polling is bad
                            // 100ms delay tolerable vs network io delay
                            // async sleep doesn't pause thread, only suspends async task
                            tokio::time::sleep(std::time::Duration::from_millis(50)).await
                        }
                        Some(url) => {
                            let res = Self::fetch_url(url).await;
                            // handle result
                        }
                    }
                }
            };
            // spawn(t) // use tokio ?
        }

        let t = {
            let jq = out.job_queue.clone();
            async move {
                loop {
                    // async sleep 50ms
                    let n = jq.len_plus_inflight();
                    if (n < num_tasks) {
                        // we track how many are in flight
                        jq.req_n(num_tasks - n);
                    }
                }
            }
        };
        // spawn(t) // use tokio ?

        out
    }

    pub async fn fetch_url(url: Url) -> Result<Document, anyhow::Error> {
        // untested code; copy pasted from phind.com
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5)) // Set the timeout to 5 seconds
            .build()
            .unwrap();

        let text = client.get(&url.target_url).send().await?.text().await?;

        // should we be using selenium ?

        let links = select::document::Document::from(text.as_str())
            .find(select::predicate::Name("a"))
            .filter_map(|n| n.attr("href").map(|x| x.to_string()))
            .collect::<Vec<_>>();

        Ok(Document { url, text, links })
    }
}
