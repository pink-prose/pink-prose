use actix_web::{ get, post, HttpResponse, Responder };
use actix_web::web::{ Json, Query };
use pink_prose_model::DiscordSignupFinalResponse;
use serde::Deserialize;
use wiwi::auth::prim::keypair::Keypair;
use wiwi::z85::encode_z85;

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

#[post("/signin/submit/discord")]
pub async fn submitter() -> impl Responder {
	let (public_key, secret_key) = Keypair::generate().into_inner();
	println!("GOTPUBLICKEYAAAKEY: {}", encode_z85(&public_key.to_bytes()));

	Json(DiscordSignupFinalResponse { secret_key })
}
