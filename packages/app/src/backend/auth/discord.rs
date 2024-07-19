use actix_web::{ get, HttpResponse, Responder };
use actix_web::web::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
	state: String
}

#[get("/signin/redir/discord")]
pub async fn redirecter(params: Query<Params>) -> impl Responder {
	// TODO: make this env or something
	let url = "<omitted>";

	let url = format!("{url}&state={}", params.state);
	HttpResponse::Found()
		.insert_header(("location", &*url))
		.body(())
}
