use js_sys::Date;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub(crate) fn print_to_console(s: &str) {
    log(s);
}

pub(crate) fn print_to_logfile(_: &str) {}

pub(crate) fn time_string() -> String {
    Date::new_0().toISOString().into()
}

pub(crate) macro ansi($($first: tt $(, $rest: tt)* $(,)?)?) {
    ""
}
