#![macro_use]
use std::panic;

use wasm_bindgen::JsValue;

extern crate web_sys;


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[macro_export]
macro_rules! console_log_unsafe {
    ( $( $t:tt )* ) => {
        #[allow(unused_unsafe)] // Currently unsafe not properly recognized by analyzer
        unsafe {
            crate::console_log!($( $t )*);
        }
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

pub fn js_value_as_usize(jsvalue: JsValue) -> Option<usize> {
    match jsvalue.as_f64() {
        Some(index_float) => {
            Some(index_float as usize)
        },
        None => None
    }
}