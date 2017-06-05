use std::collections::BinaryHeap;
use std::cmp::{PartialEq, PartialOrd, Eq, Ord, Ordering};
use std::thread;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use chrono::prelude::*;

use chrono::DateTime;

#[derive(Debug)]
enum CommandType {
    SCHEDULE,
    CANCEL,
}

struct Job {
    pub id: Uuid,
    pub cb: Box<FnOnce() + Send>,
    pub time: DateTime<UTC>,
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Job) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Job) -> bool {
        self.id == other.id
    }
}

impl Eq for Job {}

struct Message {
    command: CommandType,
    job: Job,
}

pub struct Scheduler<T> {
    tx: Option<Sender<T>>,
    jobs: Arc<Mutex<BinaryHeap<Job>>>,
}

impl Scheduler<Message> {
    pub fn new() -> Scheduler<Message> {
        Scheduler {
            tx: None,
            jobs: Arc::new(Mutex::new(BinaryHeap::new())),
        }
    }

    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel();
        let jobs = self.jobs.clone();
        self.tx = Some(tx);

        thread::spawn(move || {
            loop {
                let mut jobs = jobs.lock().unwrap();

                match rx.try_recv() {
                Ok(msg) => {
                        jobs.append(msg.job);
                    }
                    Err(_) => (),
                }

                let cmp = match jobs.peek() {
                    Some(job) => job.time.cmp(&UTC::now()),
                    None => continue,
                };

                if cmp == Ordering::Equal || cmp == Ordering::Less {
                    println!("executing job");

                    match jobs.pop() {
                        None => println!("Could not call job"),
                        Some(job) => { job.cb; },
                    }
                }
            }
        });
    }

    pub fn schedule(&mut self, job: Job) {
        match self.tx {
            None => println!("Sender not initialized"),
            Some(ref tx) => {
                tx.send(Message {
                        command: CommandType::SCHEDULE,
                        job: job,
                }).unwrap();
            },
        }
    }
}