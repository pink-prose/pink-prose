use crate::error::*;
use super::{ Argon2, Salt, Password, StructsCommon };

pub struct PasswordKey(Argon2);

impl StructsCommon for PasswordKey {
	fn to_string(&self) -> Result<String> {
		self.0.to_string()
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(Argon2::from_str(s)?))
	}
}

impl PasswordKey {
	pub(super) fn from_pw_and_salt(
		password: &Password,
		salt: &Salt
	) -> Result<Self> {
		let hash = Argon2::hash_and_salt(password.as_bytes(), salt)?;
		Ok(Self(hash))
	}

	pub(super) fn to_key_bytes(&self) -> &[u8] {
		self.0.to_hash_bytes()
	}
}
