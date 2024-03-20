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
	// pub fn from_password_key(password_key: &PasswordKey) -> Self {
	// 	let hash = Blake3::hash_key_derivation(
	// 		"meadowsys/fanfic-site 2024-03-19 17:24:53 blake3 hash for password verifier",
	// 		password_key.as_bytes()
	// 	);
	//
	// 	Self(hash)
	// }
	//
	// pub fn to_string(&self) -> String {
	// 	self.0.to_string()
	// }
	//
	// pub fn from_str(s: &str) -> Result<Self> {
	// 	Ok(Self(Blake3::from_str(s)?))
	// }
	//
	// pub fn as_bytes(&self) -> &[u8; 32] {
	// 	self.0.as_bytes()
	// }
}
