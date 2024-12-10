pub mod point;
pub mod direction;
pub mod grid;

use std::{fs, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputFileError {
    #[error("Input file with path '{0}' does not exist.")]
    InputDoesNotExists(String),
    #[error("File system operation failed: {0}")]
    FileSystemOperationFailed(#[from] io::Error),
}

pub fn read_input_file(file_path: &str) -> Result<String, InputFileError> {
    if !fs::exists(file_path)? {
        return Err(InputFileError::InputDoesNotExists(file_path.to_owned()));
    }

    Ok(fs::read_to_string(file_path)?)
}
