#[cfg(feature = "ssr")]
#[cfg_attr(feature = "ssr", actix_web::main)]
async fn main() {
	use ::actix_files::Files;
	use ::actix_web::*;
	use ::leptos::*;
	use ::leptos_actix::{ generate_route_list, LeptosRoutes };
	use ::pinkprose_frontend::App;

	let cfg = get_configuration(None).await.unwrap();
	let address = cfg.leptos_options.site_addr;
	let routes = generate_route_list(App);
	println!("listening on {address}");

	let server = HttpServer::new(move || {
		let leptos_opts = &cfg.leptos_options;
		let site_root = &leptos_opts.site_root;

		App::new()
			.service(Files::new("/lgpa", format!("{site_root}/lgpa")))
			.service(Files::new("/sa", site_root))
			.leptos_routes(
				leptos_opts.to_owned(),
				routes.to_owned(),
				App
			)
	});

	server
		.bind(&address)
		.unwrap()
		.run()
		.await
		.unwrap()
}

#[cfg(not(feature = "ssr"))]
fn main() {
	println!("Hi you are on server binary but it isn't compiled with the `ssr` feature enabled. Are you using `cargo leptos`? look in the project readme if you're stuck.");
	::std::process::exit(69);
}
