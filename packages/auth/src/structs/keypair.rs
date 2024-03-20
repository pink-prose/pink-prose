use crate::error::*;
use super::{ ChaCha20Poly1305, PasswordKey, StructsCommon, Generatable };
use ::p384::pkcs8::{ EncodePublicKey as _, EncodePrivateKey as _, DecodePublicKey as _, DecodePrivateKey as _, LineEnding::LF };
use ::rand::rngs::OsRng;

pub struct Keypair {
	pub public_key: PublicKey,
	pub private_key: PrivateKey
}

impl Generatable for Keypair {
	fn generate() -> Self {
		let private_key = ::p384::SecretKey::random(&mut OsRng);
		let public_key = private_key.public_key();

		let public_key = PublicKey(public_key);
		let private_key = PrivateKey(private_key);

		Self { public_key, private_key }
	}
}

impl Keypair {
	// pub fn validate(&self) -> bool {
	// 	self.private_key.0.public_key() == self.public_key.0
	// }
}

pub struct PublicKey(::p384::PublicKey);

impl StructsCommon for PublicKey {
	fn to_string(&self) -> Result<String> {
		Ok(self.0.to_public_key_pem(LF)?)
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(::p384::PublicKey::from_public_key_pem(s)?))
	}
}

pub struct PrivateKey(::p384::SecretKey);

impl StructsCommon for PrivateKey {
	fn to_string(&self) -> Result<String> {
		Ok((**self.0.to_pkcs8_pem(LF)?).into())
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(::p384::SecretKey::from_pkcs8_pem(s)?))
	}
}

pub struct EncryptedPrivateKey(ChaCha20Poly1305);

impl StructsCommon for EncryptedPrivateKey {
	fn to_string(&self) -> Result<String> {
		self.0.to_string()
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(ChaCha20Poly1305::from_str(s)?))
	}
}

impl EncryptedPrivateKey {
	// pub fn from_private_key_and_password_key(
	// 	private_key: &PrivateKey,
	// 	password_key: &PasswordKey
	// ) -> Result<Self> {
	// 	let private_key = private_key.to_string()?;
	// 	let encrypted = ChaCha20Poly1305::encrypt_nonce0(
	// 		private_key.as_bytes(),
	// 		*password_key.as_bytes()
	// 	)?;
	//
	// 	Ok(Self(encrypted))
	// }
	//
	// pub fn to_string(&self) -> String {
	// 	self.0.to_string()
	// }
	//
	// pub fn from_str(s: &str) -> Result<Self> {
	// 	Ok(Self(ChaCha20Poly1305::from_str(s)?))
	// }
}