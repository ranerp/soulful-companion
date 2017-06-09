pub mod hsl;
pub use self::hsl::Hsl;

pub mod rgb;
pub use self::rgb::Rgb;


pub mod color_converter;
pub use self::color_converter::hsl_to_rgb;
pub use self::color_converter::rgb_to_hsl;

