 use core::panic;
use std::fmt::{self, format};

use wasm_bindgen::{JsError, JsValue};

pub struct AxError {
    message: Option<String>,
    js: Option<JsValue>,
}

impl From<&str> for AxError {
    fn from(message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            js: None,
        }
    }
}
impl From<String> for AxError {
    fn from(message: String) -> Self {
        Self {
            message: Some(message),
            js: None,
        }
    }
}
impl From<JsError> for AxError {
    fn from(err: JsError) -> Self {
        Self {
            message: None,
            js: Some(JsValue::from(err)),
        }
    }
}
impl From<JsValue> for AxError {
    fn from(err: JsValue) -> Self {
        Self {
            message: None,
            js: Some(err),
        }
    }
}

impl From<AxError> for JsValue {
    fn from(err: AxError) -> Self {
        if let Some(v) = err.js {
            v
        } else if let Some(m) = err.message {
            JsValue::from(m)
        } else {
            panic!("AxError is empty")
        }
    }
}

impl From<AxError> for JsError {
    fn from(err: AxError) -> Self {
        JsError::new(if let Some(v) = err.js {
            format!("{:?}", v)
        } else if let Some(m) = err.message {
            m
        } else {
            panic!("AxError is empty")
        }.as_str())
    }
}

// Implement std::fmt::Display for AxError
impl fmt::Display for AxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            if let Some(ref m) = self.message {
                m.to_owned()
            } else if let Some(ref v) = self.js {
                format!("{:?}", v)
            } else {
                panic!("AxError is empty")
            }
        )
    }
}

// Implement std::fmt::Debug for AxError
impl fmt::Debug for AxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            if let Some(ref m) = self.message {
                m.to_owned()
            } else if let Some(ref v) = self.js {
                format!("{:?}", v)
            } else {
                panic!("AxError is empty")
            }
        )
    }
}
