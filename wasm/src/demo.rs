use wasm_bindgen::prelude::wasm_bindgen;
use service::demo;

#[wasm_bindgen]
pub async fn hello_async() -> String {
    demo::hello_async().await
}

