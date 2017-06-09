use std::cmp::{Ordering};

use chrono::DateTime;
use chrono::prelude::*;

use color::Rgb;
use color::rgb_to_hsl;
use color::hsl_to_rgb;

pub struct ColorModifier {
    start_color: Rgb,
    end_color: Rgb,
    start_time: DateTime<UTC>,
    end_time: DateTime<UTC>,
}

impl ColorModifier {
    pub fn new(start_color: Rgb, end_color: Rgb, start_time: DateTime<UTC>, end_time: DateTime<UTC>) -> ColorModifier {
        ColorModifier {
            start_color: start_color,
            end_color: end_color,
            start_time: start_time,
            end_time: end_time,
        }
    }

    pub fn interp_by_time_elapsed(&self) {
        let now = UTC::now();
        if now.cmp(&self.end_time) == Ordering::Greater {
            return;
        }

        let all_timeline = self.end_time.timestamp() - self.start_time.timestamp();
        let at_timeline = self.end_time.timestamp() - now.timestamp();

        let coefficient = (at_timeline / all_timeline) as f32;

        println!("coefficeint {}", coefficient);

        let hsl_start = rgb_to_hsl(self.start_color.clone());
        let hsl_end = rgb_to_hsl(self.end_color.clone());
        let hsl_interp = (&hsl_end - &hsl_start) * coefficient;
        let rgb = hsl_to_rgb(&hsl_start + &hsl_interp);
        println!("{:?}", rgb);
    }
}
