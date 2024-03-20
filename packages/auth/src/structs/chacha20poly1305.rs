use crate::error::*;
use super::StructsCommon;

pub struct ChaCha20Poly1305(Vec<u8>);

impl StructsCommon for ChaCha20Poly1305 {
	fn to_string(&self) -> Result<String> {
		Ok(::hex::encode(&*self.0))
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(::hex::decode(s.as_bytes())?))
	}
}

impl ChaCha20Poly1305 {
	// pub fn encrypt_nonce0(
	// 	to_encrypt: &[u8],
	// 	key: [u8; 32]
	// ) -> Result<Self> {
	// 	use ::chacha20poly1305::{ ChaCha20Poly1305, Key, KeyInit, Nonce };
	// 	use ::chacha20poly1305::aead::Aead;
	//
	// 	let chacha_key = Key::from(key);
	// 	let cipher = ChaCha20Poly1305::new(&chacha_key);
	// 	let nonce = Nonce::from([0u8; 12]);
	//
	// 	let encrypted = cipher.encrypt(&nonce, to_encrypt)?;
	// 	Ok(Self(encrypted))
	// }
	//
	// pub fn to_string(&self) -> String {
	// 	::hex::encode(&*self.0)
	// }
	//
	// pub fn from_str(s: &str) -> Result<Self> {
	// 	let decoded = ::hex::decode(s.as_bytes())?;
	// 	Ok(Self(decoded))
	// }
}
