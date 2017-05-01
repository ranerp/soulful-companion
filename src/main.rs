extern crate soulful_companion;

use soulful_companion::color::color_converter;
use soulful_companion::color::rgb::Rgb;
use soulful_companion::config::config;
use soulful_companion::time::action_timer::ActionTimer;

fn main() {
    let mut config: config::Config = config::load();

    let hsl = color_converter::rgb_to_hsl(Rgb::new(255, 0, 0));
    let hsl2 = color_converter::rgb_to_hsl(Rgb::new(0, 255, 0));

    let i = 0.5;

    let hsl3 = (&hsl2 - &hsl) * i;

    let rgb = color_converter::hsl_to_rgb(&hsl + &hsl3);

    println!("{:?}", hsl);
    println!("{:?}", hsl3);
    println!("{:?}", rgb);

    ActionTimer::schedule();
}
