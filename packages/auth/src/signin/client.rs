use super::*;

pub trait ClientSignin: Sized {
	type Error: From<Error>;

	fn get_signin_form(&mut self) -> fut!(SigninForm);
	fn submit_s1_request(&mut self, request: &SigninS1Request) -> fut!(SigninS1Response);
	fn submit_s2_request(&mut self, request: &SigninS2Request) -> fut!(SigninS2Response);
	fn submit_s3_request(&mut self, request: &SigninS3Request) -> fut!(SigninS3Response);
	fn store_session_info(&mut self, session_info: &SessionClientInfo) -> fut!(());
	fn finalise(self) -> fut!(());

	fn run(self) -> sealed_fut!(()) {
		seal!(self, |mut client| async move {
			let SigninForm {
				email,
				password
			} = client.get_signin_form().await?;

			let request_s1 = SigninS1Request {
				email
			};
			let SigninS1Response {
				salt,
				signin_attempt_id
			} = client.submit_s1_request(&request_s1).await?;

			let password_key = PasswordKey::from_pw_and_salt(&password, &salt)?;
			let password_verifier = PasswordVerifier::from_password_key(&password_key);

			let request_s2 = SigninS2Request {
				signin_attempt_id,
				password_verifier
			};
			let SigninS2Response {
				encrypted_secret_key,
				signing_challenge
			} = client.submit_s2_request(&request_s2).await?;

			let password_chacha_key = ChaChaKey::from_password_key(&password_key);
			let secret_key = encrypted_secret_key.decrypt_nonce0(&password_chacha_key)?;

			let signing_challenge_signature = secret_key.sign(signing_challenge.as_bytes());

			let Keypair {
				public_key: session_public_key,
				secret_key: session_secret_key
			} = Keypair::generate();

			let request_s3 = SigninS3Request {
				signin_attempt_id: request_s2.signin_attempt_id,
				signing_challenge_signature,
				session_public_key
			};
			let SigninS3Response {
				session_id
			} = client.submit_s3_request(&request_s3).await?;

			let session_info = SessionClientInfo {
				session_id,
				session_secret_key
			};
			client.store_session_info(&session_info).await?;

			client.finalise().await
		})
	}
}
