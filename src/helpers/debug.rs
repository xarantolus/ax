use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    pub(crate) fn js_debug_log(a: &str);
}

macro_rules! debug_log {
    ($str:expr) => {
        #[allow(unused_variables)]
        let p = &*format!("{}:{}: ", file!(), line!());
        #[cfg(all(target_arch = "wasm32", debug_assertions, not(test)))]
        {
            use $crate::helpers::debug::js_debug_log;
            js_debug_log(&*format!("{: <30}{}", p, $str));
        }

        #[cfg(all(not(target_arch = "wasm32"), debug_assertions))]
        {
            println!("{: <30}{}", p, $str);
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        #[allow(unused_variables)]
        let p = &*format!("{}:{}: ", file!(), line!());
        #[cfg(all(target_arch = "wasm32", debug_assertions, not(test)))]
        {
            use $crate::helpers::debug::js_debug_log;
            js_debug_log(&*format!("{: <30}{}", p, format!($fmt, $($arg)*)));
        }

        #[cfg(all(not(target_arch = "wasm32"), debug_assertions))]
        {
            println!("{: <30}{}", p, format!($fmt, $($arg)*));
        }
    };
}

pub(crate) use debug_log;
