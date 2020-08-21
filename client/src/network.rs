use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;

pub async fn get(address: &str) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&address, &opts)?;
    request.headers().set("Accept", "text/plain")?;
    let window = web_sys::window().unwrap();
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response_value.dyn_into().unwrap();
    let text = JsFuture::from(response.text()?).await?;
    let text = text.as_string().unwrap();
    Ok(text)
}
