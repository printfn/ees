use std::{borrow, error, fmt};

#[derive(Debug)]
struct FormattedError {
    message: borrow::Cow<'static, str>,
}

impl fmt::Display for FormattedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for FormattedError {}

#[inline]
pub fn error_from_string_literal(
    message: &'static str,
) -> impl error::Error + Send + Sync + 'static {
    FormattedError {
        message: borrow::Cow::Borrowed(message),
    }
}

#[inline]
pub fn error_from_string(message: String) -> impl error::Error + Send + Sync + 'static {
    FormattedError {
        message: borrow::Cow::Owned(message),
    }
}

#[derive(Debug)]
struct FormattedWrapError {
    message: borrow::Cow<'static, str>,
    source: crate::Error,
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

#[inline]
pub fn wrap_error_from_string_literal(
    source: impl Into<crate::Error>,
    message: &'static str,
) -> impl error::Error + Send + Sync + 'static {
    FormattedWrapError {
        message: borrow::Cow::Borrowed(message),
        source: source.into(),
    }
}

#[inline]
pub fn wrap_error_from_string(
    source: impl Into<crate::Error>,
    message: String,
) -> impl error::Error + Send + Sync + 'static {
    FormattedWrapError {
        message: borrow::Cow::Owned(message),
        source: source.into(),
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

#[inline]
pub fn make_opaque(
    error: impl error::Error + Send + Sync + 'static,
) -> impl error::Error + Send + Sync + 'static {
    error
}
