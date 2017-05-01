use std::fs::File;
use std::io::Read;

use io::IoError;

pub fn load_file_to_str(file_path: &str) -> Result<String, IoError> {
    let mut file = File::open(file_path).map_err(IoError::Open)?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(IoError::ReadToString)?;
    Ok(content)
}