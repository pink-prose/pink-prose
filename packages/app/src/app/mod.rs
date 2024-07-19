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

		<Router trailing_slash=TrailingSlash::Redirect>
			<main
				// style=("--cc-rosewater", move || theme().colors.rosewater.hex.to_string())
				// ... omitted rest of attrs since I'm planning to redo theme stuff anyways ~vt
			>
				<Routes>
					<Route path="" view=HomePage />
					// TODO: need an error boundary for the return pages
					<Route path="/signin" view=|| view! { <Outlet /> }>
						<Route path="" view=auth::Signin />
						<Route path="return/discord" view=auth::ReturnDiscord />
					</Route>
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
