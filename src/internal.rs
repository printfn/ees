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

#[derive(Debug)]
pub struct FormattedWrapError {
    pub message: String,
    pub source: crate::Error,
}

impl fmt::Display for FormattedWrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for FormattedWrapError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}
