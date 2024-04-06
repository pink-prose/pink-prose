use crate::sealed_future::*;
use crate::structs::{
	Email,
	EmailVerificationToken,
	EncryptedPrivateKey,
	Generatable as _,
	HashedPasswordVerifier,
	Password,
	PasswordResetToken,
	PasswordVerifier,
	PublicKey,
	Salt,
	SessionID,
	SessionServerInfo,
	SigninAttemptID,
	SigninS1GetSalt,
	SigninS1InProgress,
	SigninS1Request,
	SigninS1Response,
	SigninS2InProgress,
	SigninS2Request,
	SigninS2Response,
	SigninS2UserInfo,
	SigninS3Request,
	SigninS3Response,
	SigninS3UserInfo,
	SignupRequest,
	SignupResponse,
	StoredSignupData,
	TextChallenge,
	VerificationEmailRequest
};
use ::std::future::{ Future, IntoFuture };

pub trait ServerSignup: Sized {
	type Error: From<crate::Error>;
	type EndRV;

	fn receive_request(&mut self) -> impl Future<Output = Result<SignupRequest, Self::Error>>;
	fn ensure_unique_and_reserve(&mut self, email: &Email) -> impl Future<Output = Result<bool, Self::Error>>;
	fn finalise_email_not_unique(self) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;
	fn store_unverified_user_data(&mut self, data: &StoredSignupData) -> impl Future<Output = Result<(), Self::Error>>;
	fn send_verification_email(&mut self, email: &Email, email_verification_token: &EmailVerificationToken) -> impl Future<Output = Result<(), Self::Error>>;
	fn send_response(&mut self, response: &SignupResponse) -> impl Future<Output = Result<(), Self::Error>>;
	fn finalise(self) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;

	fn run(self) -> impl SealedFuture<Result<Self::EndRV, Self::Error>> {
		async fn run_signup<S: ServerSignup>(
			mut server: S
		) -> Result<S::EndRV, S::Error> {
			let SignupRequest {
				email,
				salt,
				password_verifier,
				public_key,
				encrypted_private_key
			} = server.receive_request().await?;

			let unique = server.ensure_unique_and_reserve(&email).await?;
			if !unique { return server.finalise_email_not_unique().await }

			let verifier_salt = Salt::generate();
			let hashed_password_verifier = HashedPasswordVerifier::from_password_verifier_and_salt(&password_verifier, &salt)?;

			let email_verification_token = EmailVerificationToken::generate();

			let data = StoredSignupData {
				email,
				salt,
				hashed_password_verifier,
				verifier_salt,
				public_key,
				encrypted_private_key,
				email_verification_token
			};
			server.store_unverified_user_data(&data).await?;

			server.send_verification_email(&data.email, &data.email_verification_token).await?;

			let response = SignupResponse {
			};
			server.send_response(&response).await?;

			server.finalise().await
		}

		SealedFutureImpl::new(self, run_signup)
	}
}

pub trait ServerRequestVerificationEmail: Sized {
	type Error: From<crate::Error>;
	type EndRV;

	fn receive_request(&mut self) -> impl Future<Output = Result<VerificationEmailRequest, Self::Error>>;
	fn check_email_in_unverified(&mut self, email: &Email) -> impl Future<Output = Result<bool, Self::Error>>;
	fn send_verification_email(&mut self, email: &Email, email_verification_token: &EmailVerificationToken) -> impl Future<Output = Result<(), Self::Error>>;
	fn finalise(self, sent: bool) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;

	fn run(self) -> impl SealedFuture<Result<Self::EndRV, Self::Error>> {
		async fn run_request_verification_email<S: ServerRequestVerificationEmail>(
			mut server: S
		) -> Result<S::EndRV, S::Error> {
			let VerificationEmailRequest {
				email
			} = server.receive_request().await?;

			if !server.check_email_in_unverified(&email).await? {
				// silent early bail
				return server.finalise(false).await
			}

			let email_verification_token = EmailVerificationToken::generate();
			server.send_verification_email(&email, &email_verification_token).await?;

			server.finalise(true).await
		}

		SealedFutureImpl::new(self, run_request_verification_email)
	}
}

// pub trait ServerRequestPasswordReset: Sized {
// 	type Error: From<crate::Error>;

// 	fn receive_email_from_client(&mut self) -> impl Future<Output = Result<Email, Self::Error>>;
// 	fn check_email_in_verified(&mut self, email: &str) -> impl Future<Output = Result<bool, Self::Error>>;
// 	// TODO: should email type in following fns be `&Email`?
// 	fn store_password_reset_token(&mut self, email: &str, password_reset_token: &PasswordResetToken) -> impl Future<Output = Result<EmailVerificationToken, Self::Error>>;
// 	fn send_reset(&mut self, email: &str, password_reset_token: &PasswordResetToken) -> impl Future<Output = Result<(), Self::Error>>;
// 	fn finalise_email_request(&mut self, sent: bool) -> impl Future<Output = Result<(), Self::Error>> {
// 		async { Ok(()) }
// 	}

// 	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
// 		async fn run_request_pw_reset<S: ServerRequestPasswordReset>(
// 			mut server: S
// 		) -> Result<(), S::Error> {
// 			let email = server.receive_email_from_client().await?;

// 			if !server.check_email_in_verified(email.as_str()).await? {
// 				// silent early bail

// 				server.finalise_email_request(false).await?;
// 				return Ok(());
// 			}

// 			let password_reset_token = PasswordResetToken::generate();
// 			server.store_password_reset_token(email.as_str(), &password_reset_token).await?;
// 			server.send_reset(email.as_str(), &password_reset_token).await?;

// 			Ok(())
// 		}

// 		SealedFutureImpl::new(self, run_request_pw_reset)
// 	}
// }

// pub trait ServerVerificationLinkClicked: Sized {
// 	type Error: From<crate::Error>;

// 	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
// 		async fn run_verification_link_click<S: ServerVerificationLinkClicked>(
// 			server: S
// 		) -> Result<(), S::Error> {
// 			todo!()
// 		}

// 		SealedFutureImpl::new(self, run_verification_link_click)
// 	}
// }

// pub trait ServerPasswordResetLinkClicked: Sized {
// 	type Error: From<crate::Error>;

// 	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
// 		async fn run_pw_reset_link_clicked<S: ServerPasswordResetLinkClicked>(
// 			server: S
// 		) -> Result<(), S::Error> {
// 			todo!()
// 		}

// 		SealedFutureImpl::new(self, run_pw_reset_link_clicked)
// 	}
// }

pub trait ServerSigninS1: Sized {
	type Error: From<crate::Error>;
	type EndRV;

	fn receive_request(&mut self) -> impl Future<Output = Result<SigninS1Request, Self::Error>>;
	fn get_salt_and_is_verified(&mut self) -> impl Future<Output = Result<SigninS1GetSalt, Self::Error>>;
	fn finalise_not_email_verified(self, email: &Email) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;
	fn finalise_invalid_email(self, email: &Email) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;
	fn store_inprogress_signin(&mut self, in_progress_data: &SigninS1InProgress) -> impl Future<Output = Result<(), Self::Error>>;
	fn send_response(&mut self, response: &SigninS1Response) -> impl Future<Output = Result<(), Self::Error>>;
	fn finalise(self) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;

	fn run(self) -> impl SealedFuture<Result<Self::EndRV, Self::Error>> {
		async fn run_signin_s1<S: ServerSigninS1>(
			mut server: S
		) -> Result<S::EndRV, S::Error> {
			let SigninS1Request {
				email
			} = server.receive_request().await?;

			let salt = match server.get_salt_and_is_verified().await? {
				SigninS1GetSalt::Verified(salt) => { salt }
				SigninS1GetSalt::NotVerified => { return server.finalise_not_email_verified(&email).await }
				SigninS1GetSalt::InvalidEmail => { return server.finalise_invalid_email(&email).await }
			};

			let signin_attempt_id = SigninAttemptID::generate();

			let in_progress_data = SigninS1InProgress {
				email,
				signin_attempt_id
			};
			server.store_inprogress_signin(&in_progress_data).await?;

			let response = SigninS1Response {
				salt,
				signin_attempt_id: in_progress_data.signin_attempt_id
			};
			server.send_response(&response).await?;

			server.finalise().await
		}

		SealedFutureImpl::new(self, run_signin_s1)
	}
}

pub trait ServerSigninS2: Sized {
	type Error: From<crate::Error>;
	type EndRV;

	fn receive_request(&mut self) -> impl Future<Output = Result<SigninS2Request, Self::Error>>;
	fn fetch_inprogress_signin(&mut self, signin_attempt_id: &SigninAttemptID) -> impl Future<Output = Result<SigninS1InProgress, Self::Error>>;
	fn fetch_user_info(&mut self, email: &Email) -> impl Future<Output = Result<SigninS2UserInfo, Self::Error>>;
	fn finalise_invalid_password_verifier(self) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;
	fn store_text_challenge(&mut self, signin_attempt_id: &SigninAttemptID, challenge: &TextChallenge) -> impl Future<Output = Result<(), Self::Error>>;
	fn send_response(&mut self, response: &SigninS2Response) -> impl Future<Output = Result<(), Self::Error>>;
	fn finalise(self) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;

	fn run(self) -> impl SealedFuture<Result<Self::EndRV, Self::Error>> {
		async fn run_signin_s2<S: ServerSigninS2>(
			mut server: S
		) -> Result<S::EndRV, S::Error> {
			let SigninS2Request {
				signin_attempt_id,
				password_verifier
			} = server.receive_request().await?;

			let SigninS1InProgress {
				email,
				signin_attempt_id
			} = server.fetch_inprogress_signin(&signin_attempt_id).await?;

			let SigninS2UserInfo {
				verifier_salt,
				hashed_password_verifier,
				encrypted_private_key
			} = server.fetch_user_info(&email).await?;

			let sent_hashed_password_verifier = HashedPasswordVerifier::from_password_verifier_and_salt(&password_verifier, &verifier_salt)?;

			let hpv_same = ::constant_time_eq::constant_time_eq_n(
				hashed_password_verifier.as_bytes(),
				sent_hashed_password_verifier.as_bytes()
			);
			if !hpv_same { return server.finalise_invalid_password_verifier().await }

			let text_challenge = TextChallenge::generate();
			server.store_text_challenge(&signin_attempt_id, &text_challenge).await?;

			let response = SigninS2Response {
				encrypted_private_key,
				text_challenge
			};
			server.send_response(&response).await?;

			server.finalise().await
		}

		SealedFutureImpl::new(self, run_signin_s2)
	}
}

pub trait ServerSigninS3: Sized {
	type Error: From<crate::Error>;
	type EndRV;

	fn receive_request(&mut self) -> impl Future<Output = Result<SigninS3Request, Self::Error>>;
	fn fetch_inprogress_signin(&mut self, signin_attempt_id: &SigninAttemptID) -> impl Future<Output = Result<SigninS2InProgress, Self::Error>>;
	fn fetch_user_info(&mut self, email: &Email) -> impl Future<Output = Result<SigninS3UserInfo, Self::Error>>;
	fn finalise_invalid_signature(self, email: Email) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;
	fn store_session_info(&mut self, session_info: &SessionServerInfo) -> impl Future<Output = Result<(), Self::Error>>;
	fn send_response(&mut self, response: &SigninS3Response) -> impl Future<Output = Result<(), Self::Error>>;
	fn finalise(self) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;

	fn run(self) -> impl SealedFuture<Result<Self::EndRV, Self::Error>> {
		async fn run_signin_s3<S: ServerSigninS3>(
			mut server: S
		) -> Result<S::EndRV, S::Error> {
			let SigninS3Request {
				signin_attempt_id,
				text_challenge_signature,
				session_public_key
			} = server.receive_request().await?;

			let SigninS2InProgress {
				email,
				signin_attempt_id,
				text_challenge
			} = server.fetch_inprogress_signin(&signin_attempt_id).await?;

			let SigninS3UserInfo {
				public_key
			} = server.fetch_user_info(&email).await?;

			let verified = text_challenge.verify(&public_key, &text_challenge_signature);
			if !verified { return server.finalise_invalid_signature(email).await }

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
		}

		SealedFutureImpl::new(self, run_signin_s3)
	}
}

// pub trait ServerAuthenticatedAPIRequest: Sized {
// 	type Error: From<crate::Error>;

// 	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
// 		async fn run_authenticated_api_req<S: ServerAuthenticatedAPIRequest>(
// 			server: S
// 		) -> Result<(), S::Error> {
// 			todo!()
// 		}

// 		SealedFutureImpl::new(self, run_authenticated_api_req)
// 	}
// }