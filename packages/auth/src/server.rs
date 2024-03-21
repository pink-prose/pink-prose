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
	SigninAttemptID,
	SigninS1GetSalt,
	SigninS1InProgress,
	SigninS1Request,
	SigninS1Response,
	SignupData,
	StoredSignupData
};
use ::std::future::{ Future, IntoFuture };

pub trait ServerSignup: Sized {
	type Error: From<crate::Error>;
	type ExtraData;
	type EndRV;

	/// Part of step 7.5
	///
	/// Receives request information from the client.
	fn receive_request(&mut self) -> impl Future<Output = Result<SignupData<Self::ExtraData>, Self::Error>>;

	/// Part of step 8
	///
	/// Processes extra data obtained by the client. Ex. redeeming captcha tokens
	///
	/// There is a default implementation, so you don't have to implement this if
	/// you don't need to use it
	fn process_extra_data_pre(&mut self, data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}

	/// Part of step 9
	///
	/// Ensure the email is unique (ie. it doesn't already exist in the verified
	/// or unverified users database, or otherwise in any way, in the db) (ie.
	/// make sure that this is valid to make a new account with the email / extra
	/// data)
	// TODO: should the email type in these following two fns be `&Email`?
	fn ensure_unique_and_reserve(&mut self, email: &str, extra_data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>>;

	/// Part of step 12
	///
	/// Store the details and extra data so they can be retrieved later
	fn store_unverified_user_data(&mut self, data: &StoredSignupData<Self::ExtraData>) -> impl Future<Output = Result<(), Self::Error>>;

	/// Part of step 13
	///
	/// Send verification email to the email and provided email verification token
	fn send_verification_email(&mut self, email: &Email, email_verification_token: &EmailVerificationToken) -> impl Future<Output = Result<(), Self::Error>>;

	fn send_response(&mut self, response: ()) -> impl Future<Output = Result<(), Self::Error>>;

	fn process_extra_data_post(&mut self, extra_data: &Self::ExtraData) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;

	/// Part of step 14
	///
	/// Finalise anything if necessary. You can return back to the user in this
	/// function or after the run function returns, whichever is more convenient
	/// for you and whatever server lib you're using.
	fn finalise(self) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;

	/// Run signup server.
	fn run(self) -> impl SealedFuture<Result<Self::EndRV, Self::Error>> {
		async fn run_signup<S: ServerSignup>(
			mut server: S
		) -> Result<S::EndRV, S::Error> {
			// step 7.5: get the data that the client submitted in step 7,
			// and the extra data if that needed to be sent.
			let SignupData {
				email,
				salt,
				password_verifier,
				public_key,
				encrypted_private_key,
				extra_data
			} = server.receive_request().await?;

			// step 8: process extra data if necessary
			server.process_extra_data_pre(&extra_data).await?;

			// step 9: make sure the email is available. Extra data is
			// passed here too so you can ex. check a username
			server.ensure_unique_and_reserve(email.as_str(), &extra_data).await?;

			// step 10: generate salt to hash the verifier, then hash it
			let verifier_salt = Salt::generate();
			let hashed_password_verifier = HashedPasswordVerifier::from_password_verifier_and_salt(&password_verifier, &salt)?;

			// step 11: generate an email token
			let email_verification_token = EmailVerificationToken::generate();

			// step 12: store email, salt, hashed password verifier, verifier salt,
			// pub key, encrypted priv key, email verification token (in an array
			// to ensure multiple can be stored together), and do it in a seperate
			// unverified users DB
			let data = StoredSignupData {
				email,
				salt,
				hashed_password_verifier,
				verifier_salt,
				public_key,
				encrypted_private_key,
				email_verification_token,
				extra_data
			};
			server.store_unverified_user_data(&data).await?;

			// step 13: send an email verification, also containing the token (without waiting for verification,
			// return as soon as its successfully sent)
			server.send_verification_email(&data.email, &data.email_verification_token).await?;

			server.send_response(()).await?;

			server.process_extra_data_post(&data.extra_data).await?;
			// step 14: finalise anything if necessary
			server.finalise().await
		}

		SealedFutureImpl::new(self, run_signup)
	}
}

// pub trait ServerRequestVerificationEmail: Sized {
// 	type Error: From<crate::Error>;
// 	type ExtraData;

// 	fn receive_request(&mut self) -> impl Future<Output = Result<(Email, Self::ExtraData), Self::Error>>;
// 	fn process_extra_data_pre(&mut self, data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>> {
// 		async { Ok(()) }
// 	}
// 	fn check_email_in_unverified(&mut self, email: &str) -> impl Future<Output = Result<bool, Self::Error>>;
// 	// TODO: should email type in following fns be `&Email`?
// 	fn store_email_verification_token(&mut self, email: &str, email_verification_token: &EmailVerificationToken) -> impl Future<Output = Result<EmailVerificationToken, Self::Error>>;
// 	fn send_verification(&mut self, email: &str, email_verification_token: &EmailVerificationToken) -> impl Future<Output = Result<(), Self::Error>>;
// 	fn finalise(self, sent: bool) -> impl Future<Output = Result<(), Self::Error>> {
// 		async { Ok(()) }
// 	}

// 	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
// 		async fn run_request_verification_email<S: ServerRequestVerificationEmail>(
// 			mut server: S
// 		) -> Result<(), S::Error> {
// 			let (email, extra_data) = server.receive_request().await?;

// 			server.process_extra_data_pre(&extra_data).await?;

// 			if !server.check_email_in_unverified(email.as_str()).await? {
// 				// silent early bail
// 				server.finalise(false).await?;
// 				return Ok(());
// 			}

// 			// email is indeed in db, send and store
// 			let email_verification_token = EmailVerificationToken::generate();
// 			server.store_email_verification_token(email.as_str(), &email_verification_token).await?;
// 			server.send_verification(email.as_str(), &email_verification_token).await?;

// 			server.finalise(true).await?;
// 			Ok(())
// 		}

// 		SealedFutureImpl::new(self, run_request_verification_email)
// 	}
// }

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
	type ExtraData;
	type EndRV;

	fn receive_request(&mut self) -> impl Future<Output = Result<SigninS1Request<Self::ExtraData>, Self::Error>>;
	fn process_extra_data_pre(&mut self, extra_data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}
	fn get_salt_and_is_verified(&mut self) -> impl Future<Output = Result<SigninS1GetSalt, Self::Error>>;
	fn finalise_not_email_verified(self, email: Email) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;
	fn store_inprogress_signin(&mut self, in_progress_data: &SigninS1InProgress) -> impl Future<Output = Result<(), Self::Error>>;
	fn send_response(&mut self, response: &SigninS1Response) -> impl Future<Output = Result<(), Self::Error>>;
	fn finalise(self) -> impl Future<Output = Result<Self::EndRV, Self::Error>>;

	fn run(self) -> impl SealedFuture<Result<Self::EndRV, Self::Error>> {
		async fn run_signin_s1<S: ServerSigninS1>(
			mut server: S
		) -> Result<S::EndRV, S::Error> {
			let SigninS1Request {
				email,
				extra_data
			} = server.receive_request().await?;

			server.process_extra_data_pre(&extra_data).await?;

			let salt = match server.get_salt_and_is_verified().await? {
				SigninS1GetSalt::Verified(salt) => { salt }
				SigninS1GetSalt::NotVerified => { return server.finalise_not_email_verified(email).await }
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
