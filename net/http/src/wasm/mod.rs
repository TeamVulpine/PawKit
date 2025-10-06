use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Response, Window, js_sys::Uint8Array};

pub async fn fetch_string(url: &str) -> Option<String> {
    let window: Window = web_sys::window()?;
    let resp: Response = JsFuture::from(window.fetch_with_str(url))
        .await
        .ok()?
        .dyn_into()
        .ok()?;

    let text_value = JsFuture::from(resp.text().ok()?).await.ok()?;
    return text_value.as_string();
}

pub async fn fetch_binary(url: &str) -> Option<Vec<u8>> {
    let window: Window = web_sys::window()?;
    let resp: Response = JsFuture::from(window.fetch_with_str(url))
        .await
        .ok()?
        .dyn_into()
        .ok()?;

    let buffer = JsFuture::from(resp.array_buffer().ok()?).await.ok()?;

    let array = Uint8Array::new(&buffer);
    let mut vec = vec![0; array.length() as usize];
    array.copy_to(&mut vec);

    return Some(vec);
}
