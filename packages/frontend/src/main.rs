#![allow(
	dead_code,
	unused_imports,
	unused_macros,
	unused_mut,
	unused_variables
)]

use self::pages::Home;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{ Deserialize, Serialize };
use tracing::{ Level, info };

mod pages;

#[derive(Clone, Debug, PartialEq, Routable, Serialize, Deserialize)]
enum Route {
	#[layout(RootLayout)]
	#[route("/")]
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
