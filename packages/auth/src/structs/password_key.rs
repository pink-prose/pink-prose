use crate::error::*;
use super::{ Argon2, Salt, UserPassword };

pub struct PasswordKey(Argon2);

impl PasswordKey {
	pub fn from_pw_and_salt(password: &UserPassword, salt: &Salt) -> Result<Self> {
		let hash = Argon2::hash_and_salt(password, salt)?;
		Ok(Self(hash))
	}

	pub fn as_bytes(&self) -> &[u8; 32] {
		self.0.as_bytes()
	}
}
