//! This library is intended to provide simple error-handling-related helper functions
//! and types. Rather than provide its own error-related types, it is centered around
//! the [std::error::Error] trait.
//!
//! Usage:
//! ```no_run
//! use std::io::Read;
//!
//! // Use ees::Error for arbitrary owned errors
//! // You can also use ees::Result<()> as a shorthand
//! fn do_work() -> Result<(), ees::Error> {
//!     let mut file = std::fs::File::open("hello world")?;
//!     let mut contents = String::new();
//!     file.read_to_string(&mut contents)?;
//!     if contents.is_empty() {
//!         // Construct an error on the fly
//!         ees::bail!("file is empty");
//!     }
//!     Ok(())
//! }
//!
//! // Take an arbitrary borrowed error
//! fn take_an_error(error: ees::ErrorRef<'_>) {
//!     // Print the complete error chain
//!     println!("Error: {}", ees::print_error_chain(error));
//! }
//!
//! // Use ees::MainResult to automatically create nicely-
//! // formatted error messages in the main() function
//! fn main() -> ees::MainResult {
//!     do_work()?;
//!     do_work().map_err(
//!         // add additional context
//!         |e| ees::wrap!(e, "failed to do work"))?;
//!     Ok(())
//! }
//! ```

#[doc(hidden)]
pub mod internal;

use std::{error, fmt};

/// Represents an arbitrary owned error
pub type Error = Box<dyn error::Error + Send + Sync + 'static>;

/// Represents an arbitrary borrowed error with a given lifetime
pub type ErrorRef<'a> = &'a (dyn error::Error + 'static);

/// `Result<T, Error>`
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct ErrorChain<'a> {
    error: Box<dyn error::Error + 'a>,
}

impl fmt::Display for ErrorChain<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut error = self.error.as_ref();
        write!(f, "{}", &error)?;
        if f.alternate() {
            if let Some(first_inner) = error.source() {
                writeln!(f, "\n\nCaused by:")?;
                if let Some(second_inner) = first_inner.source() {
                    writeln!(f, "{: >5}: {}", 0, first_inner)?;
                    write!(f, "{: >5}: {}", 1, second_inner)?;
                    error = second_inner;
                    let mut n = 2;
                    while let Some(inner) = error.source() {
                        write!(f, "\n{: >5}: {}", n, inner)?;
                        error = inner;
                        n += 1;
                    }
                } else {
                    write!(f, "    {}", first_inner)?;
                }
            }
        } else {
            while let Some(inner) = error.source() {
                write!(f, ": {}", inner)?;
                error = inner;
            }
        }
        Ok(())
    }
}

/// Print the complete error chain of an error, separated with colons
#[must_use]
#[inline]
pub fn print_error_chain<'a>(error: impl error::Error + 'a) -> impl fmt::Display + 'a {
    ErrorChain {
        error: Box::new(error),
    }
}

/// This type wraps an arbitrary error, and is intended for use in the `main()` method
pub struct MainError {
    error: Error,
}

impl fmt::Display for MainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#}", print_error_chain(self.error.as_ref()))
    }
}

impl fmt::Debug for MainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#}", print_error_chain(self.error.as_ref()))
    }
}

impl<E: Into<Error>> From<E> for MainError {
    fn from(error: E) -> Self {
        Self {
            error: error.into(),
        }
    }
}

/// A convenient way to return arbitrary errors from `main()`
pub type MainResult = std::result::Result<(), MainError>;

/// Construct an error on the fly
#[macro_export]
macro_rules! err {
    ($fmt:expr) => {
        $crate::internal::error_from_args(::std::format_args!($fmt))
    };

    ($fmt:expr, $($args:tt)*) => {
        $crate::internal::error_from_args(::std::format_args!($fmt, $($args)*))
    };
}

/// Construct an error on the fly, and immediately return from the current function
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err(::std::convert::Into::into($crate::err!($($arg)*)));
    };
}

/// Wrap an error in a new on-the-fly error
#[macro_export]
macro_rules! wrap {
    ($source:expr, $fmt:expr) => {
        $crate::internal::wrap_error_from_args($source, ::std::format_args!($fmt))
    };

    ($source:expr, $fmt:expr, $($args:tt)*) => {
        $crate::internal::wrap_error_from_args($source, ::std::format_args!($fmt, $($args)*))
    };
}

/// Convert any error into a type that implements [std::error::Error]. This
/// is mainly useful for converting [Error](crate::Error) types to `anyhow::Error`
/// or similar.
#[inline]
pub fn to_err(error: impl Into<Error>) -> impl error::Error + Send + Sync + 'static {
    internal::WrapError {
        inner: error.into(),
    }
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
        let e = crate::err!("unknown error");
        let _e2 = crate::err!("unknown error {}{3}{1}{2}{1}", 7, 3, 5, 1);
        let e = crate::wrap!(e, "te{}{}", "st", 1);
        let e = crate::wrap!(e, "outer test");
        let printed = crate::print_error_chain(e);
        assert_eq!(printed.to_string(), "outer test: test1: unknown error");
    }

    #[test]
    fn formatted() {
        let e = crate::err!("hello {}", "world");
        let owned: crate::Error = e.into();
        assert_eq!(owned.to_string(), "hello world");
    }

    fn test_bail_main_result() -> crate::MainResult {
        crate::bail!("test bail");
    }

    #[test]
    fn test_main_result_format() {
        let e = test_bail_main_result().unwrap_err();
        assert_eq!(format!("Error: {:?}", e), "Error: test bail");
    }

    fn test_bail() -> Result<(), crate::Error> {
        crate::bail!("bailing");
    }

    #[test]
    fn to_err_tests() {
        let error: crate::Error = test_bail().unwrap_err();
        let actual_error = crate::to_err(error);
        let actual_error_2 = crate::to_err(actual_error);
        assert_eq!(
            crate::print_error_chain(&actual_error_2).to_string(),
            "bailing"
        );
    }

    #[test]
    fn test_wrap_io_err() {
        std::fs::File::open("hello")
            .map_err(|e| wrap!(e, "error"))
            .unwrap_err();
    }

    fn _return_wrap() -> Result<(), crate::Error> {
        // test macro expansion by running:
        // cargo +nightly rustc --profile=check -- -Zunstable-options --pretty=expanded --cfg test
        let e = err!("hi");
        Err(wrap!(e, "wrap"))?;
        Ok(())
    }

    #[test]
    fn multiline_error_chain() {
        // both err!() and wrap!() should show a warning if they are unused
        let e = crate::err!("unknown error {}", 7);
        let e = crate::wrap!(e, "unknown error {}", 7);
        let e = crate::wrap!(e, "unknown error {}", 18);
        assert_eq!(
            format!("{:#}", crate::print_error_chain(e)),
            "unknown error 18

Caused by:
    0: unknown error 7
    1: unknown error 7"
        );
    }

    #[test]
    fn multline_single_error() {
        let e = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "oh no");
        assert_eq!(format!("{:#}", crate::print_error_chain(e)), "oh no");
    }

    #[test]
    fn multline_two_errors() {
        let e = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "oh no");
        let e = crate::wrap!(e, "permission denied");
        assert_eq!(
            format!("{:#}", crate::print_error_chain(e)),
            "permission denied

Caused by:
    oh no"
        );
    }

    #[test]
    fn more_than_ten_errors() {
        let mut e: crate::Error =
            std::io::Error::new(std::io::ErrorKind::PermissionDenied, "oh no").into();
        for i in 0..12 {
            e = crate::wrap!(e, "permission denied {}", i).into();
        }
        assert_eq!(
            format!("{:#}", crate::print_error_chain(e.as_ref())),
            "permission denied 11

Caused by:
    0: permission denied 10
    1: permission denied 9
    2: permission denied 8
    3: permission denied 7
    4: permission denied 6
    5: permission denied 5
    6: permission denied 4
    7: permission denied 3
    8: permission denied 2
    9: permission denied 1
   10: permission denied 0
   11: oh no"
        );
    }
}
