pub struct SignupData<T> {
	pub email: String,
	pub salt: String,
	pub password_verifier: String,
	pub public_key: String,
	pub encrypted_private_key: String,
	pub extra_data: T
}
