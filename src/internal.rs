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
#[must_use]
pub fn error_from_args(args: fmt::Arguments<'_>) -> impl error::Error + Send + Sync + 'static {
    FormattedError {
        message: if let Some(message) = args.as_str() {
            borrow::Cow::Borrowed(message)
        } else {
            borrow::Cow::Owned(fmt::format(args))
        },
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
#[must_use]
pub fn wrap_error_from_args(
    source: impl Into<crate::Error>,
    args: fmt::Arguments<'_>,
) -> impl error::Error + Send + Sync + 'static {
    let message = if let Some(message) = args.as_str() {
        borrow::Cow::Borrowed(message)
    } else {
        borrow::Cow::Owned(fmt::format(args))
    };
    FormattedWrapError {
        source: source.into(),
        message,
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
