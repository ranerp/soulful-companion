use io::io;
use config::ConfigError;
use color::rgb::Rgb;
use serde_yaml;

const CONF_FILE_PATH: &'static str = "resources/conf.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TimerConfig {
    pub run_duration_min: u32,
    pub update_frequency_sec: u32,
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
                run_duration_min: 60,
                update_frequency_sec: 10,
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

pub fn save(config: &Config) -> Result<String, ConfigError> {
    match serde_yaml::to_string(&config) {
        Ok(ref str) => return io::save_str_to_file(str, CONF_FILE_PATH).map_err(ConfigError::Save),
        Err(err) => return Err(ConfigError::Serialize(err)),
    }
}

fn load_yaml_config(file_path: &str) -> Result<Config, ConfigError> {
    match io::load_file_to_str(file_path) {
        Ok(ref str) => return serde_yaml::from_str::<Config>(str).map_err(ConfigError::Deserialize),
        Err(err) => return Err(ConfigError::Load(err)),
    }
}