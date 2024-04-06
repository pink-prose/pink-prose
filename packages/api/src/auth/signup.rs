use ::web_sys::window;
use ::wasm_bindgen_futures::JsFuture;

pub struct Signup {}

#[cfg(feature = "client")]
pub async fn signup_client() -> wasm_bindgen_futures::wasm_bindgen::JsValue {
	let window = window()
		.expect("should be running in browser");

	let promise = window.fetch_with_str("urllol");
	// let res = JsFuture::from(promise).await.unwrap();
	JsFuture::from(promise).await.unwrap()
	// wasm_bindgen_futures::wasm_bindgen::convert::

	// window.fetch_with_str(input)
}

#[cfg(feature = "server")]
pub fn signup_server() {}
