use super::*;

pub trait ServerSigninS1: Sized {
	type Error: From<Error>;

	fn receive_request(&mut self) -> fut!(SigninS1Request);
	fn get_salt_if_verified(&mut self) -> fut!(SigninS1GetSalt);
	fn finalise_unverified(self) -> fut!(());
	fn finalise_invalid_email(self) -> fut!(());
	fn store_in_progress_signin(&mut self, signin: &SigninS1InProgress) -> fut!(());
	fn send_response(&mut self, response: &SigninS1Response) -> fut!(());
	fn finalise(self) -> fut!(());

	fn run(self) -> sealed_fut!(()) {
		seal!(self, |mut server| async move {
			let SigninS1Request {
				email
			} = server.receive_request().await?;

			let salt = match server.get_salt_if_verified().await? {
				SigninS1GetSalt::Verified(salt) => { salt }
				SigninS1GetSalt::Unverified => { return server.finalise_unverified().await }
				SigninS1GetSalt::InvalidEmail => { return server.finalise_invalid_email().await }
			};

			let signin_attempt_id = SigninAttemptID::generate();

			let signin = SigninS1InProgress {
				email,
				signin_attempt_id,
				time: UTCDateTime::now()
			};
			server.store_in_progress_signin(&signin).await?;

			let response = SigninS1Response {
				salt,
				signin_attempt_id: signin.signin_attempt_id
			};
			server.send_response(&response).await?;
			server.finalise().await
		})
	}
}

pub trait ServerSigninS2: Sized {
	type Error: From<Error>;

	fn receive_request(&mut self) -> fut!(SigninS2Request);
	fn fetch_in_progress_signin(&mut self) -> fut!(SigninS1InProgress);
	fn fetch_user_info(&mut self, signin_attempt_id: &SigninAttemptID) -> fut!(SigninS2UserInfo);
	fn finalise_invalid_password_verifier(self) -> fut!(());
	fn store_in_progress_signin(&mut self, signin: &SigninS2InProgress) -> fut!(());
	fn send_response(&mut self, response: &SigninS2Response) -> fut!(());
	fn finalise(self) -> fut!(());

	fn run(self) -> sealed_fut!(()) {
		seal!(self, |mut server| async move {
			let SigninS2Request {
				signin_attempt_id,
				password_verifier
			} = server.receive_request().await?;

			let SigninS1InProgress {
				email,
				signin_attempt_id,
				time
			} = server.fetch_in_progress_signin().await?;

			let SigninS2UserInfo {
				hashed_password_verifier,
				password_verifier_salt,
				encrypted_secret_key
			} = server.fetch_user_info(&signin_attempt_id).await?;

			let hashed_password_verifier_new = HashedPasswordVerifier::from_password_verifier_and_salt(&password_verifier, &password_verifier_salt)?;
			let password_verifier_eq = ::constant_time_eq::constant_time_eq_32(
				hashed_password_verifier.as_bytes(),
				hashed_password_verifier_new.as_bytes()
			);
			if !password_verifier_eq { return server.finalise_invalid_password_verifier().await }

			let signing_challenge = SigningChallenge::generate();

			let signin = SigninS2InProgress {
				email,
				signin_attempt_id,
				signing_challenge,
				time: UTCDateTime::now()
			};
			server.store_in_progress_signin(&signin).await?;

			let response = SigninS2Response {
				encrypted_secret_key,
				signing_challenge: signin.signing_challenge
			};
			server.send_response(&response).await?;

			server.finalise().await
		})
	}
}

pub trait ServerSigninS3: Sized {
	type Error: From<Error>;

	fn receive_request(&mut self) -> fut!(SigninS3Request);
	fn fetch_in_progress_signin(&mut self) -> fut!(SigninS2InProgress);
	fn fetch_user_info(&mut self, signin_attempt_id: &SigninAttemptID) -> fut!(SigninS3UserInfo);
	fn finalise_invalid_signature(self) -> fut!(());
	fn store_session_info(&mut self, session_info: &SessionServerInfo) -> fut!(());
	fn send_response(&mut self, response: &SigninS3Response) -> fut!(());
	fn finalise(self) -> fut!(());

	fn run(self) -> sealed_fut!(()) {
		seal!(self, |mut server| async move {
			let SigninS3Request {
				signin_attempt_id,
				signing_challenge_signature,
				session_public_key
			} = server.receive_request().await?;

			let SigninS2InProgress {
				email,
				signin_attempt_id,
				signing_challenge,
				time
			} = server.fetch_in_progress_signin().await?;

			let SigninS3UserInfo {
				public_key
			} = server.fetch_user_info(&signin_attempt_id).await?;

			let good_sig = public_key.verify(
				signing_challenge.as_bytes(),
				&signing_challenge_signature
			);
			if !good_sig { return server.finalise_invalid_signature().await }

			let session_id = SessionID::generate();

			let session_info = SessionServerInfo {
				session_id,
				session_public_key
			};
			server.store_session_info(&session_info).await?;

			let response = SigninS3Response {
				session_id: session_info.session_id
			};
			server.send_response(&response).await?;

			server.finalise().await
		})
	}
}
