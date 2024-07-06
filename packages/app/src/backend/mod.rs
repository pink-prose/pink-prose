use super::app::App;
use actix_files::Files as ActixFiles;
use actix_web::{ App as ActixApp, HttpServer };
use actix_web::web::Data;
use anyhow::Result;
use leptos::get_configuration;
use leptos_actix::{ generate_route_list, LeptosRoutes };

pub async fn main() -> Result<()> {
	let config = get_configuration(None)
		.await
		.expect("failed to get configuration");
	let bind_addr = config.leptos_options.site_addr;
	let routes = generate_route_list(App);
	eprintln!("listening on {bind_addr}");

	let app_factory = move || {
		let site_root = &*config.leptos_options.site_root;

		ActixApp::new()
			.service(ActixFiles::new("/-", &*format!("{site_root}/-")))
			.service(ActixFiles::new("/-a", site_root))
			// TODO: favicon?
			.leptos_routes(config.leptos_options.clone(), routes.clone(), App)
			.app_data(Data::new(config.leptos_options.clone()))
	};

	HttpServer::new(app_factory)
		.bind(&bind_addr)?
		.run()
		.await
		.map_err(Into::into)
}
