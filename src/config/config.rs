extern crate serde_yaml;

use io::io;
use config::ConfigError;
use color::rgb::Rgb;

const CONF_FILE_PATH: &'static str = "resources/conf.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TimerConfig {
    pub run_duration: u32,
    pub start_activity_percent: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorConfig {
    pub start: Rgb,
    pub end: Rgb,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub timer: TimerConfig,
    pub color: ColorConfig,
}

impl Config {
    fn default() -> Config {
        Config {
            timer: TimerConfig {
                run_duration: 60,
                start_activity_percent: 0.05
            },
            color: ColorConfig {
                start: Rgb::new(255, 255, 255),
                end: Rgb::new(0, 0, 0),
            }
        }
    }
}

pub fn load() -> Config {
    match load_yaml_config(CONF_FILE_PATH) {
        Ok(config) => return config,
        Err(err) => {
            println!("{}.\nLoading default configuration.", err);
            return Config::default()
        },
    }
}

fn load_yaml_config(file_path: &str) -> Result<Config, ConfigError> {
    match io::load_file_to_str(file_path) {
        Ok(ref str) => return serde_yaml::from_str::<Config>(str).map_err(ConfigError::Deserialize),
        Err(err) => return Err(ConfigError::Load(err)),
    }
}