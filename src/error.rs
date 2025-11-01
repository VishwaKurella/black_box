
pub enum ErrorType{
    SystemError,
}
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
}

