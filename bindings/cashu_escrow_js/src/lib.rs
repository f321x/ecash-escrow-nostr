use cashu_escrow_common::nostr::NostrClient;
use error::{into_err, Result};
use nostr_sdk::prelude::*;
use wasm_bindgen::prelude::*;

mod error;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = NostrClient)]
pub struct JsNostrClient {
    _inner: NostrClient,
}

#[wasm_bindgen(js_class = NostrClient)]
impl JsNostrClient {
    #[wasm_bindgen(constructor)]
    pub async fn new(key: &str) -> Result<JsNostrClient> {
        let keys = Keys::parse(key).map_err(into_err)?;
        let _inner = NostrClient::new(keys).await.map_err(into_err)?;
        Ok(Self { _inner })
    }
}
