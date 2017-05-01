pub mod config;

extern crate serde_yaml;

use std::error::Error;
use std::fmt;

use self::serde_yaml::Error as SerdeError;

use io::IoError;

#[derive(Debug)]
pub enum ConfigError {
    Deserialize(SerdeError),
    Load(IoError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::Deserialize(ref err) => write!(f, "Deserialize error: {}", err),
            ConfigError::Load(ref err) => write!(f, "Load error: {}", err),
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::Deserialize(ref err) => err.description(),
            ConfigError::Load(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConfigError::Deserialize(ref err) => Some(err),
            ConfigError::Load(ref err) => Some(err),
        }
    }
}