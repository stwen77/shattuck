//

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum RuntimeError {
    SegFault,
    AccessConflict,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            RuntimeError::SegFault => write!(f, "segfault"),
            RuntimeError::AccessConflict => write!(f, "access conflict"),
        }
    }
}

impl Error for RuntimeError {}
