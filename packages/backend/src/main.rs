// TODO: remove later
#![allow(
	dead_code,
	unused_imports,
	unused_macros,
	unused_mut,
	unused_variables
)]

use actix_web::*;

#[main]
async fn main() -> anyhow::Result<()> {
	let server = HttpServer::new(|| {
		App::new()
			.service(hello)
			.service(echo)
			.route("/manuallol", web::get().to(manual_hello))
	});

	server.bind(("127.0.0.1", 8080))?
		.run()
		.await?;

	Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("hi lol")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
	HttpResponse::Ok().body("manual hello says hello")
}
