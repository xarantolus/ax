use core::panic;
use std::fmt::{self};

use wasm_bindgen::{JsError, JsValue};

#[derive(Debug, Clone, PartialEq)]
pub struct AxError {
    detail: Option<String>,
    message: Option<String>,
    js: Option<JsValue>,

    pub(crate) signals_normal_finish: bool,
}

// Some convenience addons
impl AxError {
    pub(crate) fn end_execution(&self) -> Self {
        Self {
            message: self.message.clone(),
            js: self.js.clone(),
            signals_normal_finish: true,
            detail: self.detail.clone(),
        }
    }

    pub(crate) fn add_detail(&self, s: String) -> AxError {
        AxError {
            detail: Some(s),
            message: self.message.clone(),
            js: self.js.clone(),
            signals_normal_finish: self.signals_normal_finish,
        }
    }
}

// ----------------------------------------------------------------
// Convert various types to AxErrors
// ----------------------------------------------------------------
impl From<&str> for AxError {
    fn from(message: &str) -> Self {
        Self {
            detail: None,
            message: Some(message.to_string()),
            js: None,
            signals_normal_finish: false,
        }
    }
}
impl From<String> for AxError {
    fn from(message: String) -> Self {
        Self {
            detail: None,
            message: Some(message),
            js: None,
            signals_normal_finish: false,
        }
    }
}
impl From<JsError> for AxError {
    fn from(err: JsError) -> Self {
        Self {
            detail: None,
            message: None,
            js: Some(JsValue::from(err)),
            signals_normal_finish: false,
        }
    }
}
impl From<JsValue> for AxError {
    fn from(err: JsValue) -> Self {
        Self {
            detail: None,
            message: None,
            js: Some(err),
            signals_normal_finish: false,
        }
    }
}

// ----------------------------------------------------------------
// Convert AxErrors to various types
// ----------------------------------------------------------------

fn stringify_js_value(js: JsValue) -> String {
    match js.as_string() {
        Some(s) => s,
        None => match js.js_typeof().as_string() {
            Some(s) => format!("JsValue of type {}", s),
            None => "stringify_js_value: value not a string and cannot apply typeof".to_string(),
        },
    }
}

impl From<AxError> for String {
    fn from(err: AxError) -> Self {
        let js = err.js.map(stringify_js_value);
        let msg = err.message;
        let detail = err.detail;

        let mut s = String::new();
        if let Some(d) = detail {
            s.push_str(&d);
        }

        if let Some(m) = msg {
            s.push_str(&m);
        }

        if let Some(j) = js {
            s.push_str(&j);
        }

        if s.is_empty() {
            panic!("AxError is empty");
        }

        s
    }
}

impl From<AxError> for JsValue {
    fn from(err: AxError) -> Self {
        let info = String::from(err);
        JsValue::from(info)
    }
}

impl From<AxError> for JsError {
    fn from(err: AxError) -> Self {
        let info: String = String::from(err);
        JsError::new(info.as_str())
    }
}

impl fmt::Display for AxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = String::from(self.clone());
        write!(f, "{}", s)
    }
}
