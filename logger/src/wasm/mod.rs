pub use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn print(s: &str) {
    log(s);
}

pub(crate) macro ansi ($($first: tt $(, $rest: tt)* $(,)?)?) {
    ""
}
