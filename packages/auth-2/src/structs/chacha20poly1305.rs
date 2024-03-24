use crate::internal_prelude::*;
use ::chacha20poly1305::{ aead::Aead as _, KeyInit as _ };
use ::wiwi::z85::{ encode_z85, decode_z85 };
use ::rand::{ Rng, rngs::OsRng };

pub struct ChaCha20Poly1305(Vec<u8>);

// impl VecSerialisation for ChaCha20Poly1305 {
// 	fn to_vec(&self) -> Result<String> {
// 		Ok(encode_z85(&self.0))
// 	}

// 	fn from_bytes(s: &str) -> Result<Self> {
// 		Ok(Self(decode_z85(s.as_bytes())?))
// 	}
// }

impl ChaCha20Poly1305 {
	pub(crate) fn encrypt(
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

	pub(crate) fn decrypt(
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

// impl ArraySerialisation for ChaChaKey {
// 	const N: usize = 32;

// 	fn to_array(&self) -> Result<String> {
// 		Ok(encode_z85(&self.0))
// 	}

// 	fn from_array(a: &[u8; 12]) -> Result<Self> {
// 		Ok(Self(*a))
// 	}
// }

impl ChaChaKey {
	pub(crate) fn from_password_key(password_key: &PasswordKey) -> Self {
		Self(*password_key.to_key_bytes())
	}
}

pub struct ChaChaNonce([u8; 12]);

// impl ArraySerialisation for ChaChaNonce {
// 	const N: usize = 12;

// 	fn to_array(&self) -> Result<String> {
// 		Ok(encode_z85(&self.0))
// 	}

// 	fn from_array(a: &[u8; 12]) -> Result<Self> {
// 		Ok(Self(*a))
// 	}
// }

impl Generatable for ChaChaNonce {
	fn generate() -> Self {
		Self(rand_array())
	}
}
