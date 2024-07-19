use gloo_net::http::Request;
use js_sys::{ Function, Reflect };
use leptos::*;
use leptos_router::*;
use serde::{ Deserialize, Serialize };
use std::borrow::Cow;
use wasm_bindgen::{ JsCast as _, JsValue, UnwrapThrowExt as _ };
use wasm_bindgen::closure::Closure;
use web_sys::{ AddEventListenerOptions, MessageEvent, window };
use wiwi::rand::ThreadLocalChaCha20Rng;
use wiwi::with_cloned;
use wiwi::hex::{ decode_hex, encode_hex };
use wiwi::z85::{ decode_z85, encode_z85 };

const DISCORD_AUTH_STATE_SESSION_STORAGE_KEY: &str = "discord-auth-state";

#[component]
pub fn Signin() -> impl IntoView {
	let mut cb = None::<Closure<dyn Fn(MessageEvent)>>;

	let mut open_window = move |url: &'_ str| {
		let window = window().unwrap_throw();
		let popup = window.open_with_url_and_target_and_features(
			url,
			"signin-window",
			"popup,width=700,height=850,top=100,left=100"
		).unwrap_throw();

		if cb.is_none() {
			// TODO: lazy initialisation, is there better way to do this? ~vt
			cb = Some(Closure::new(with_cloned! { window in move |event: MessageEvent| {
				// stuff is expected to come as a query string already
				let stuff = event.data().as_string().unwrap_throw();
				let url = format!("/signin/return?{stuff}");
				window.location()
					.set_href(&url)
					.unwrap_throw()
			}}));
		}

		if let Some(_popup) = popup {
			window.add_event_listener_with_callback_and_add_event_listener_options(
				"message",
				cb.as_ref().unwrap().as_ref().unchecked_ref(),
				// this struct has weird chain-ish API that's not chaining but
				// chaining so I dunno if you wanna make a chaining API, please,
				// make it take ownership of `self` and then return it by value
				// again so I can actually chain!!! ~vt
				&{
					let mut not_a_real_chain = AddEventListenerOptions::new();
					not_a_real_chain.once(true);
					not_a_real_chain
				}
			).unwrap_throw();
		} else {
			// TODO: do a fallback redir here
		}
	};

	let trigger_discord = move |_| {
		let mut state_bytes = vec![0; 32];

		ThreadLocalChaCha20Rng.fill(&mut state_bytes);
		let state = encode_hex(&state_bytes);
		let url = format!("/signin/redir/discord?state={state}");

		let session_storage = window()
			.unwrap_throw()
			.session_storage()
			.unwrap_throw()
			.unwrap_throw();

		session_storage.set_item(DISCORD_AUTH_STATE_SESSION_STORAGE_KEY, &encode_z85(&state_bytes)).unwrap_throw();
		open_window(&url);
	};

	view! {
		"the signin page"
		<br />
		<button on:click=trigger_discord>"The Discord Signin Button™"</button>
	}
}

#[component]
pub fn ReturnDiscord() -> impl IntoView {
	// TODO: need an error page that's like "error auth failed pls try again"
	// and perhaps server side logging, with an issue ID that can be submitted to us if they want I guess?

	create_resource(|| {}, |_| async {
		#[inline]
		async fn finalise() -> Option<()> {
			let window = window()?;
			let opener = window.opener().ok()?;
			if !opener.is_truthy() { return None }
			let location = window.location();
			let session_storage = window.session_storage().ok()??;

			let stored_state = session_storage
				.get_item(DISCORD_AUTH_STATE_SESSION_STORAGE_KEY)
				.ok()??;
			let stored_state = decode_z85(stored_state.as_bytes()).ok()?;

			#[derive(Deserialize, Serialize)]
			struct QueryParams<'h> {
				code: Cow<'h, str>,
				state: Cow<'h, str>
			}

			let query_params_string = location.search().ok()?;
			let query_params = serde_qs::from_str::<QueryParams>(&query_params_string[1..]).ok()?;

			let state = decode_hex(query_params.state.as_bytes()).ok()?;
			if stored_state != state { return None }

			let res = Request::post("/signin/submit/discord")
				.header("content-type", "text/plain")
				.body(&*query_params.code)
				.ok()?
				.send()
				.await
				.ok()?;
			if res.status() != 200 { return None }
			// TODO: determine the response structure (seperate model crate???)
			// retrieve key, save it,
			// res.json().await;

			logging::log!("we've made it to, uh, here?");
			Some(())
		}

		let _do_something_with_the_option = finalise().await;

		// let post_msg = Reflect::get(&opener, &JsValue::from_str("postMessage")).unwrap_throw();

		// post_msg
		// 	.dyn_ref::<Function>()
		// 	.unwrap_throw()
		// 	.call1(&opener, &JsValue::from_str(&query_params))
		// 	.unwrap_throw();

		// window.close().unwrap_throw();
	});

	view! {
		"Finalising signin.. placeholder"
	}
}
