use crate::error::DiagnosticError;

pub type Result<T> = std::result::Result<T, DiagnosticError>;
