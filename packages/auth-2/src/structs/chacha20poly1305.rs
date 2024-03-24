use crate::error::*;
use super::{ Generatable, StructsCommon, z85_to_array };
use ::chacha20poly1305::{ aead::Aead as _, KeyInit as _ };
use ::wiwi::z85::{ encode_z85, decode_z85 };
use ::rand::{ Rng, rngs::OsRng };

pub struct ChaCha20Poly1305(Vec<u8>);

impl StructsCommon for ChaCha20Poly1305 {
	fn to_string(&self) -> Result<String> {
		Ok(encode_z85(&self.0))
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(decode_z85(s.as_bytes())?))
	}
}

impl ChaCha20Poly1305 {
	pub(super) fn encrypt(
		to_encrypt: &[u8],
		key: &ChaChaKey,
		nonce: &ChaChaNonce
	) -> Result<Self> {
		use ::chacha20poly1305::{ ChaCha20Poly1305, Key, Nonce };

		let chacha_key = Key::from(key.0);
		let cipher = ChaCha20Poly1305::new(&chacha_key);
		let nonce = Nonce::from(nonce.0);

		let encrypted = cipher.encrypt(&nonce, to_encrypt)?;
		Ok(Self(encrypted))
	}

	pub(super) fn decrypt(
		&self,
		key: &ChaChaKey,
		nonce: &ChaChaNonce
	) -> Result<Vec<u8>> {
		use ::chacha20poly1305::{ ChaCha20Poly1305, Key, Nonce };

		let chacha_key = Key::from(key.0);
		let cipher = ChaCha20Poly1305::new(&chacha_key);
		let nonce = Nonce::from(nonce.0);

		let decrypted = cipher.decrypt(&nonce, &*self.0)?;
		Ok(decrypted)
	}
}

pub struct ChaChaKey([u8; 32]);

impl StructsCommon for ChaChaKey {
	fn to_string(&self) -> Result<String> {
		Ok(encode_z85(&self.0))
	}

	fn from_str(s: &str) -> Result<Self> {
		z85_to_array(s, Self)
	}
}

pub struct ChaChaNonce([u8; 12]);

impl StructsCommon for ChaChaNonce {
	fn to_string(&self) -> Result<String> {
		Ok(encode_z85(&self.0))
	}

	fn from_str(s: &str) -> Result<Self> {
		z85_to_array(s, Self)
	}
}

impl Generatable for ChaChaNonce {
	fn generate() -> Self {
		let mut buf = [0u8; 12];
		OsRng.fill(&mut buf as &mut [u8]);
		Self(buf)
	}
}
