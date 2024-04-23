use super::*;

pub trait ClientSignup: Sized {
	type Error: From<Error>;

	fn submit_request(&mut self, signup_request: &SignupRequest) -> fut!(SignupResponse);
	fn finalise(self) -> fut!(());

	fn run(mut self, signup_form: SignupForm) -> sealed_fut!(()) {
		seal!(async move {
			let SignupForm {
				email,
				password
			} = signup_form;

			let Keypair { public_key, secret_key } = Keypair::generate();
			let salt = Salt::generate();

			let password_key = PasswordKey::from_pw_and_salt(&password, &salt)?;
			let password_verifier = PasswordVerifier::from_password_key(&password_key);
			let password_chacha_key = ChaChaKey::from_password_key(&password_key);
			let encrypted_secret_key = EncryptedSecretKey::encrypt_nonce0(&secret_key, &password_chacha_key)?;

			let signup_request = SignupRequest {
				email,
				salt,
				password_verifier,
				public_key,
				encrypted_secret_key
			};
			let SignupResponse {} = self.submit_request(&signup_request).await?;
			self.finalise().await
		})
	}
}
