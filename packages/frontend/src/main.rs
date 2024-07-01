#![allow(
	dead_code,
	unused_imports,
	unused_macros,
	unused_mut,
	unused_variables
)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{ Deserialize, Serialize };
use tracing::{ Level, info };

#[derive(Clone, Debug, PartialEq, Routable, Serialize, Deserialize)]
enum Route {
	#[route("/")]
	#[layout(RootLayout)]
	Home {},
	#[route("/signin")]
	SignIn {}
}

fn main() {
	dioxus_logger::init(Level::INFO)
		.expect("failed to init logger");
	launch(App);
}

#[component]
fn App() -> Element {
	rsx! {
		Router::<Route> {}
	}
}

#[component]
fn Home() -> Element {
	rsx! {
		"home page"
		Link { to: "/signin", "go signin page" }
	}
}

#[component]
fn SignIn() -> Element {
	rsx! {
		"signin page"
		Link { to: "/", "go home" }
	}
}

#[component]
fn RootLayout() -> Element {
	rsx! {
		NavBar {}
		Outlet::<Route> {}
	}
}

#[component]
fn NavBar() -> Element {
	let route = use_route::<Route>();
	rsx! {
		"navbar yay"
		br {}
	}
}
