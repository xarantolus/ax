use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    pub(crate) fn js_debug_log(a: &str);
}

#[macro_export]
macro_rules! debug_log {
    ($str:expr) => {
        #[cfg(all(target_arch = "wasm32", debug_assertions))]
        {
            use crate::instructions::debug::js_debug_log;
            js_debug_log(&*format!("{}:{}: {}", file!(), line!(), $str));
        }

        #[cfg(all(not(target_arch = "wasm32"), debug_assertions))]
        {
            println!("{}:{}: {}", file!(), line!(), $str);
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        #[cfg(all(target_arch = "wasm32", debug_assertions))]
        {
            use crate::instructions::debug::js_debug_log;
            js_debug_log(&*format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg)*)));
        }

        #[cfg(all(not(target_arch = "wasm32"), debug_assertions))]
        {
            println!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg)*));
        }
    };
}
