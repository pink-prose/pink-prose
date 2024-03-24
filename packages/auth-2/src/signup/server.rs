use super::*;

pub trait ServerSignup: Sized {
	type Error: From<Error>;

	fn receive_request(&mut self) -> fut!(SignupRequest);
	fn ensure_unique_and_reserve(&mut self, email: &Email) -> fut!(bool);
	fn finalise_email_not_unique(self) -> fut!(());
	fn store_unverified_user_data(&mut self) -> fut!(());
	// fn send_verification_email(&mut self, email: &Email) -> fut!(());
	// fn send_response(&mut self, response: &SignupResponse) -> fut!(());
	fn finalise(self) -> fut!(());

	fn run(self) -> sealed_fut!(()) {
		seal!(self, |mut server| async move {
			let SignupRequest {
				email,
				salt,
				nonce,
				password_verifier,
				public_key,
				encrypted_secret_key
			} = server.receive_request().await?;

			let unique = server.ensure_unique_and_reserve(&email).await?;
			if !unique { return server.finalise_email_not_unique().await }

			// let hashed_password_verifier = todo!();
			// generate email verification token



			server.finalise().await
		})
	}
}
