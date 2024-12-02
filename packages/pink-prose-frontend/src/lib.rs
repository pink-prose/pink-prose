#[cfg(feature = "server")]
use actix_web::http::StatusCode;
use leptos::prelude::*;
#[cfg(feature = "server")]
use leptos_actix::ResponseOptions;
use leptos_meta::{ provide_meta_context, Stylesheet, StylesheetProps, Title };
use leptos_router::path;
use leptos_router::components::{ Route, Router, Routes };

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<Stylesheet
			id = "leptos"
			href = "/-/pink-prose.css"
		/>
		<Title text = "hallo" />

		<Router>
			<main>
				<Routes fallback = move || "bweh">
					<Route path = path!("") view = HomePage />
					<Route path = path!(":any") view = NotFound />
				</Routes>
			</main>
		</Router>
	}
}

#[component]
fn HomePage() -> impl IntoView {
	let count = RwSignal::new(0usize);
	let on_click = move |_| *count.write() += 1;

	view! {
		<h1>"Hi lol"</h1>
		<button on:click = on_click>"klik me"</button>
	}
}

#[component]
fn NotFound() -> impl IntoView {
	#[cfg(feature = "server")] {
		expect_context::<ResponseOptions>()
			.set_status(StatusCode::NOT_FOUND);
	}

	view! {
		<h1>"not found"</h1>
	}
}
