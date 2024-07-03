use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod theme;

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();
	theme::provide_theme();

	let theme = theme::use_theme();

	view! {
		<Stylesheet id="leptos" href="/-/pink-prose-frontend.css" />
		<Title text="Welcome to Leptos" />

		<Router>
			<main
				style=("--cc-rosewater", move || theme().colors.rosewater.hex.to_string())
				style=("--cc-flamingo", move || theme().colors.flamingo.hex.to_string())
				style=("--cc-pink", move || theme().colors.pink.hex.to_string())
				style=("--cc-mauve", move || theme().colors.mauve.hex.to_string())
				style=("--cc-red", move || theme().colors.red.hex.to_string())
				style=("--cc-maroon", move || theme().colors.maroon.hex.to_string())
				style=("--cc-peach", move || theme().colors.peach.hex.to_string())
				style=("--cc-yellow", move || theme().colors.yellow.hex.to_string())
				style=("--cc-green", move || theme().colors.green.hex.to_string())
				style=("--cc-teal", move || theme().colors.teal.hex.to_string())
				style=("--cc-sky", move || theme().colors.sky.hex.to_string())
				style=("--cc-sapphire", move || theme().colors.sapphire.hex.to_string())
				style=("--cc-blue", move || theme().colors.blue.hex.to_string())
				style=("--cc-lavender", move || theme().colors.lavender.hex.to_string())
				style=("--cc-text", move || theme().colors.text.hex.to_string())
				style=("--cc-subtext1", move || theme().colors.subtext1.hex.to_string())
				style=("--cc-subtext0", move || theme().colors.subtext0.hex.to_string())
				style=("--cc-overlay2", move || theme().colors.overlay2.hex.to_string())
				style=("--cc-overlay1", move || theme().colors.overlay1.hex.to_string())
				style=("--cc-overlay0", move || theme().colors.overlay0.hex.to_string())
				style=("--cc-surface2", move || theme().colors.surface2.hex.to_string())
				style=("--cc-surface1", move || theme().colors.surface1.hex.to_string())
				style=("--cc-surface0", move || theme().colors.surface0.hex.to_string())
				style=("--cc-base", move || theme().colors.base.hex.to_string())
				style=("--cc-mantle", move || theme().colors.mantle.hex.to_string())
				style=("--cc-crust", move || theme().colors.crust.hex.to_string())
			>
				<Routes>
					<Route path="" view=HomePage />
					<Route path="/*any" view=NotFound />
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

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
	// set an HTTP status code 404
	// this is feature gated because it can only be done during
	// initial server-side rendering
	// if you navigate to the 404 page subsequently, the status
	// code will not be set because there is not a new HTTP request
	// to the server
	#[cfg(feature = "ssr")] {
		// this can be done inline because it's synchronous
		// if it were async, we'd use a server function
		let resp = expect_context::<leptos_actix::ResponseOptions>();
		resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
	}

	view! { <h1>"Not Found"</h1> }
}
