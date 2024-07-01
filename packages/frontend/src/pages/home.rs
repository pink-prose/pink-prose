use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
	rsx! {
		"home page"
		Link { to: "/signin", "go signin page" }
	}
}
