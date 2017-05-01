use timer::Timer;
use timer::Guard;
use chrono::Duration;
use std::thread;

pub struct ActionTimer {
    timer: Timer,
    guard: Guard,
}

impl ActionTimer {
    pub fn new(timer: Timer, guard: Guard) -> ActionTimer {
        ActionTimer {
            timer: timer,
            guard: guard,
        }
    }

    pub fn schedule() {
        let timer = Timer::new();
        let guard = timer.schedule_repeating(Duration::seconds(2), print);

        thread::sleep(::std::time::Duration::new(10, 0));

        drop(guard);
    }
}

fn print() {
    println!("{}", "action!");
}

