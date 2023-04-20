/// Represents an error that occurred while processing input data.
#[derive(Debug)]
pub struct InputError {
    details: String,
}

impl InputError {
    /// Creates a new `InputError` with the specified error message.
    pub fn new(msg: &str) -> InputError {
        InputError {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for InputError {
    /// Formats the `InputError` for display to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for InputError {
    /// Returns a short description of the error.
    fn description(&self) -> &str {
        &self.details
    }
}
