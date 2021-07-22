use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use futures::StreamExt;

pub async fn fetch(url: String) -> Result<Response, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "*/*")?;

    // TODO: remove unwraps and assert
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());

    Ok(resp_value.dyn_into().unwrap())
}

pub async fn fetch_body_stream(url: String) -> Result<web_sys::ReadableStream, JsValue> {
    let response: Response = fetch(url).await?;

    // TODO: remove unwraps
    let body = response.body().unwrap();

    Ok(body)
}

pub async fn fetch_body(url: String) -> Result<Vec<u8>, JsValue> {
    let stream = fetch_body_stream(url).await?;

    let body = wasm_streams::ReadableStream::from_raw(stream.dyn_into().unwrap_throw());

    let mut stream = body.into_stream();

    let mut result: Vec<u8> = vec![];
    while let Some(Ok(chunk)) = stream.next().await {
        match chunk.as_string() {
            Some(string) => {
                result.extend(string.as_bytes());
            },
            None => break
        }
    }

    Ok(result)
}

