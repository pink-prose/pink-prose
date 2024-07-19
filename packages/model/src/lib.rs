use serde::{ Deserialize, Serialize };
use wiwi::auth::prim::keypair::SecretKey;

#[derive(Deserialize, Serialize)]
pub struct DiscordSignupFinalResponse {
	pub secret_key: SecretKey
}
