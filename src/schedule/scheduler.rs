use std::collections::BinaryHeap;
use std::cmp::{PartialEq, PartialOrd, Eq, Ord, Ordering};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender};
use std::time::Duration;
use std::hash::{Hash, Hasher};

use uuid::Uuid;
use chrono::prelude::*;
use chrono::DateTime;
use chrono::Duration as CDuration;

#[derive(Debug)]
pub enum CommandType {
    SchedulePeriodic,
    ScheduleOnce,
    Cancel,
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

    fn call(&mut self) {
        let cb = self.cb.clone();
        let cb = cb.lock().unwrap();
        cb();
    }
}

#[derive(Clone)]
pub struct Job {
    id: Uuid,
    cb: ThreadSafeCallback,
    time: DateTime<UTC>,
    end: Option<DateTime<UTC>>,
    period_ms: Option<u64>,
}

impl Job {
    pub fn new_once(id: Uuid, cb: ThreadSafeCallback, time: DateTime<UTC>) -> Job {
        Job {
            id: id,
            cb: cb,
            time: time,
            end: None,
            period_ms: None,
        }
    }

    pub fn new_periodic(id: Uuid, cb: ThreadSafeCallback, first_time: DateTime<UTC>, finish_time: DateTime<UTC>, period_ms: u64) -> Job {
        Job {
            id: id,
            cb: cb,
            time: first_time,
            end: Some(finish_time),
            period_ms: Some(period_ms),
        }
    }
}

impl Hash for Job {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.time.hash(state);
        self.period_ms.hash(state);
    }
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

pub struct Message {
    pub command: CommandType,
    pub job: Job,
}

struct Manager {
    one_time_jobs: BinaryHeap<Job>,
    periodic_jobs: Vec<Job>,
}

impl Manager {

    pub fn new() -> Manager {
        Manager {
            one_time_jobs: BinaryHeap::new(),
            periodic_jobs: Vec::new(),
        }
    }

    fn schedule_once(&mut self, job: Job) {
        self.one_time_jobs.push(job);
    }

    fn schedule_periodic(&mut self, job: Job) {
        self.periodic_jobs.push(job);
    }

    fn cancel(&mut self, job: Job) {
        if self.periodic_jobs.contains(&job) {
            self.cancel_periodic_job(job);
        } else {
            self.cancel_one_time_job(job);
        }
    }

    // TODO Replace with better data structure!
    fn cancel_periodic_job(&mut self, job: Job) {
        let mut index: Option<usize> = None;
        for (i, look) in self.periodic_jobs.iter().enumerate() {
            if *look == job {
                index = Some(i);
                break;
            }
        }

        match index {
            Some(i) => { self.periodic_jobs.remove(i); },
            None => println!("Could not remove job"),
        }
    }

    fn cancel_one_time_job(&mut self, job: Job) {
        let mut tmp = BinaryHeap::new();

        while let Some(j) = self.one_time_jobs.pop() {
            if j == job {
                break;
            }

            tmp.push(j);
        }

        self.one_time_jobs.append(&mut tmp);
    }

    fn cancel_over_period_jobs(&mut self) {
        let mut rm_idx = Vec::new();

        for (i, job) in self.periodic_jobs.iter().enumerate() {
            match job.end {
                Some(end) => {
                    if UTC::now().cmp(&end) == Ordering::Greater {
                        rm_idx.push(i);
                    }
                }
                None => (),
            }
        }

        for i in rm_idx.iter() {
            self.periodic_jobs.remove(*i);
        }
    }

    fn call_periodic_jobs(&mut self) {
        for mut job in self.periodic_jobs.iter_mut() {
            if job.time.cmp(&UTC::now()) == Ordering::Greater {
                continue;
            }

            job.cb.call();
            job.time = UTC::now().checked_add_signed(CDuration::milliseconds(job.period_ms.unwrap() as i64)).unwrap();
        }
    }

    fn call_one_time_jobs(&mut self) {
        loop {
            let cmp = match self.one_time_jobs.peek() {
                Some(job) => job.time.cmp(&UTC::now()),
                None => break,
            };

            if cmp == Ordering::Greater {
                break;
            }

            match self.one_time_jobs.pop() {
                Some(mut job) => { job.cb.call(); },
                None => panic!("Jobs heap should not be empty at this point"),
            }
        }
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

    pub fn schedule_once(&self, job: Job) {
        self.tx.send(Message {
            command: CommandType::ScheduleOnce,
            job: job,
        }).unwrap();
    }

    pub fn schedule_periodic(&self, job: Job) {
        self.tx.send(Message {
            command: CommandType::SchedulePeriodic,
            job: job,
        }).unwrap();
    }

    pub fn cancel(&self, job: Job) {
        self.tx.send(Message {
            command: CommandType::Cancel,
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
                            CommandType::ScheduleOnce => manager.schedule_once(msg.job),
                            CommandType::SchedulePeriodic => manager.schedule_periodic(msg.job),
                            CommandType::Cancel => manager.cancel(msg.job),
                        }
                    }
                    Err(_) => (),
                }

                manager.cancel_over_period_jobs();

                manager.call_periodic_jobs();
                manager.call_one_time_jobs();

                thread::sleep(Duration::from_millis(1));
            }
        });

        tx
    }
}