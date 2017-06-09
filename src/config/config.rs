use io;
use config::ConfigError;
use color::Rgb;
use serde_yaml;

const CONF_FILE_PATH: &'static str = "resources/conf.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TimerConfig {
    run_duration_min: u32,
    update_frequency_sec: u32,
    start_activity_percent: f32,
}

impl TimerConfig {
    pub fn run_duration_min(&mut self) -> u32 { self.run_duration_min }
    pub fn update_frequency_sec(&mut self) -> u32 { self.update_frequency_sec }
    pub fn update_frequency_ms(&mut self) -> f32 { self.update_frequency_sec() as f32 * 1_000.0 }
    pub fn start_activity_percent(&mut self) -> f32 { self.start_activity_percent }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ColorConfig {
    start: Rgb,
    end: Rgb,
}

impl ColorConfig {
    pub fn start(&mut self) -> &mut Rgb { &mut self.start }
    pub fn end(&mut self) -> &mut Rgb { &mut self.end }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    timer: TimerConfig,
    color: ColorConfig,
}

impl Config {
    pub fn timer(&mut self) -> &mut TimerConfig { &mut self.timer }
    pub fn color(&mut self) -> &mut ColorConfig { &mut self.color }
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

    pub fn activity_duration_sec(&mut self) -> f32 {
        self.timer.run_duration_min as f32 * self.timer.start_activity_percent * 60.0
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