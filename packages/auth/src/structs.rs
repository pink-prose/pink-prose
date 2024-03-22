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

pub mod password_reset_token;
pub use self::password_reset_token::PasswordResetToken;

pub mod password_verifier;
pub use self::password_verifier::PasswordVerifier;

pub mod salt;
pub use self::salt::Salt;

pub mod session;
pub use self::session::{ SessionID, SessionClientInfo, SessionServerInfo };

pub mod signin_attempt_id;
pub use self::signin_attempt_id::SigninAttemptID;

pub mod signin_s1;
pub use self::signin_s1::{ SigninForm, SigninS1Request, SigninS1Response, SigninS1GetSalt, SigninS1InProgress };

pub mod signin_s2;
pub use self::signin_s2::{ SigninS2Form, SigninS2Request, SigninS2Response, SigninS2UserInfo, SigninS2InProgress };

pub mod signin_s3;
pub use self::signin_s3::{ SigninS3Request, SigninS3Response, SigninS3UserInfo };

pub mod signup;
pub use self::signup::{ SignupRequest, SignupResponse, SignupForm, StoredSignupData };

pub mod text_challenge;
pub use self::text_challenge::{ TextChallenge, TextChallengeSignature };

use crate::error::Result;

/// Provides standardised methods for structs in this module
pub trait StructsCommon: Sized {
	fn to_string(&self) -> Result<String>;
	fn from_str(s: &str) -> Result<Self>;
}

pub trait Generatable: Sized {
	fn generate() -> Self;
}
