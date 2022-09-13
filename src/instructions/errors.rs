use std::{fmt, io::Error};

pub struct AxError {
	pub message: String,
}

// Implement std::fmt::Display for AxError
impl fmt::Display for AxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Implement std::fmt::Debug for AxError
impl fmt::Debug for AxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}
