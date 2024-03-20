pub(crate) mod argon2;
pub(crate) use self::argon2::Argon2;

pub(crate) mod blake3;
pub(crate) use self::blake3::Blake3;

pub(crate) mod chacha20poly1305;
pub(crate) use self::chacha20poly1305::ChaCha20Poly1305;

pub(crate) mod email;
pub(crate) use self::email::Email;

pub(crate) mod hashed_password_verifier;
pub(crate) use self::hashed_password_verifier::HashedPasswordVerifier;

pub(crate) mod keypair;
pub(crate) use self::keypair::{ EncryptedPrivateKey, Keypair, PublicKey, PrivateKey };

pub(crate) mod password;
pub(crate) use self::password::Password;

pub(crate) mod password_key;
pub(crate) use self::password_key::PasswordKey;

pub(crate) mod password_verifier;
pub(crate) use self::password_verifier::PasswordVerifier;

pub(crate) mod salt;
pub(crate) use self::salt::Salt;

pub mod signup_data;
pub use self::signup_data::{ SignupData, StoredSignupData };

use crate::error::*;

/// Provides standardised methods to convert values to and from string, for ex.
/// storing in a database.
pub trait ToFromString: Sized {
	fn to_string(&self) -> Result<String>;
	fn from_str(s: &str) -> Result<Self>;
}
