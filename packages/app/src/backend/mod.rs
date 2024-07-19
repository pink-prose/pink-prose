use super::app::App;
use actix_files::Files as ActixFiles;
use actix_web::{ App as ActixApp, HttpServer };
use actix_web::web::Data;
use anyhow::Result;
use leptos::{ get_configuration, LeptosOptions };
use leptos_actix::{ generate_route_list, LeptosRoutes };
use pink_prose_db::{ Db, DbNewParams };
use std::sync::Arc;

mod auth;

type DataLeptosOptions = Data<LeptosOptions>;
type DataDb = Data<Arc<Db>>;

pub async fn main() -> Result<()> {
	let config = get_configuration(None)
		.await
		.expect("failed to get configuration");
	let bind_addr = config.leptos_options.site_addr;
	let routes = generate_route_list(App);
	eprintln!("listening on {bind_addr}");

	let db = Db::new(DbNewParams {
		addr: "localhost:8000",
		// TODO: this needs to read from an env var
		password: "root",
		ns: "pp",
		db: "pp"
	}).await?;
	let db = Arc::new(db);

	let app_factory = move || {
		let site_root = &*config.leptos_options.site_root;

		ActixApp::new()
			// app shared state
			.app_data(DataLeptosOptions::new(config.leptos_options.clone()))
			.app_data(DataDb::new(Arc::clone(&db)))

			// signin
			.service(auth::discord::redirecter)
			.service(auth::discord::submitter)

			// static files (leptos and assets)
			.service(ActixFiles::new("/-", &*format!("{site_root}/-")))
			.service(ActixFiles::new("/-a", site_root))
			// TODO: favicon?

			// ssr
			.leptos_routes(config.leptos_options.clone(), routes.clone(), App)
	};

	HttpServer::new(app_factory)
		.bind(&bind_addr)?
		.run()
		.await
		.map_err(Into::into)
}
