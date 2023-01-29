use core::panic;
use std::{
    error::Error,
    fmt::{self},
};

use js_sys::Reflect;
use wasm_bindgen::{JsError, JsValue};

#[derive(Clone, PartialEq)]
pub struct AxError {
    detail: Option<String>,
    message: Option<String>,
    js: Option<JsValue>,
    call_stack: Option<String>,
    trace: Option<String>,

    pub(crate) signals_normal_finish: bool,
}

impl Error for AxError {}

// Some convenience addons
impl AxError {
    pub(crate) fn end_execution(&self) -> Self {
        Self {
            message: self.message.clone(),
            js: self.js.clone(),
            signals_normal_finish: true,
            detail: self.detail.clone(),
            call_stack: self.call_stack.clone(),
            trace: self.trace.clone(),
        }
    }

    pub(crate) fn add_detail(&self, s: String, call_stack: String, trace: String) -> AxError {
        AxError {
            detail: if !s.is_empty() {
                Some(s)
            } else {
                self.detail.clone()
            },
            message: self.message.clone(),
            js: self.js.clone(),
            signals_normal_finish: self.signals_normal_finish,
            call_stack: if !call_stack.is_empty() {
                Some(call_stack)
            } else {
                self.call_stack.clone()
            },
            trace: if !trace.is_empty() {
                Some(trace)
            } else {
                self.trace.clone()
            },
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
            call_stack: None,
            trace: None,
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
            call_stack: None,
            trace: None,
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
            call_stack: None,
            trace: None,
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
            call_stack: None,
            trace: None,
        }
    }
}
impl From<Box<dyn std::error::Error>> for AxError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Self {
            detail: None,
            message: Some(err.to_string()),
            js: None,
            signals_normal_finish: false,
            call_stack: None,
            trace: None,
        }
    }
}

// ----------------------------------------------------------------
// Convert AxErrors to various types
// ----------------------------------------------------------------

fn stringify_js_value(js: JsValue) -> String {
    match js.as_string() {
        Some(s) => s,
        None => match Reflect::get(&js, &JsValue::from("toString")) {
            Ok(f) => js_sys::Function::from(f)
                .call0(&js)
                .unwrap_or_else(|_| JsValue::from("Error stringifying error object: Could not call toString() on JsValue"))
                .as_string()
                .unwrap_or_else(|| {
                    "Error stringifying error object: Could not convert JsValue returnd from toString() to string".to_string()
                }),
            Err(_) => "".to_string(),
        },
    }
}

impl From<AxError> for String {
    fn from(err: AxError) -> Self {
        let js = err.js.map(stringify_js_value);
        let msg = err.message;
        let detail = err.detail;
        let call_stack = err.call_stack;
        let trace = err.trace;

        let mut s = String::new();
        if let Some(d) = detail {
            s.push_str(&d);
            s.push('\n');
        }

        if let Some(m) = msg {
            s.push_str(&m);
            s.push('\n');
        }

        if let Some(j) = js {
            s.push_str(&j);
            s.push('\n');
        }

        if let Some(t) = call_stack {
            s.push_str("Call stack: \n");
            s.push_str(&t);
            s.push('\n');
        }

        if let Some(t) = trace {
            s.push_str("Trace: \n");
            s.push_str(&t);
            s.push('\n');
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

impl fmt::Display for AxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = String::from(self.clone());
        write!(f, "{}", s)
    }
}

impl fmt::Debug for AxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = String::from(self.clone());
        write!(f, "{}", s)
    }
}
