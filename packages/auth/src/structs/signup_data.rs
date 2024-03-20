use super::{
	Email,
	EmailVerificationToken,
	EncryptedPrivateKey,
	HashedPasswordVerifier,
	PasswordVerifier,
	PublicKey,
	Salt
};

pub struct SignupData<T> {
	pub email: Email,
	pub salt: Salt,
	pub password_verifier: PasswordVerifier,
	pub public_key: PublicKey,
	pub encrypted_private_key: EncryptedPrivateKey,
	pub extra_data: T
}

pub struct StoredSignupData<T> {
	pub email: Email,
	pub salt: Salt,
	pub hashed_password_verifier: HashedPasswordVerifier,
	pub verifier_salt: Salt,
	pub public_key: PublicKey,
	pub encrypted_private_key: EncryptedPrivateKey,
	pub email_verification_token: EmailVerificationToken,
	pub extra_data: T
}
