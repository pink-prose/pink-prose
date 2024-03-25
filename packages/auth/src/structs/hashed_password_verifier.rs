use crate::internal_prelude::*;

pub struct HashedPasswordVerifier(Argon2);

// impl StructsCommon for HashedPasswordVerifier {
// 	fn to_string(&self) -> Result<String> {
// 		self.0.to_string()
// 	}

// 	fn from_str(s: &str) -> Result<Self> {
// 		Ok(Self(Argon2::from_str(s)?))
// 	}
// }

impl HashedPasswordVerifier {
	pub(crate) fn from_password_verifier_and_salt(
		password_verifier: &PasswordVerifier,
		salt: &Salt
	) -> Result<Self> {
		let hash = Argon2::hash_and_salt(password_verifier.as_bytes(), salt)?;
		Ok(Self(hash))
	}

	pub(crate) fn as_bytes(&self) -> &[u8; 32] {
		self.0.to_hash_bytes()
	}
}
