extern crate chrono;
extern crate soulful_companion;
extern crate uuid;

use soulful_companion::config::config;
use soulful_companion::schedule::Scheduler;
use soulful_companion::schedule::Job;
use soulful_companion::schedule::ThreadSafeCallback;
use soulful_companion::led::ColorModifier;
use soulful_companion::led::Controller;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use uuid::Uuid;
use chrono::prelude::*;
use chrono::Duration as CDuration;

fn main() {
    let mut config: config::Config = config::load();
    let scheduler = Scheduler::new();

    let start_time = UTC::now();
    let end_time = start_time.checked_add_signed(CDuration::seconds(config.activity_duration_sec() as i64)).unwrap();
    let color_modifier = Arc::new(Mutex::new(ColorModifier::new(config.color().start().clone(), config.color().end().clone(), start_time, end_time)));

    let mut controller = Controller::new();
    controller.set_pwm_freq(60_f64);

    let controller = Arc::new(Mutex::new(controller));

    let job = Job::new_periodic(
        Uuid::new_v4(),
        ThreadSafeCallback::new(move || {
            let color_modifier = color_modifier.clone();
            let mut color_modifier = color_modifier.lock().unwrap();

            let controller = controller.clone();
            let mut controller = controller.lock().unwrap();
            color_modifier.interp_by_time_elapsed();

            controller.set_pwm(0, color_modifier.at_color.b as u8, 0);

            println!("{:?}", UTC::now());
        }),
        start_time,
        end_time,
        config.timer().update_frequency_ms() as u64);

    scheduler.schedule_periodic(job);

    thread::sleep(Duration::from_secs(10));
}
