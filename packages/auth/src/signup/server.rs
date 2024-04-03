use super::*;

pub trait ServerSignup: Sized {
	type Error: From<Error>;

	fn receive_request(&mut self) -> fut!(SignupRequest);
	fn ensure_unique_and_reserve(&mut self, email: &Email) -> fut!(bool);
	fn finalise_email_not_unique(self) -> fut!(());
	fn store_unverified_user_data(&mut self, data: &StoredSignupData) -> fut!(());
	fn send_verification_email(&mut self, email: &Email, email_verification_token: &EmailVerificationToken) -> fut!(());
	fn send_response(&mut self, response: &SignupResponse) -> fut!(());
	fn finalise(self) -> fut!(());

	fn run(mut self) -> sealed_fut!(()) {
		seal!(async move {
			let SignupRequest {
				email,
				salt,
				password_verifier,
				public_key,
				encrypted_secret_key
			} = self.receive_request().await?;

			let unique = self.ensure_unique_and_reserve(&email).await?;
			if !unique { return self.finalise_email_not_unique().await }

			let password_verifier_salt = Salt::generate();
			let email_verification_token = EmailVerificationToken::generate();

			let hashed_password_verifier = HashedPasswordVerifier::from_password_verifier_and_salt(
				&password_verifier,
				&password_verifier_salt
			)?;
			let last_email_token_generate_time = UTCDateTime::now();

			let stored_data = StoredSignupData {
				email,
				salt,
				hashed_password_verifier,
				password_verifier_salt,
				public_key,
				encrypted_secret_key,
				email_verification_token,
				last_email_token_generate_time
			};
			self.store_unverified_user_data(&stored_data).await?;

			self.send_verification_email(&stored_data.email, &stored_data.email_verification_token).await?;

			let response = SignupResponse {};
			self.send_response(&response).await?;

			self.finalise().await
		})
	}
}
