// #[cfg(feature = "client")]
pub mod client;
// #[cfg(feature = "client")]
pub use self::client::*;

pub mod server;

use crate::internal_prelude::*;

pub struct SignupForm {
	pub email: Email,
	pub password: Password
}

pub struct SignupRequest {
	pub email: Email,
	pub salt: Salt,
	pub nonce: ChaChaNonce,
	pub password_verifier: PasswordVerifier,
	pub public_key: PublicKey,
	pub encrypted_secret_key: EncryptedSecretKey
}

pub struct SignupResponse {}

pub struct StoredSignupData {}
