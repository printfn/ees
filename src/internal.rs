use std::{error, fmt};

#[derive(Debug)]
pub struct FormattedError {
    pub message: String,
}

impl fmt::Display for FormattedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for FormattedError {}
