extern crate chrono;
extern crate soulful_companion;
extern crate uuid;
//use soulful_companion::color::color_converter;
//use soulful_companion::color::rgb::Rgb;
//use soulful_companion::config::config;
use soulful_companion::schedule::scheduler::Scheduler;
use soulful_companion::schedule::scheduler::Job;
use soulful_companion::schedule::scheduler::ThreadSafeCallback;

use uuid::Uuid;
use chrono::prelude::*;

use std::thread;
use std::time::Duration;

fn main() {
    //let mut config: config::Config = config::load();

    //let hsl = color_converter::rgb_to_hsl(Rgb::new(255, 0, 0));
    //let hsl2 = color_converter::rgb_to_hsl(Rgb::new(0, 255, 0));

    //let i = 0.5;

    //let hsl3 = (&hsl2 - &hsl) * i;

    //let rgb = color_converter::hsl_to_rgb(&hsl + &hsl3);

    //println!("{:?}", hsl);
    //println!("{:?}", hsl3);
    //println!("{:?}", rgb);

    let scheduler = Scheduler::new();

    let job = Job {
        id: Uuid::new_v4(),
        cb: ThreadSafeCallback::new(|| {
            println!("job1");
        }),
        time: UTC::now(),
    };

    thread::sleep(Duration::from_secs(1));

    let job2 = Job {
        id: Uuid::new_v4(),
        cb: ThreadSafeCallback::new(|| {
            println!("job2");
        }),
        time: UTC::now(),
    };

    thread::sleep(Duration::from_secs(1));

    let job3 = Job {
        id: Uuid::new_v4(),
        cb: ThreadSafeCallback::new(|| {
            println!("job3");
        }),
        time: UTC::now(),
    };

    scheduler.schedule(job.clone());
    scheduler.schedule(job2.clone());
    scheduler.schedule(job3.clone());

    println!("{:?}", job.id);

    thread::sleep(Duration::from_secs(5));
}
