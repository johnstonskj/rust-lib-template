/*!
Provides this crate's [`Error`] and [`Result`] types as well as helper functions.

 */

use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug)]
pub enum Error {
    /// An error was signaled by the standard library I/O functions.
    IoError { source: std::io::Error },
    /// Multiple errors were aggregated from some function below.
    MultipleErrors { sources: Vec<Error> },
    /// An unknown error occurred.
    Unknown { message: String },
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type Result<T> = StdResult<Error, T>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Construct an `Error` from the provided source error.
#[inline]
pub fn io_error(source: IoError) -> Error {
    Error::IoError { source }
}

/// Construct an `Error` from the provided message.
#[inline]
pub fn unknown_error<S: Into<String>>(message: S) -> Error {
    Error::Unknown {
        message: message.into(),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::IoError { source } => format!("An I/O error occurred; source: {}", source),
                Self::MultipleErrors { sources } => {
                    format!(
                    "Multiple errors occurred:\n{}",
                    err.iter()
                        .enumerate()
                        .map(|(i, e)| format!("{i:<3}. {e}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
                }
                Self::Unknown { message } => format!("An unknown error occurred; message: {}", message
            }
        )
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::IoError { source } => Some(source),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations From
// ------------------------------------------------------------------------------------------------

impl From<IoError> for Error {
    fn from(source: IoError) -> Self {
        Self::IoError(source)
    }
}

impl From<Error> for Error {
    fn from(sources: Error) -> Self {
        Self::MultipleErrors(vec![sources])
    }
}

impl From<Vec<Error>> for Error {
    fn from(sources: Vec<Error>) -> Self {
        Self::MultipleErrors(sources)
    }
}

impl From<&[Error]> for Error {
    fn from(sources: &[Error]) -> Self {
        Self::MultipleErrors(sources.to_vec())
    }
}

impl FromIterator<Error> for Error {
    fn from_iter<I: IntoIterator<Item = Error>>(iter: I) -> Self {
        Self::MultipleErrors(iter.into_iter().collect())
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::Unknown(message)
    }
}
