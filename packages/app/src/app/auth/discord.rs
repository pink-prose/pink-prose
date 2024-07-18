use leptos::*;
use wasm_bindgen::UnwrapThrowExt as _;
use web_sys::window;

#[component]
pub fn AuthDiscord() -> impl IntoView {
	create_effect(|_| {
		// TODO: we should implement state parameter too, using client side cookies etc

		// TODO: make this config / env var, using data extractor and stuffs
		let url = "<omitted>";

		window()
			.unwrap_throw()
			.location()
			.set_href(url)
			.unwrap_throw()
	});

	view! {
		"\n\nplæceholder"
	}
}
