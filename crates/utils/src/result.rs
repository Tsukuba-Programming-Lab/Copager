use std::error::Error as StdError;
use std::result::Result as StdResult;

use crate::error::DiagnosticError;

pub type Result<T> = StdResult<T, DiagnosticError>;

pub trait ResultExt<T>
where
    Self: Sized,
{
    fn into_diagnostics(self) -> Result<T>;
}

impl<T, E> ResultExt<T> for StdResult<T, E>
where
    E: StdError + 'static,
{
    fn into_diagnostics(self) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(DiagnosticError::from(e)),
        }
    }
}
