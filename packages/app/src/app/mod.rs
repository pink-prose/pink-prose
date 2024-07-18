use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod auth;
mod not_found;
mod theme;

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();
	theme::provide_theme();

	// let theme = theme::use_theme();

	view! {
		<Stylesheet id="leptos" href="/-/pink-prose.css" />
		<Title text="Welcome to Leptos" />

		<Router>
			<main
				// style=("--cc-rosewater", move || theme().colors.rosewater.hex.to_string())
				// ... omitted rest of attrs since I'm planning to redo theme stuff anyways ~vt
			>
				<Routes>
					<Route path="" view=HomePage />
					<Route path="/auth" view=auth::Auth />

					<Route path="/*any" view=not_found::NotFound />
				</Routes>
			</main>
		</Router>
	}
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
	let (count, set_count) = create_signal(0);
	let on_click = move |_| set_count.update(|count| *count += 1);

	view! {
		<h1>"Welcome to Leptos!"</h1>
		<button on:click=on_click class="bg-flamingo">"Click Me: " {count}</button>
	}
}
