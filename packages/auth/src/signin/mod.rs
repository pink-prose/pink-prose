pub mod client;
pub mod server;

use crate::internal_prelude::*;

pub struct SigninForm {
	pub email: Email,
	pub password: Password
}

pub struct SigninS1Request {
	pub email: Email
}

pub struct SigninS1Response {
	pub salt: Salt,
	pub signin_attempt_id: SigninAttemptID
}

pub struct SigninS2Request {
	pub signin_attempt_id: SigninAttemptID,
	pub password_verifier: PasswordVerifier
}

pub struct SigninS2Response {
	pub encrypted_secret_key: EncryptedSecretKey,
	pub signing_challenge: SigningChallenge
}

pub struct SigninS3Request {
	pub signin_attempt_id: SigninAttemptID,
	pub signing_challenge_signature: Signature,
	pub session_public_key: PublicKey
}

pub struct SigninS3Response {
	pub session_id: SessionID
}

pub enum SigninS1GetSalt {
	Verified(Salt),
	Unverified,
	InvalidEmail
}

pub struct SigninS1InProgress {
	pub email: Email,
	pub signin_attempt_id: SigninAttemptID,
	pub time: UTCDateTime
}

pub struct SigninS2UserInfo {
	pub hashed_password_verifier: HashedPasswordVerifier,
	pub password_verifier_salt: Salt,
	pub encrypted_secret_key: EncryptedSecretKey
}

pub struct SigninS2InProgress {
	pub email: Email,
	pub signin_attempt_id: SigninAttemptID,
	pub signing_challenge: SigningChallenge,
	pub time: UTCDateTime,
}

pub struct SigninS3UserInfo {
	pub public_key: PublicKey
}

pub struct SessionClientInfo {
	pub session_id: SessionID,
	pub session_secret_key: SecretKey
}

pub struct SessionServerInfo {
	pub session_id: SessionID,
	pub session_public_key: PublicKey
}
