#[cfg(feature = "frontend")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	leptos::mount::mount_to_body(pink_prose_frontend::App);
}
