use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    pub(crate) fn js_debug_log(a: &str);
}

macro_rules! debug_log {
    ($str:expr) => {
        #[cfg(all(target_arch = "wasm32",not(wasi), debug_assertions, not(test)))]
        {
            use $crate::helpers::debug::js_debug_log;
            js_debug_log(&*format!("{}:{}: {}", file!(), line!(), $str));
        }

        #[cfg(all(not(all(target_arch = "wasm32",not(wasi))), debug_assertions))]
        {
            println!("{}:{}: {}", file!(), line!(), $str);
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        #[cfg(all(target_arch = "wasm32",not(wasi), debug_assertions, not(test)))]
        {
            use $crate::helpers::debug::js_debug_log;
            js_debug_log(&*format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg)*)));
        }

        #[cfg(all(not(all(target_arch = "wasm32",not(wasi))), debug_assertions))]
        {
            println!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg)*));
        }
    };
}

pub(crate) use debug_log;
