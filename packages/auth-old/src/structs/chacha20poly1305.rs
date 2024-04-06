use crate::error::*;
use super::StructsCommon;
use ::chacha20poly1305::{ aead::Aead, Key, KeyInit, Nonce };

pub struct ChaCha20Poly1305(Vec<u8>);

impl StructsCommon for ChaCha20Poly1305 {
	fn to_string(&self) -> Result<String> {
		Ok(::wiwi::hex::encode_hex(&*self.0))
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(::wiwi::hex::decode_hex(s.as_bytes())?))
	}
}

impl ChaCha20Poly1305 {
	pub(crate) fn encrypt_nonce0(
		to_encrypt: &[u8],
		key: [u8; 32]
	) -> Result<Self> {
		use ::chacha20poly1305::ChaCha20Poly1305;

		let chacha_key = Key::from(key);
		let cipher = ChaCha20Poly1305::new(&chacha_key);
		let nonce = Nonce::from([0u8; 12]);

		let encrypted = cipher.encrypt(&nonce, to_encrypt)?;
		Ok(Self(encrypted))
	}

	pub(crate) fn decrypt_nonce0(self, key: [u8; 32]) -> Result<Vec<u8>> {
		use ::chacha20poly1305::ChaCha20Poly1305;

		let chacha_key = Key::from(key);
		let cipher = ChaCha20Poly1305::new(&chacha_key);
		let nonce = Nonce::from([0u8; 12]);

		let decrypted = cipher.decrypt(&nonce, &*self.0)?;
		Ok(decrypted)
	}
}
