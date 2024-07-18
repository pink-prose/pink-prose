use actix_web::{ get, HttpResponse, Responder };
use actix_web::web::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
	state: String,
	code: Option<String>
}

#[get("/auth/discord")]
pub async fn redirecter(params: Query<Params>) -> impl Responder {
	// // TODO: make this env or something

	if let Some(ref code) = params.code {
		HttpResponse::Ok()
			.content_type("text/plain")
			.body(format!("aha! uw code is {}", code))
	} else {
		let url = "<omitted>";

		let url = format!("{url}&state={}", urlencoding::encode(&params.state));
		HttpResponse::Found()
			.insert_header(("location", &*url))
			.body(())
	}
}
