use js_sys::{ Function, Reflect };
use leptos::*;
use leptos_router::*;
use serde::Deserialize;
use std::borrow::Cow;
use wasm_bindgen::{ JsCast as _, JsValue, UnwrapThrowExt as _ };
use wasm_bindgen::closure::Closure;
use web_sys::{ AddEventListenerOptions, MessageEvent, window };
use wiwi::rand::ThreadLocalChaCha20Rng;
use wiwi::with_cloned;
use wiwi::z85::encode_z85;

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
			cb = Some(Closure::new(|event: MessageEvent| {
				logging::log!("was I ever called??");
				web_sys::window()
					.unwrap_throw()
					.alert_with_message(&format!("aaaa {:?}", event.data()))
					.unwrap_throw();
			}));
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
		let state = encode_z85(&state_bytes);
		let url = format!("/signin/redir/discord?state={}", urlencoding::encode(&state));

		let session_storage = window()
			.unwrap_throw()
			.session_storage()
			.unwrap_throw()
			.unwrap_throw();

		session_storage.set_item(DISCORD_AUTH_STATE_SESSION_STORAGE_KEY, &state).unwrap_throw();
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
	#[derive(Deserialize)]
	struct QueryParams<'h> {
		code: &'h str,
		state: &'h str
	}

	create_effect(|_| {
		let window = window().unwrap_throw();
		let opener = window.opener().unwrap_throw();
		let location = window.location();

		if !opener.is_truthy() { return }

		let query_params = location.search().unwrap_throw();

		let post_msg = Reflect::get(&opener, &JsValue::from_str("postMessage")).unwrap_throw();

		post_msg
			.dyn_ref::<Function>()
			.unwrap_throw()
			.call1(&opener, &JsValue::from_str(&query_params[1..]))
			.unwrap_throw();

		window.close().unwrap_throw();
	});

	view! {
		"Finalising signin.. placeholder"
	}
}
