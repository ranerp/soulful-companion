extern crate chrono;
extern crate soulful_companion;
extern crate uuid;

use soulful_companion::color::color_converter;
use soulful_companion::color::rgb::Rgb;
use soulful_companion::config::config;
use soulful_companion::schedule::scheduler::Scheduler;
use soulful_companion::schedule::scheduler::Job;
use soulful_companion::schedule::scheduler::ThreadSafeCallback;

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
    let update_freq_sec = config.timer.update_frequency_sec;

    let job = Job::new_periodic(
        Uuid::new_v4(),
        ThreadSafeCallback::new(|| {
            let hsl = color_converter::rgb_to_hsl(Rgb::new(255, 0, 0));
            let hsl2 = color_converter::rgb_to_hsl(Rgb::new(0, 255, 0));

            let start = start;
            let finish = UTC::now().checked_add_signed(CDuration::seconds(update_freq_sec));

            let i = 0.5;

            let hsl3 = (&hsl2 - &hsl) * i;

            let rgb = color_converter::hsl_to_rgb(&hsl + &hsl3);

            println!("{:?}", hsl);
            println!("{:?}", hsl3);
            println!("{:?}", rgb);
        }),
        UTC::now(),
        (config.timer.update_frequency_sec * 1_000) as u64);

    scheduler.schedule_periodic(job);

    thread::sleep(Duration::from_secs(15));
}
