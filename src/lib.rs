//! This library is intended to provide simple error-handling-related helper functions
//! and types. Rather than provide its own error-related types, it is centered around
//! the [std::error::Error] type.

use std::{error, fmt};

/// Represents an arbitrary owned error
pub type Error = Box<dyn error::Error + Send + Sync + 'static>;

/// Represents an arbitrary borrowed error with a given lifetime
pub type ErrorRef<'a> = &'a (dyn error::Error + 'static);

#[derive(Debug)]
struct ErrorChain<'a> {
    error: ErrorRef<'a>,
}

impl fmt::Display for ErrorChain<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut error = self.error;
        write!(f, "{}", error)?;
        while let Some(inner) = error.source() {
            write!(f, ": {}", inner)?;
            error = inner;
        }
        Ok(())
    }
}

/// Print the complete error chain of an error, separated with colons
#[must_use]
pub fn print_error_chain<'a>(error: ErrorRef<'a>) -> impl fmt::Display + 'a {
    ErrorChain { error }
}

#[derive(Debug)]
struct SimpleStringError {
    message: &'static str,
}

impl fmt::Display for SimpleStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for SimpleStringError {}

/// Create a new error with a given message
#[must_use]
pub fn error_with_message(message: &'static str) -> Error {
    SimpleStringError { message }.into()
}

#[derive(Debug)]
struct SourceStringError {
    message: &'static str,
    source: Error,
}

impl fmt::Display for SourceStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for SourceStringError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

/// Wrap the given error in a new error, with the given error message
#[must_use]
pub fn add_message(error: Error, message: &'static str) -> Error {
    SourceStringError {
        message,
        source: error,
    }
    .into()
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    #[test]
    fn error_types() {
        let sample_error = std::fs::metadata("oihaoidbo89ya7dsuhaod8atntdao7sdy").unwrap_err();
        let owned_error: crate::Error = sample_error.into();
        let _error_ref: crate::ErrorRef = owned_error.as_ref();
        let _error_ref_2: crate::ErrorRef = owned_error.deref();
    }

    #[test]
    fn messages() {
        let e = crate::error_with_message("unknown error");
        let e = crate::add_message(e, "generic error");
        let printed = crate::print_error_chain(e.as_ref());
        assert_eq!(printed.to_string(), "generic error: unknown error");
    }
}
