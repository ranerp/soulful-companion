extern crate chrono;
extern crate soulful_companion;
extern crate uuid;

use soulful_companion::color::rgb_to_hsl;
use soulful_companion::color::hsl_to_rgb;
use soulful_companion::color::Rgb;
use soulful_companion::config::config;
use soulful_companion::schedule::Scheduler;
use soulful_companion::schedule::Job;
use soulful_companion::schedule::ThreadSafeCallback;

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

    let timeline_sec = finish.timestamp() - start.timestamp();

    let update_freq_sec = config.timer.update_frequency_sec;

    let job = Job::new_periodic(
        Uuid::new_v4(),
        ThreadSafeCallback::new(move || {
            let hsl = rgb_to_hsl(Rgb::new(255, 0, 0));
            let hsl2 = rgb_to_hsl(Rgb::new(0, 255, 0));

            let start = start;

            let i = 0.5;

            let hsl3 = (&hsl2 - &hsl) * i;

            let rgb = hsl_to_rgb(&hsl + &hsl3);

            println!("{:?}", UTC::now());
            println!("{:?}", rgb);
        }),
        UTC::now(),
        finish,
        (config.timer.update_frequency_sec * 1_000) as u64);

    scheduler.schedule_periodic(job);

    thread::sleep(Duration::from_secs(15));
}
