extern crate timer;
extern crate chrono;

extern crate soulful_companion;

use soulful_companion::color::color_converter;
use soulful_companion::color::rgb::Rgb;

//use std::sync::mpsc;

fn main() {

    let hsl = color_converter::rgb_to_hsl(Rgb::new(255, 0, 0));
    let hsl2 = color_converter::rgb_to_hsl(Rgb::new(0, 255, 0));

    let i = 0.5;

    let hsl3 = (&hsl2 - &hsl) * 0.3;

    let rgb = color_converter::hsl_to_rgb(&hsl + &hsl3);

    println!("{} {} {}", hsl.h, hsl.s, hsl.l);
    println!("{} {} {}", hsl2.h, hsl2.s, hsl2.l);
    println!("{} {} {}", hsl3.h, hsl3.s, hsl3.l);
    println!("{}, {}, {}", rgb.r, rgb.g, rgb.b);

    //let timer = timer::Timer::new();
    //let (tx, rx) = mpsc::channel();
    //
    //let _guard = timer.schedule_with_delay(chrono::Duration::minutes(3), move || {
    //let _ignored = tx.send(());
    //});
    //
    //rx.recv().unwrap();
    //println!("This code has been executed after 3 seconds");

}
