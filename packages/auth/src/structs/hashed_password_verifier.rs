use crate::error::*;
use super::{ Argon2, PasswordVerifier, Salt, StructsCommon };

pub struct HashedPasswordVerifier(Argon2);

impl StructsCommon for HashedPasswordVerifier {
	fn to_string(&self) -> Result<String> {
		self.0.to_string()
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(Argon2::from_str(s)?))
	}
}

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
