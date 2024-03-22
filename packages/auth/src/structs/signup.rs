use super::{
	Email,
	EmailVerificationToken,
	EncryptedPrivateKey,
	HashedPasswordVerifier,
	Password,
	PasswordVerifier,
	PublicKey,
	Salt
};

pub struct SignupForm<ExtraData> {
	pub email: Email,
	pub password: Password,
	pub extra_data: ExtraData
}

pub struct SignupRequest<ExtraData> {
	pub email: Email,
	pub salt: Salt,
	pub password_verifier: PasswordVerifier,
	pub public_key: PublicKey,
	pub encrypted_private_key: EncryptedPrivateKey,
	pub extra_data: ExtraData
}

pub struct SignupResponse {}

pub struct StoredSignupData<ExtraData> {
	pub email: Email,
	pub salt: Salt,
	pub hashed_password_verifier: HashedPasswordVerifier,
	pub verifier_salt: Salt,
	pub public_key: PublicKey,
	pub encrypted_private_key: EncryptedPrivateKey,
	pub email_verification_token: EmailVerificationToken,
	pub extra_data: ExtraData
}
