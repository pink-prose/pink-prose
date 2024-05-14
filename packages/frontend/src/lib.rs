/*
// TODO: remove later
#![allow(
	dead_code,
	unused_imports,
	unused_macros,
	unused_mut,
	unused_variables
)]

use ::leptos::*;
use ::leptos_meta::*;
use ::leptos_router::*;

#[cfg(feature = "hydrate")]
#[cfg_attr(feature = "hydrate", ::wasm_bindgen::prelude::wasm_bindgen)]
pub fn hydrate() {
	::std::panic::set_hook(Box::new(::console_error_panic_hook::hook));
	mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<Router>
			<Routes>
				<Route path="" view=Home />
				<Route path="/*any" view=NotFound />
			</Routes>
		</Router>
	}
}

#[component]
fn Home() -> impl IntoView {
	"something so it shows up lol"
}

#[component]
fn NotFound() -> impl IntoView {
	"404 not found"
}
*/ // lol, that /*any in that string is breaking it
*/
*/
