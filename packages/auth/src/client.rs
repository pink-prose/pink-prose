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
	SigninS1Request,
	SigninS1Response,
	SignupData,
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
	fn submit_request(&mut self, signup_data: &SignupData<Self::ExtraData>) -> impl Future<Output = Result<(), Self::Error>>;

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

			let signup_data = SignupData {
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

pub trait ClientSigninS1: Sized {
	type Error: From<crate::Error>;
	type ExtraData;

	fn get_user_email(&mut self) -> impl Future<Output = Result<Email, Self::Error>>;
	fn get_user_extra_data(&mut self) -> impl Future<Output = Result<Self::ExtraData, Self::Error>>;
	fn submit_request(&mut self, req: &SigninS1Request, extra_data: &Self::ExtraData) -> impl Future<Output = Result<SigninS1Response, Self::Error>>;

	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
		async fn run_signin_s1<C: ClientSigninS1>(
			mut client: C
		) -> Result<(), C::Error> {
			let email = client.get_user_email().await?;
			let extra_data = client.get_user_extra_data().await?;

			let req = SigninS1Request {
				email
			};
			let SigninS1Response {
				salt,
				signin_attempt_id
			} = client.submit_request(&req, &extra_data).await?;

			todo!()
		}

		SealedFutureImpl::new(self, run_signin_s1)
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
