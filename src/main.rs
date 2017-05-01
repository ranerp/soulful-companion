extern crate soulful_companion;
extern crate timer;
extern crate chrono;

use soulful_companion::color::color_converter;
use soulful_companion::color::rgb::Rgb;
use soulful_companion::config::config::Config;

fn main() {

    let hsl = color_converter::rgb_to_hsl(Rgb::new(255, 0, 0));
    let hsl2 = color_converter::rgb_to_hsl(Rgb::new(0, 255, 0));

    let i = 0.5;

    let hsl3 = (&hsl2 - &hsl) * i;

    let rgb = color_converter::hsl_to_rgb(&hsl + &hsl3);

    println!("{:?}", hsl);
    println!("{:?}", hsl3);
    println!("{:?}", rgb);

    let config: Config = Config::new();

    println!("{:?}", config.color.start);
    println!("{:?}", config.color.end);
}
