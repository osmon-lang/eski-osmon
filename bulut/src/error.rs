use std::error::Error;

#[derive(Debug)]
pub enum VmError {
    RuntimeError(String),
    LabelNotFound(usize),
    GlobalNotFound(usize),
    Expected(String, String),
}

impl VmError {
    fn as_str(&self) -> String {
        match self.clone() {
            VmError::RuntimeError(cause) => format!("Runtime Error: `{}`", cause),
            VmError::LabelNotFound(id) => format!("Label `{}` not found", id),
            VmError::GlobalNotFound(id) => format!("Global `{}` not found", id),
            VmError::Expected(expected, found) => {
                format!("Expected `{}` found `{}`", expected, found)
            }
        }
    }
}

impl Error for VmError {
    fn description(&self) -> &str {
        match self {
            &VmError::Expected(_, _) => "Expected: ",
            &VmError::GlobalNotFound(_) => "GlobalNotFound:",
            &VmError::LabelNotFound(_) => "LabelNotFound:",
            &VmError::RuntimeError(_) => "RuntimeError:",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

use std::fmt;

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
