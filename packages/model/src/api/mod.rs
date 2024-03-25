use ::bitcode::{ Decode, Encode };
use ::serde::{ Deserialize, Serialize };

#[derive(Decode, Encode, Deserialize, Serialize)]
pub struct SignupRequest {
	pub email: String,
	pub salt: Vec<u8>,
	pub password_verifier: Vec<u8>,
	pub public_key: Vec<u8>,
	pub encrypted_secret_key: Vec<u8>
}
