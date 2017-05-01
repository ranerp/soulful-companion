pub mod config;

use std::error::Error;
use std::fmt;

use serde_yaml::Error as SerdeError;

use io::IoError;

#[derive(Debug)]
pub enum ConfigError {
    Serialize(SerdeError),
    Deserialize(SerdeError),
    Save(IoError),
    Load(IoError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::Serialize(ref err) => write!(f, "Serialize error: {}", err),
            ConfigError::Deserialize(ref err) => write!(f, "Deserialize error: {}", err),
            ConfigError::Save(ref err) => write!(f, "Save error: {}", err),
            ConfigError::Load(ref err) => write!(f, "Load error: {}", err),
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::Serialize(ref err) => err.description(),
            ConfigError::Deserialize(ref err) => err.description(),
            ConfigError::Save(ref err) => err.description(),
            ConfigError::Load(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConfigError::Serialize(ref err) => Some(err),
            ConfigError::Deserialize(ref err) => Some(err),
            ConfigError::Save(ref err) => Some(err),
            ConfigError::Load(ref err) => Some(err),
        }
    }
}