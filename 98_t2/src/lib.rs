use std::collections::*;
use std::sync::*;
use tokio::sync::Notify;

async fn fetch(url: &str) -> String {
    todo!("");
}

fn extract_links(content: &str) -> Vec<String> {
    todo!("");
}

pub struct JobQueueInner {
    jobs: Vec<String>,
    already_pushed: HashSet<String>,
}

#[derive(Clone)]
pub struct JobQueue {
    inner: Arc<Mutex<JobQueueInner>>,
}

impl JobQueue {
    // this one abstraction prevents the last 3 design mistakes I made
    pub fn push(&self, urls: Vec<String>) {
        let mut t = self.inner.lock().unwrap();
        for url in urls {
            if !t.already_pushed.contains(&url) {
                t.already_pushed.insert(url.clone());
                t.jobs.push(url);
            }
        }
    }

    pub fn pop(&self) -> Option<String> {
        let mut t = self.inner.lock().unwrap();
        t.jobs.pop()
    }

    pub fn new(init: Vec<String>) -> JobQueue {
        JobQueue {
            inner: Arc::new(Mutex::new(JobQueueInner {
                jobs: init.clone(),
                already_pushed: init.into_iter().collect::<_>(),
            })),
        }
    }
}

#[derive(Clone)]
pub struct JobController {
    cnt: Arc<Mutex<usize>>,
    notify: Arc<Notify>,
    max_concurrent: usize,
}

impl JobController {
    pub fn new(max_concurrent: usize) -> JobController {
        JobController {
            cnt: Arc::new(Mutex::new(0)),
            notify: Arc::new(Notify::new()),
            max_concurrent,
        }
    }

    fn inc(&self) {
        let mut cnt = self.cnt.lock().unwrap();
        *cnt = *cnt + 1;
    }

    fn dec(&self) {
        let mut cnt = self.cnt.lock().unwrap();
        *cnt = *cnt - 1;
        self.notify.notify_one();
    }

    pub fn get(&self) -> usize {
        let mut cnt = self.cnt.lock().unwrap();
        *cnt
    }

    async fn wait_for_dec(&self) {
        self.notify.notified().await;
    }

    async fn fire_job<F: std::future::Future<Output = ()> + Send + Sync + 'static>(&self, f: F) {
        while self.get() > self.max_concurrent {
            self.notify.notified().await;
        }

        self.inc();

        tokio::spawn({
            let s = self.clone();
            async move {
                f.await;
                s.dec();
            }
        });
    }
}

fn main() {
    let jq: JobQueue = JobQueue::new(vec!["a.com".to_string(), "b.com".to_string()]);
    let job_controller: JobController = JobController::new(1000);

    let outer = async move {
        loop {
            match jq.pop() {
                None => {
                    if job_controller.get() == 0 {
                        return;
                    } else {
                        job_controller.wait_for_dec().await;
                    }
                }
                Some(url) => {
                    let task = {
                        let jq = jq.clone();
                        let in_flight = job_controller.clone();
                        async move {
                            let content = fetch(&url).await;
                            let links = extract_links(&content);
                            jq.push(links);
                        }
                    };

                    job_controller.fire_job(task);
                }
            }
        }
    };

    // tokio block_on(outer);
}
