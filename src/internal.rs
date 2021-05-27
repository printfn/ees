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

pub(crate) struct WrapError {
    pub(crate) inner: crate::Error,
}

impl fmt::Debug for WrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl fmt::Display for WrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl error::Error for WrapError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.inner.source()
    }
}

pub fn make_opaque(
    error: impl error::Error + Send + Sync + 'static,
) -> impl error::Error + Send + Sync + 'static {
    error
}
