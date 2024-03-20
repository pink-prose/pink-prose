use crate::error::*;
use super::{ Argon2, PasswordVerifier, Salt };

pub struct HashedPasswordVerifier(Argon2);

impl HashedPasswordVerifier {
	// pub fn from_password_verifier_and_salt(password: &PasswordVerifier, salt: &Salt) -> Result<Self> {
	// 	let hash = Argon2::hash_and_salt(password.as_bytes(), salt)?;
	// 	Ok(Self(hash))
	// }
	//
	// pub fn to_string(&self) -> String {
	// 	self.0.to_string()
	// }
}
