use wasm_bindgen::prelude::*;
use web_sys::js_sys::Date;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct DefaultLoggerCallback;

impl LoggerCallback for DefaultLoggerCallback {
    fn print_to_console(&self, s: &str) {
        log(s);
    }
}

pub(crate) fn time_string() -> String {
    return Date::new_0().to_iso_string().into();
}

pub(crate) macro ansi($($first: tt $(, $rest: tt)* $(,)?)?) {
    ""
}
