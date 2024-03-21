use super::{
	Email,
	EncryptedPrivateKey,
	HashedPasswordVerifier,
	Password,
	PasswordVerifier,
	Salt,
	SigninAttemptID,
	TextChallenge
};

pub struct SigninS2Form {
	pub password: Password
}

pub struct SigninS2Request {
	pub signin_attempt_id: SigninAttemptID,
	pub password_verifier: PasswordVerifier
}

pub struct SigninS2Response {
	pub encrypted_private_key: EncryptedPrivateKey,
	pub text_challenge: TextChallenge
}

pub struct SigninS2UserInfo {
	pub verifier_salt: Salt,
	pub hashed_password_verifier: HashedPasswordVerifier,
	pub encrypted_private_key: EncryptedPrivateKey
}

pub struct SigninS2InProgress {
	pub email: Email,
	pub signin_attempt_id: SigninAttemptID,
	pub text_challenge: TextChallenge
}
