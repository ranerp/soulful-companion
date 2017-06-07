use std::collections::BinaryHeap;
use std::cmp::{PartialEq, PartialOrd, Eq, Ord, Ordering};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender};
use std::time::Duration;

use uuid::Uuid;
use chrono::prelude::*;
use chrono::DateTime;

#[derive(Debug)]
enum CommandType {
    SCHEDULE,
    CANCEL,
}

#[derive(Clone)]
pub struct ThreadSafeCallback {
    cb: Arc<Mutex<Fn() + Send + 'static>>
}

impl ThreadSafeCallback {
    pub fn new<F>(cb: F) -> ThreadSafeCallback
        where F: Fn() + Send + 'static {
        ThreadSafeCallback {
            cb: Arc::new(Mutex::new(cb)),
        }
    }

    fn call(&self) {
        let cb = self.cb.clone();
        let cb = cb.lock().unwrap();
        (cb)()
    }
}

#[derive(Clone)]
pub struct Job {
    pub id: Uuid,
    pub cb: ThreadSafeCallback,
    pub time: DateTime<UTC>,
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Job) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Job) -> bool {
        self.id == other.id
    }
}

impl Eq for Job {}

struct Message {
    pub command: CommandType,
    pub job: Job,
}

struct Manager {
    jobs: BinaryHeap<Job>,
}

impl Manager {

    pub fn new() -> Manager {
        Manager {
            jobs: BinaryHeap::new(),
        }
    }

    fn schedule(&mut self, job: Job) {
        self.jobs.push(job);
    }

    fn cancel(&mut self, cancel_job: Job) {
        let mut tmp = BinaryHeap::new();

        while let Some(job) = self.jobs.pop() {
            if job == cancel_job {
                break;
            }

            tmp.push(job);
        }

        self.jobs.append(&mut tmp);
    }
}

pub struct Scheduler<T> {
    tx: Sender<T>,
}

impl Scheduler<Message> {
    pub fn new() -> Scheduler<Message> {
        Scheduler {
            tx: Scheduler::start(),
        }
    }

    pub fn schedule(&self, job: Job) {
        self.tx.send(Message {
            command: CommandType::SCHEDULE,
            job: job,
        }).unwrap();
    }

    pub fn cancel(&self, job: Job) {
        self.tx.send(Message {
            command: CommandType::CANCEL,
            job: job,
        }).unwrap();
    }

    fn start() -> Sender<Message> {
        let (tx, rx) = mpsc::channel::<Message>();

        thread::spawn(move || {
            let mut manager = Manager::new();

            loop {
                match rx.try_recv() {
                Ok(msg) => {
                        match msg.command {
                            CommandType::SCHEDULE => manager.schedule(msg.job),
                            CommandType::CANCEL => manager.cancel(msg.job),
                        }
                    }
                    Err(_) => (),
                }

                loop {
                    let cmp = match manager.jobs.peek() {
                        Some(job) => job.time.cmp(&UTC::now()),
                        None => break,
                    };

                    if cmp == Ordering::Greater {
                        break;
                    }

                    match manager.jobs.pop() {
                        Some(job) => { job.cb.call(); },
                        None => panic!("Jobs heap should not be empty at this point"),
                    }
                }

                thread::sleep(Duration::from_millis(1));
            }
        });

        tx
    }
}