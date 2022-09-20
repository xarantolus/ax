use std::fmt;

use wasm_bindgen::JsValue;

pub struct AxError {
    pub message: String,
}

impl From<&str> for AxError {
    fn from(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
impl From<String> for AxError {
    fn from(message: String) -> Self {
        Self { message: message }
    }
}

impl From<AxError> for JsValue {
    fn from(err: AxError) -> Self {
        JsValue::from_str(&err.message)
    }
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
