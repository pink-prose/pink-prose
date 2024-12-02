use actix_files::Files;
use actix_web::{ App, HttpServer };
use anyhow::Result;
use leptos::prelude::*;
use leptos_actix::{ generate_route_list, LeptosRoutes as _ };
use wiwi::macro_util::with_cloned_2;

pub async fn run() -> Result<()> {
	let config = get_configuration(None)?;
	let address = config.leptos_options.site_addr;
	let route_list = generate_route_list(pink_prose_frontend::App);

	let app_factory = move || {
		let leptos_options = &config.leptos_options;
		// let routes = generate_route_list(pink_prose_frontend::App);

		eprintln!("address is {}", config.leptos_options.site_addr);

		App::new()
			.service(Files::new("/-", format!("{}/-", leptos_options.site_root)))
			.service(Files::new("/-a", &*leptos_options.site_root))
			.leptos_routes(
				route_list.clone(),
				with_cloned_2!(&leptos_options in {
					move || pink_prose_frontend::shell(&leptos_options)
				}
			))
	};

	HttpServer::new(app_factory)
		.bind(address)?
		.run()
		.await
		.map_err(Into::into)
}
