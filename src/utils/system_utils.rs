use crate::error::{AppError, ErrorType, Result};
use chrono::Local;
use std::path::PathBuf;
use std::{env, fs};

/// Struct for SystemUtils, Contains the common utilities abstracting time, file etc.
///
/// # Author
/// K Vishwanath
pub struct SystemUtils;

/// Implementation of SystemUtils Struct
///
/// # Author
/// K Vishwanath
impl SystemUtils {
    /// 100 MB
    const LARGE_FILE_SIZE: u64 = 104857600;

    /// Returns a timestamp in the format <strong>yyyy-MM-dd_HH:mm:ss,SSS</strong>
    pub fn get_timestamp() -> String {
        Local::now().format("%Y-%m-%d_%H:%M:%S,%3f").to_string()
    }

    /// It is assumed that the passed path is a file.
    /// If using RustRover or any IDE where the code snippet fs::metadata(path) highlights path as red you can ignore this.
    pub fn is_file_large(path: &PathBuf) -> Result<bool> {
        let metadata = fs::metadata(path).map_err(|_| {
            let message = format!(
                "SystemError: Unable to get metadata of file, in the path: {}",
                path.display()
            );
            AppError::new(message, ErrorType::SystemError)
        })?;

        Ok(metadata.len() > Self::LARGE_FILE_SIZE)
    }

    /// This forms a path from the project root directory
    pub fn form_path(paths: &[&str]) -> Result<PathBuf> {
        let mut _path = env::current_dir().map_err(|_| {
            AppError::new(
                "SystemError: Unable to get current working directory",
                ErrorType::SystemError,
            )
        })?;

        let paths = paths
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();
        for segment in paths {
            _path.push(segment);
        }

        Ok(_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_get_timestamp() {
        let timestamp = SystemUtils::get_timestamp();
        assert_eq!(timestamp.len(), 23usize);
    }

    #[test]
    fn test_is_file_large() {
        let temp_file_path = PathBuf::from("./data.txt");

        if(!temp_file_path.exists()){
            File::create_new(temp_file_path.clone()).unwrap();
        }
        let is_large = SystemUtils::is_file_large(&temp_file_path).unwrap();
        assert_eq!(is_large, false);
    }
}
