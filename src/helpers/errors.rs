use core::panic;
use std::{
    error::Error,
    fmt::{self},
};

use js_sys::Reflect;
use wasm_bindgen::{JsError, JsValue};

#[derive(Clone, PartialEq)]
pub struct AxError {
    inner: Box<AxErrorInner>,
}

#[derive(Clone, PartialEq)]
struct AxErrorInner {
    detail: Option<String>,
    message: Option<String>,
    js: Option<JsValue>,
    call_stack: Option<String>,
    trace: Option<String>,
    stack_dump: Option<String>,

    pub(crate) signals_normal_finish: bool,
}

impl Error for AxError {}

// Some convenience addons
impl AxError {
    pub(crate) fn end_execution(&self) -> Self {
        Self {
            inner: Box::new(AxErrorInner {
                detail: self.inner.detail.clone(),
                message: self.inner.message.clone(),
                js: self.inner.js.clone(),
                signals_normal_finish: true,
                call_stack: self.inner.call_stack.clone(),
                trace: self.inner.trace.clone(),
                stack_dump: self.inner.stack_dump.clone(),
            }),
        }
    }

    pub(crate) fn normal_finish(&self) -> bool {
        self.inner.signals_normal_finish
    }

    pub(crate) fn add_detail(
        &self,
        s: String,
        call_stack: String,
        trace: String,
        stack_dump: String,
    ) -> AxError {
        AxError {
            inner: Box::new(AxErrorInner {
                detail: if !s.is_empty() {
                    Some(s)
                } else {
                    self.inner.detail.clone()
                },
                message: self.inner.message.clone(),
                js: self.inner.js.clone(),
                signals_normal_finish: self.inner.signals_normal_finish,
                call_stack: if !call_stack.is_empty() {
                    Some(call_stack)
                } else {
                    self.inner.call_stack.clone()
                },
                trace: if !trace.is_empty() {
                    Some(trace)
                } else {
                    self.inner.trace.clone()
                },
                stack_dump: if !stack_dump.is_empty() {
                    Some(stack_dump)
                } else {
                    self.inner.stack_dump.clone()
                },
            }),
        }
    }
}

// ----------------------------------------------------------------
// Convert various types to AxErrors
// ----------------------------------------------------------------
impl From<&str> for AxError {
    fn from(message: &str) -> Self {
        Self {
            inner: Box::new(AxErrorInner {
                detail: None,
                message: Some(message.to_string()),
                js: None,
                signals_normal_finish: false,
                call_stack: None,
                trace: None,
                stack_dump: None,
            }),
        }
    }
}
impl From<String> for AxError {
    fn from(message: String) -> Self {
        Self {
            inner: Box::new(AxErrorInner {
                detail: None,
                message: Some(message),
                js: None,
                signals_normal_finish: false,
                call_stack: None,
                trace: None,
                stack_dump: None,
            }),
        }
    }
}
impl From<JsError> for AxError {
    fn from(err: JsError) -> Self {
        Self {
            inner: Box::new(AxErrorInner {
                detail: None,
                message: None,
                js: Some(JsValue::from(err)),
                signals_normal_finish: false,
                call_stack: None,
                trace: None,
                stack_dump: None,
            }),
        }
    }
}
impl From<JsValue> for AxError {
    fn from(err: JsValue) -> Self {
        Self {
            inner: Box::new(AxErrorInner {
                detail: None,
                message: None,
                js: Some(err),
                signals_normal_finish: false,
                call_stack: None,
                trace: None,
                stack_dump: None,
            }),
        }
    }
}
impl From<Box<dyn std::error::Error>> for AxError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Self {
            inner: Box::new(AxErrorInner {
                detail: None,
                message: Some(err.to_string()),
                js: None,
                signals_normal_finish: false,
                call_stack: None,
                trace: None,
                stack_dump: None,
            }),
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
        let js = err.inner.js.map(stringify_js_value);
        let msg = err.inner.message;
        let detail = err.inner.detail;
        let call_stack = err.inner.call_stack;
        let trace = err.inner.trace;
        let stack_dump = err.inner.stack_dump;

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

        if let Some(c) = call_stack {
            s.push_str("Call stack: \n");
            s.push_str(&c);
            s.push('\n');
        }

        if let Some(d) = stack_dump {
            s.push_str(&d);
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
        write!(f, "{s}")
    }
}

impl fmt::Debug for AxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = String::from(self.clone());
        write!(f, "{s}")
    }
}
