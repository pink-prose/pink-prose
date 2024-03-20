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
	pub(crate) fn from_pw_and_salt(
		password: &Password,
		salt: &Salt
	) -> Result<Self> {
		let hash = Argon2::hash_and_salt(password.as_bytes(), salt)?;
		Ok(Self(hash))
	}

	pub(crate) fn as_bytes(&self) -> &[u8; 32] {
		self.0.hash_bytes()
	}
}
