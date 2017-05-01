pub mod io;

use std::io as std_io;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum IoError {
    Io(std_io::Error),
    Create(std_io::Error),
    Open(std_io::Error),
    ReadToString(std_io::Error),
    Write(std_io::Error),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IoError::Io(ref err) => write!(f, "Io error: {}", err),
            IoError::Create(ref err) => write!(f, "Create error: {}", err),
            IoError::Open(ref err) => write!(f, "Open error: {}", err),
            IoError::ReadToString(ref err) => write!(f, "Read to string error: {}", err),
            IoError::Write(ref err) => write!(f, "Write to file error: {}", err),
        }
    }
}

impl error::Error for IoError {
    fn description(&self) -> &str {
        match *self {
            IoError::Io(ref err) => err.description(),
            IoError::Create(ref err) => err.description(),
            IoError::Open(ref err) => err.description(),
            IoError::ReadToString(ref err) => err.description(),
            IoError::Write(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            IoError::Io(ref err) => Some(err),
            IoError::Create(ref err) => Some(err),
            IoError::Open(ref err) => Some(err),
            IoError::ReadToString(ref err) => Some(err),
            IoError::Write(ref err) => Some(err),
        }
    }
}

impl From<std_io::Error> for IoError {
    fn from(err: std_io::Error) -> IoError {
        IoError::Io(err)
    }
}