pub mod signup {
	use crate::{ Error, Result };
	use super::*;
	use ::p384::pkcs8::{ EncodePublicKey, EncodePrivateKey, DecodePublicKey, DecodePrivateKey, LineEnding::LF };
	use ::rand::{ Rng, rngs::OsRng };

	// these are indeed supposed to be higher than default
	const ARGON2_M_COST: u32 = 0x8000;
	const ARGON2_T_COST: u32 = 4;
	const ARGON2_P_COST: u32 = 2;

	pub struct PublicKey(::p384::PublicKey);
	pub struct PrivateKey(::p384::SecretKey);

	pub struct Keypair {
		pub public_key: PublicKey,
		pub private_key: PrivateKey
	}

	impl PublicKey {
		pub fn from_private_key(private_key: &PrivateKey) -> Self {
			Self(private_key.0.public_key())
		}

		pub fn to_pem(&self) -> Result<String> {
			Ok(self.0.to_public_key_pem(LF)?)
		}

		pub fn from_pem(pem: &str) -> Result<Self> {
			Ok(Self(::p384::PublicKey::from_public_key_pem(pem)?))
		}
	}

	impl PrivateKey {
		pub fn generate() -> Self {
			Self(::p384::SecretKey::random(&mut OsRng))
		}

		pub fn to_pem(&self) -> Result<String> {
			Ok((**self.0.to_pkcs8_pem(LF)?).into())
		}

		pub fn from_pem(pem: &str) -> Result<Self> {
			Ok(Self(::p384::SecretKey::from_pkcs8_pem(pem)?))
		}
	}

	impl Keypair {
		pub fn generate() -> Self {
			let private_key = PrivateKey::generate();
			let public_key = PublicKey::from_private_key(&private_key);

			Self { public_key, private_key }
		}
	}

	pub struct Salt([u8; 64]);

	impl Salt {
		pub fn generate() -> Self {
			let mut buf = [0u8; 64];
			let mut_buf = &mut buf as &mut [u8];
			OsRng.fill(mut_buf);
			Self(buf)
		}
	}

	pub struct PasswordKey([u8; 32]);

	impl PasswordKey {
		pub fn from_password_and_salt(password: &str, salt: &Salt) -> Result<Self> {
			use ::argon2::{ Algorithm, ParamsBuilder, PasswordHasher, Version };

			let hasher = ::argon2::Argon2::new(
				Algorithm::Argon2id,
				Version::default(),
				ParamsBuilder::new()
					.m_cost(ARGON2_M_COST)
					.t_cost(ARGON2_T_COST)
					.p_cost(ARGON2_P_COST)
					.build()
					.expect("invalid argon2 params")
			);

			let mut out = [0u8; 32];
			hasher.hash_password_into(password.as_bytes(), &salt.0, &mut out)?;
			Ok(Self(out))
		}
	}

	pub struct PasswordVerifier([u8; 32]);

	impl PasswordVerifier {
		pub fn from_password_key(pw_key: &PasswordKey) -> Self {
			let hash = *::blake3::hash(&pw_key.0).as_bytes();
			Self(hash)
		}
	}

	pub struct EncryptedPrivateKey(String);

	impl EncryptedPrivateKey {
		pub fn from_private_key_and_password_key(private_key: PrivateKey, password_key: &PasswordKey) -> Result<Self> {
			use ::chacha20poly1305::{ ChaCha20Poly1305, Key, KeyInit, Nonce };
			use ::chacha20poly1305::aead::Aead;

			let key = Key::from(password_key.0);
			let cipher = ChaCha20Poly1305::new(&key);
			let nonce = Nonce::from([0u8; 12]);

			let pem = private_key.to_pem()?;
			let encrypted = cipher.encrypt(&nonce, pem.as_bytes())?;
			let encrypted_hex = ::hex::encode(&*encrypted);
			Ok(Self(encrypted_hex))
		}
	}
}
