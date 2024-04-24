use super::*;

use std::sync::{Arc, Mutex};
// use std::time::Duration;

// ==== Interface

// ==== Impl

#[derive(Clone)]
pub struct UrlQueue {
    job_queue: Arc<Mutex<VecDeque<Url>>>,
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
        }
    }

    pub fn len(&self) -> usize {
        self.job_queue.lock().unwrap().len()
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
                    let n = jq.len();
                    if (n < num_tasks) {
                        // how do we handle this ?
                        // what if network delay ?
                        // exponential backoff ?
                        // if we request  (num_tasks - n)  on k ticks,
                        // do we suddenly get result of k * (num_tasks -n) elements ?
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
