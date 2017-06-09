extern crate chrono;
extern crate soulful_companion;
extern crate uuid;

use soulful_companion::color::Rgb;
use soulful_companion::config::config;
use soulful_companion::schedule::Scheduler;
use soulful_companion::schedule::Job;
use soulful_companion::schedule::ThreadSafeCallback;
use soulful_companion::led::ColorModifier;

use std::time::Instant;
use std::thread;
use std::time::Duration;
use std::cmp;

use uuid::Uuid;
use chrono::prelude::*;
use chrono::Duration as CDuration;

fn main() {
    let mut config: config::Config = config::load();
    let scheduler = Scheduler::new();

    let start = UTC::now();

    let finish_from_now = (config.timer.run_duration_min as f32 * config.timer.start_activity_percent * 60.0) as i64;
    let finish = UTC::now().checked_add_signed(CDuration::seconds(finish_from_now)).unwrap();

    let color_modifier = ColorModifier::new(config.color.start, config.color.end, start, finish);

    let job = Job::new_periodic(
        Uuid::new_v4(),
        ThreadSafeCallback::new(move || {
            let color_modifier = color_modifier;
            color_modifier.interp_by_time_elapsed();
            println!("{:?}", UTC::now());
        }),
        UTC::now(),
        finish,
        (config.timer.update_frequency_sec * 1_000) as u64);

    scheduler.schedule_periodic(job);

    thread::sleep(Duration::from_secs(15));
}
