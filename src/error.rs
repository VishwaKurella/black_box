use std::fmt::{self, Debug, Display};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum ErrorType {
    SystemError,
    // Add more variants as needed, e.g.:
    // IoError,
    // ValidationError,
    // NetworkError,
}

#[derive(Debug)]
pub struct AppError {
    message: String,
    error_type: ErrorType,
}

impl AppError {
    pub fn new(message: impl Into<String>, error_type: ErrorType) -> Self {
        Self {
            message: message.into(),
            error_type,
        }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_error_type(&self) -> &ErrorType {
        &self.error_type
    }
}

/// If using RustRover or any IDE where the code snippet impl Display for AppError highlights Display as red you can ignore this.
impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)
    }
}
