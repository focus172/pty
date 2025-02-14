use crate::fd::DescriptorError;
use std::error::Error;
use std::fmt;

/// The alias `Result` learns `MasterError` possibility.

/// The enum `MasterError` defines the possible errors from constructor Master.
#[derive(Clone, Copy, Debug)]
pub enum MasterError {
    WaitpidFail,
    BadDescriptor(DescriptorError),
    PtsnameError,
}

impl fmt::Display for MasterError {
    /// The function `fmt` formats the value using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ::errno::errno())
    }
}

impl Error for MasterError {
    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            MasterError::WaitpidFail => "`libc::waitpid` returned an error",
            MasterError::BadDescriptor(_) => "the descriptor as occured an error",
            MasterError::PtsnameError => "the `ptsname` has a error",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            MasterError::BadDescriptor(ref err) => Some(err),
            _ => None,
        }
    }
}
