use dioxus::prelude::*;
use tracing::{ Level, info };


#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
	#[route("/")]
	Home {},
	#[route("/blog/:id")]
	Blog { id: i32 },
}


fn main() {
	// Init logger
	dioxus_logger::init(Level::INFO).expect("failed to init logger");
	launch(App);
}

#[component]
fn App() -> Element {
	rsx! {
		Router::<Route> {}
	}
}

#[component]
fn Blog(id: i32) -> Element {
	rsx! {
		Link { to: Route::Home {}, "Go to counter" }
		"Blog post {id}"
	}
}

#[component]
fn Home() -> Element {
	let mut count = use_signal(|| 0);
	let mut text = use_signal(|| String::from("..."));

	rsx! {
		Link {
			to: Route::Blog {
				id: count()
			},
			"Go to blog"
		}
		div {
			h1 { "classic counter: {count}" }
			button { onclick: move |_| count += 1, "+1" }
			button { onclick: move |_| count -= 1, "-1" }
			button {
				onclick: move |_| async move {
					if let Ok(data) = get_server_data().await {
						tracing::info!("Client received: {}", data);
						text.set(data.clone());
						post_server_data(data).await.unwrap();
					}
				},
				"Get Server Data"
			}
			p { "Server data: {text}" }
		}
	}
}


#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
	info!("Server received: {}", data);
	Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
	Ok("Hello from the server!".to_string())
}
