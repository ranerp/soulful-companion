use std::f32;
use std::u8;

use color::hsl::Hsl;
use color::rgb::Rgb;

/// Code implemented from: http://stackoverflow.com/questions/2353211/hsl-to-rgb-color-conversion
///
/// Converts an HSL color value to RGB. Conversion formula
/// adapted from http://en.wikipedia.org/wiki/HSL_color_space.
/// Assumes h, s, and l are contained in the set [0, 1] and
/// returns r, g, and b in the set [0, 255].
///
pub fn hsl_to_rgb(hsl: Hsl) -> Rgb {
    let h = hsl.h;
    let s = hsl.s;
    let l = hsl.l;

    if s == 0.0 {
        let c = l * u8::MAX as f32;
        let c_u8 = c.round() as u8;
        return Rgb::new(c_u8, c_u8, c_u8);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };

    let p = 2.0 * l - q;

    let r = hue_to_rgb(p, q, h + 1.0/3.0) * u8::MAX as f32;
    let g = hue_to_rgb(p, q, h) * u8::MAX as f32;
    let b = hue_to_rgb(p, q, h - 1.0/3.0) * u8::MAX as f32;

    Rgb::new(r.round() as u8, g.round() as u8, b.round() as u8)
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let t = match t {
        n if n < 0.0 => t + 1.0,
        n if n > 1.0 => t - 1.0,
        _ => t,
    };

    match t {
        n if n < 1.0 / 6.0 => return p + (q - p) * 6.0 * t,
        n if n < 1.0 / 2.0 => return q,
        n if n < 2.0 / 3.0 => return p + (q - p) * (2.0 / 3.0 - t) * 6.0,
        _ => return p
    }
}

/// Code implemented from: http://stackoverflow.com/questions/2353211/hsl-to-rgb-color-conversion
///
/// Converts an RGB color value to HSL. Conversion formula
/// adapted from http://en.wikipedia.org/wiki/HSL_color_space.
/// Assumes r, g, and b are contained in the set [0, 255] and
/// returns h, s, and l in the set [0, 1].
///
pub fn rgb_to_hsl(rgb: Rgb) -> Hsl {
    let r = rgb.r as f32 / u8::MAX as f32;
    let g = rgb.g as f32 / u8::MAX as f32;
    let b = rgb.b as f32 / u8::MAX as f32;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);

    let mid = || (max + min) / 2.0;

    let l = mid();

    if max == min {
        return Hsl::new(0.0, 0.0, l);
    }

    let d = max - min;

    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let g_add = if g < b {
        6.0
    } else {
        0.0
    };
    let h = match max {
        n if (n == r) => (g - b) / d + g_add,
        n if (n == g) => (b - r) / d + 2.0,
        n if (n == b) => (r - g) / d + 4.0,
        _ => panic!("Max must always be red, green or blue channel value.")
    };

    let h = h / 6.0;

    Hsl::new(h, s, l)
}