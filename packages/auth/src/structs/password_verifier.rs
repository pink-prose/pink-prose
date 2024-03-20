use super::{ Blake3, PasswordKey };

pub struct PasswordVerifier(Blake3);

impl PasswordVerifier {
	pub fn from_password_key(password_key: &PasswordKey) -> Self {
		let hash = Blake3::hash_key_derivation(
			"meadowsys/fanfic-site 2024-03-19 17:24:53 blake3 hash for password verifier",
			password_key.as_bytes()
		);

		Self(hash)
	}

	pub fn to_string(&self) -> String {
		self.0.to_string()
	}
}
