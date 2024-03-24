use crate::error::*;
use super::{ Blake3, PasswordKey, StructsCommon };

pub struct PasswordVerifier(Blake3);

impl StructsCommon for PasswordVerifier {
	fn to_string(&self) -> Result<String> {
		self.0.to_string()
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(Blake3::from_str(s)?))
	}
}

impl PasswordVerifier {
	pub(super) fn from_password_key(password_key: &PasswordKey) -> Self {
		let hash = Blake3::hash_key_derivation(
			"pink-prose/pink-prose auth-2 23 mrt 2024 23:01:46 blake3 hash for password verifier",
			password_key.to_key_bytes()
		);
		Self(hash)
	}
}
