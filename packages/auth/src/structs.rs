pub mod argon2;
pub use self::argon2::Argon2;

pub mod blake3;
pub use self::blake3::Blake3;

pub mod chacha20poly1305;
pub use self::chacha20poly1305::ChaCha20Poly1305;

pub mod email;
pub use self::email::Email;

pub mod email_verification_token;
pub use self::email_verification_token::EmailVerificationToken;

pub mod hashed_password_verifier;
pub use self::hashed_password_verifier::HashedPasswordVerifier;

pub mod keypair;
pub use self::keypair::{ EncryptedPrivateKey, Keypair, PublicKey, PrivateKey };

pub mod password;
pub use self::password::Password;

pub mod password_key;
pub use self::password_key::PasswordKey;

pub mod password_verifier;
pub use self::password_verifier::PasswordVerifier;

pub mod salt;
pub use self::salt::Salt;

pub mod signup_data;
pub use self::signup_data::{ SignupData, StoredSignupData };

use crate::error::Result;

/// Provides standardised methods for structs in this module
pub trait StructsCommon: Sized {
	fn to_string(&self) -> Result<String>;
	fn from_str(s: &str) -> Result<Self>;
}

pub trait Generatable: Sized {
	fn generate() -> Self;
}