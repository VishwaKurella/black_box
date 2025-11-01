use std::{env, fs};
use std::path::{Path, PathBuf};
use chrono::Local;
use crate::error::{AppError, ErrorType};

pub struct SystemUtils;

impl SystemUtils {
    /// 100 MB
    const LARGE_FILE_SIZE: u64 = 104857600;

    ///
    pub fn get_timestamp() -> String {
        Local::now().format("%Y-%m-%d_%H:%M:%S,%3f").to_string()
    }

    /// It is assumed that the passed path is a file.
    pub fn is_file_large(path: &Path) -> Result<bool, AppError> {
        let metadata = fs::metadata(path.into()).map_err(|_| {
            let message = format!(
                "SystemError: Unable to get metadata of file, in the path: {}",
                path.display()
            );
            AppError::new(message, ErrorType::SystemError)
        })?;

        Ok(metadata.len() > Self::LARGE_FILE_SIZE)
    }

    /// This forms a path from the project root directory
    pub fn form_path(paths: &[&str]) -> Result<PathBuf, AppError> {
        let mut _path = env::current_dir().map_err(|_| {
            AppError::new(
                "SystemError: Unable to get current working directory",
                ErrorType::SystemError,
            )
        })?;

        let mut paths = paths.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
        for segment in paths {
            _path.push(segment);
        }

        Ok(_path)
    }
}
