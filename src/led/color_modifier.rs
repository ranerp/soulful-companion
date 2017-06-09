use std::cmp;

use chrono::DateTime;
use chrono::prelude::*;

use color::Rgb;
use color::rgb_to_hsl;
use color::hsl_to_rgb;

pub struct ColorModifier {
    start_color: Rgb,
    end_color: Rgb,
    at_color: Rgb,
    start_time: DateTime<UTC>,
    end_time: DateTime<UTC>,
}

impl ColorModifier {
    pub fn new(start_color: Rgb, end_color: Rgb, start_time: DateTime<UTC>, end_time: DateTime<UTC>) -> ColorModifier {
        ColorModifier {
            start_color: start_color.clone(),
            end_color: end_color,
            at_color: start_color,
            start_time: start_time,
            end_time: end_time,
        }
    }

    pub fn interp_by_time_elapsed(&mut self) {
        if self.at_color == self.end_color {
            return;
        }

        let now = UTC::now();

        let timeline = self.end_time.timestamp() - self.start_time.timestamp();
        let at = cmp::min(now.timestamp() - self.start_time.timestamp(), timeline);

        let coefficient = at as f32 / timeline as f32;

        let start = rgb_to_hsl(self.start_color.clone());
        let end = rgb_to_hsl(self.end_color.clone());
        let lerp = (&end - &start) * coefficient;
        
        let rgb = hsl_to_rgb(&start + &lerp);
        
        println!("{:?}", rgb);

        self.at_color = rgb;
    }
}
