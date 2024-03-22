use crate::sealed_future::*;
use crate::structs::{
	Email,
	EncryptedPrivateKey,
	Generatable as _,
	Keypair,
	Password,
	PasswordKey,
	PasswordVerifier,
	Salt,
	SessionClientInfo,
	SigninForm,
	SigninS1Request,
	SigninS1Response,
	SigninS2Form,
	SigninS2Request,
	SigninS2Response,
	SigninS3Request,
	SigninS3Response,
	SignupRequest,
	SignupResponse,
	SignupForm,
	StructsCommon as _
};
use ::std::future::{ Future, IntoFuture };

pub trait ClientSignup: Sized {
	type Error: From<crate::Error>;
	type ExtraData;
	type EndRV;

	/// Part of signup step 1
	///
	/// Gets the user email, password, and other things, ex. from a signup form
	fn get_signup_form(&mut self) -> impl Future<Output = Result<SignupForm<Self::ExtraData>, Self::Error>>;

	/// Part of signup step 2
	///
	/// Gives you the opportunity to do anything you would like to with the extra
	/// data.
	///
	/// There is a default implementation, so you don't have to implement this if
	/// you don't need to use it
	fn process_extra_data_pre(&mut self, extra_data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}

	/// Part of signup step 7
	///
	/// Submit all the information here, and the extra data if you need, to the
	/// server for futher processing
	fn submit_request(&mut self, signup_data: &SignupRequest<Self::ExtraData>) -> impl Future<Output = Result<SignupResponse, Self::Error>>;

	fn process_extra_data_post(&mut self, extra_data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}

	/// Part of signup step 16
	///
	/// Perform any finalisation if necessary.
	fn finalise(self) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;

	/// Run signup client.
	fn run(self) -> impl SealedFuture<Result<Self::EndRV, Self::Error>> {
		async fn run_signup<C: ClientSignup>(
			mut client: C
		) -> Result<C::EndRV, C::Error> {
			// step 1: get user details
			let SignupForm {
				email,
				password,
				extra_data
			} = client.get_signup_form().await?;

			client.process_extra_data_pre(&extra_data).await?;

			let Keypair { public_key, private_key } = Keypair::generate();
			let salt = Salt::generate();

			let password_key = PasswordKey::from_pw_and_salt(&password, &salt)?;
			let password_verifier = PasswordVerifier::from_password_key(&password_key);
			let encrypted_private_key = EncryptedPrivateKey::from_private_key_and_password_key(
				&private_key,
				&password_key
			)?;

			let signup_data = SignupRequest {
				email,
				salt,
				password_verifier,
				public_key,
				encrypted_private_key,
				extra_data
			};
			client.submit_request(&signup_data).await?;

			client.process_extra_data_post(&signup_data.extra_data).await?;
			client.finalise().await
		}

		SealedFutureImpl::new(self, run_signup)
	}
}

// pub trait ClientRequestVerificationEmail: Sized {
// 	type Error: From<crate::Error>;
// 	type ExtraData;

// 	fn get_user_email(&mut self) -> impl Future<Output = Result<Email, Self::Error>>;
// 	fn get_user_extra_data(&mut self) -> impl Future<Output = Result<Self::ExtraData, Self::Error>>;
// 	fn submit_request(&mut self, email: &Email, extra_data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>>;
// 	fn finalise(self) -> impl Future<Output = Result<(), Self::Error>> {
// 		async { Ok(()) }
// 	}

// 	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
// 		async fn run_request_verification_email<C: ClientRequestVerificationEmail>(
// 			mut client: C
// 		) -> Result<(), C::Error> {
// 			let email = client.get_user_email().await?;
// 			let extra_data = client.get_user_extra_data().await?;

// 			client.submit_request(&email, &extra_data).await?;

// 			client.finalise().await?;
// 			Ok(())
// 		}

// 		SealedFutureImpl::new(self, run_request_verification_email)
// 	}
// }

// pub trait ClientRequestPasswordReset: Sized {
// 	type Error: From<crate::Error>;

// 	fn get_user_email(&mut self) -> impl Future<Output = Result<Email, Self::Error>>;
// 	fn send_email_to_server(&mut self, email: &Email) -> impl Future<Output = Result<(), Self::Error>>;

// 	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
// 		async fn run_request_pw_reset<C: ClientRequestPasswordReset>(
// 			mut client: C
// 		) -> Result<(), C::Error> {
// 			let email = client.get_user_email().await?;
// 			client.send_email_to_server(&email).await?;
// 			Ok(())
// 		}

// 		SealedFutureImpl::new(self, run_request_pw_reset)
// 	}
// }

// pub trait ClientVerificationLinkClicked: Sized {
// 	type Error: From<crate::Error>;

// 	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
// 		async fn run_verification_link_click<C: ClientVerificationLinkClicked>(
// 			client: C
// 		) -> Result<(), C::Error> {
// 			todo!()
// 		}

// 		SealedFutureImpl::new(self, run_verification_link_click)
// 	}
// }

// pub trait ClientPasswordResetLinkClicked: Sized {
// 	type Error: From<crate::Error>;

// 	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
// 		async fn run_pw_reset_link_clicked<C: ClientPasswordResetLinkClicked>(
// 			client: C
// 		) -> Result<(), C::Error> {
// 			todo!()
// 		}

// 		SealedFutureImpl::new(self, run_pw_reset_link_clicked)
// 	}
// }

pub trait ClientSignin: Sized {
	type Error: From<crate::Error>;
	type ExtraData;
	type EndRV;

	fn get_signin_form(&mut self) -> impl Future<Output = Result<SigninForm<Self::ExtraData>, Self::Error>>;
	fn process_extra_data_pre(&mut self, extra_data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}
	fn submit_s1_request(&mut self, request: &SigninS1Request<Self::ExtraData>) -> impl Future<Output = Result<SigninS1Response, Self::Error>>;
	fn submit_s2_request(&mut self, request: &SigninS2Request) -> impl Future<Output = Result<SigninS2Response, Self::Error>>;
	fn submit_s3_request(&mut self, request: &SigninS3Request<Self::ExtraData>) -> impl Future<Output = Result<SigninS3Response, Self::Error>>;
	fn store_session(&mut self, session: &SessionClientInfo) -> impl Future<Output = Result<(), Self::Error>>;
	fn process_extra_data_post(&mut self, extra_data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}
	fn finalise(self) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;

	fn run(self) -> impl SealedFuture<Result<Self::EndRV, Self::Error>> {
		async fn run_signin<C: ClientSignin>(
			mut client: C
		) -> Result<C::EndRV, C::Error> {
			let SigninForm {
				email,
				password,
				extra_data
			} = client.get_signin_form().await?;

			client.process_extra_data_pre(&extra_data).await?;

			let request = SigninS1Request {
				email,
				extra_data
			};
			let SigninS1Response {
				salt,
				signin_attempt_id
			} = client.submit_s1_request(&request).await?;

			// so we can use it later lol
			let SigninS1Request { extra_data, .. } = request;

			let password_key = PasswordKey::from_pw_and_salt(&password, &salt)?;
			let password_verifier = PasswordVerifier::from_password_key(&password_key);

			let request = SigninS2Request {
				signin_attempt_id,
				password_verifier
			};
			let SigninS2Response {
				encrypted_private_key,
				text_challenge
			} = client.submit_s2_request(&request).await?;

			let private_key = encrypted_private_key.into_private_key_with_password_key(&password_key)?;
			let text_challenge_signature = text_challenge.sign(&private_key);

			let Keypair {
				public_key: session_public_key,
				private_key: session_private_key
			} = Keypair::generate();

			let request = SigninS3Request {
				signin_attempt_id: request.signin_attempt_id,
				text_challenge_signature,
				session_public_key,
				extra_data
			};
			let SigninS3Response {
				session_id
			} = client.submit_s3_request(&request).await?;

			let session = SessionClientInfo {
				session_id,
				session_private_key
			};
			client.store_session(&session).await?;

			client.process_extra_data_post(&request.extra_data).await?;
			client.finalise().await
		}

		SealedFutureImpl::new(self, run_signin)
	}
}

// pub trait ClientAuthenticatedAPIRequest: Sized {
// 	type Error: From<crate::Error>;

// 	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
// 		async fn run_authenticated_api_req<C: ClientAuthenticatedAPIRequest>(
// 			client: C
// 		) -> Result<(), C::Error> {
// 			todo!()
// 		}

// 		SealedFutureImpl::new(self, run_authenticated_api_req)
// 	}
// }
