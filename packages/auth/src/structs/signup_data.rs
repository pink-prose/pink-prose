pub struct SignupData<T> {
	pub email: String,
	pub salt: String,
	pub password_verifier: String,
	pub public_key: String,
	pub encrypted_private_key: String,
	pub extra_data: T
}

pub struct StoredSignupData {
	pub email: String,
	pub salt: String,
	pub hashed_password_verifier: String,
	pub public_key: String,
	pub encrypted_private_key: String,
	pub email_verification_token: String
}
