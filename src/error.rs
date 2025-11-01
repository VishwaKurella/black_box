use std::fmt::{self, Debug, Display};

/// A convenience type alias for functions that return a result with [`AppError`].
///
/// # Author
/// K Vishwanath
pub type Result<T> = std::result::Result<T, AppError>;

/// Enum representing the type of error that occurred.
/// Each variant can be extended to capture different categories of failures,
/// such as system-level, I/O, validation, or network-related issues.
///
/// # Author
/// K Vishwanath
#[derive(Debug)]
pub enum ErrorType {
    /// Represents a generic system-level error (e.g., environment or I/O failures).
    SystemError,
    // Future variants:
    // IoError,
    // ValidationError,
    // NetworkError,
}

/// Represents an application-level error with a message and its category.
///
/// This struct encapsulates both the descriptive message and
/// a high-level classification of the error (via [`ErrorType`]).
///
/// # Author
/// K Vishwanath
#[derive(Debug)]
pub struct AppError {
    /// The detailed error message.
    message: String,

    /// The type/category of error (see [`ErrorType`]).
    error_type: ErrorType,
}

/// Implementation for AppError.
///
/// # Author
/// K Vishwanath
impl AppError {
    /// Creates a new [`AppError`] instance.
    ///
    /// # Arguments
    ///
    /// * `message` - A descriptive message about what went wrong.
    /// * `error_type` - The specific type of the error.
    ///
    /// # Returns
    /// * `Self`
    ///
    /// # Example
    /// ```
    /// use crate::error::{AppError, ErrorType};
    /// let err = AppError::new("Failed to open file", ErrorType::SystemError);
    /// ```
    pub fn new(message: impl Into<String>, error_type: ErrorType) -> Self {
        Self {
            message: message.into(),
            error_type,
        }
    }

    /// Returns the descriptive error message.
    ///
    /// # Returns
    /// * `&str`
    pub fn get_message(&self) -> &str {
        &self.message
    }

    /// Returns the associated [`ErrorType`] of this error.
    ///
    /// # Returns
    /// * `&ErrorType`
    pub fn get_error_type(&self) -> &ErrorType {
        &self.error_type
    }
}

/// Implements [`Display`] for [`AppError`], enabling user-friendly formatting.
///
/// If your IDE (e.g., RustRover) highlights `Display` in red, this can be ignored.
/// The implementation is correct and functional.
///
/// # Author
/// K Vishwanath
impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)
    }
}
