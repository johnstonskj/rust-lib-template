/*!
Provides a common `Error` and `Result` type and a set of common error messages.
*/

use std::fmt::{Display, Formatter};
use std::result::Result as StdResult;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// `Error` type for this crate.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Unknown,
}

///
/// `Result` type for this crate.
///
pub type Result<T> = StdResult<T, Error>;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

///
/// Required by convention to replace the deprecated `Error::description` function.
///
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::Unknown => "An unknown error occurred",
            }
        )
    }
}

///
/// Nothing more required unless you want to store the source of any error.
///
impl std::error::Error for Error {}

///
/// Turns `Err(Error::Unknown)` into `Error::Unknown.into()` which is sometimes more readable.
///
impl<T> Into<Result<T>> for Error {
    fn into(self) -> Result<T> {
        Err(self)
    }
}
