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
	SignupData,
	StructsCommon as _
};
use ::std::future::{ Future, IntoFuture };

pub trait ClientSignup: Sized {
	type Error: From<crate::Error>;
	type ExtraData;

	/// Part of signup step 1
	///
	/// Gets the user email, ex. from a signup form
	fn get_user_email(&mut self) -> impl Future<Output = Result<Email, Self::Error>>;

	/// Part of signup step 1
	///
	/// Gets the user password, ex. from a signup form
	fn get_user_password(&mut self) -> impl Future<Output = Result<Password, Self::Error>>;

	/// Part of signup step 1
	///
	/// Gets any extra information from the user form, ex. a captcha.
	fn get_user_extra_data(&mut self) -> impl Future<Output = Result<Self::ExtraData, Self::Error>>;

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
	fn submit_request(&mut self, signup_data: &SignupData, extra_data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>>;

	/// Part of signup step 16
	///
	/// Perform any finalisation if necessary.
	///
	/// There is a default implementation, so you don't have to implement this if
	/// you don't need to use it
	fn finalise(self) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}

	/// Run signup client.
	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
		async fn run_signup<C: ClientSignup>(
			mut client: C
		) -> Result<(), C::Error> {
			// step 1: get user details
			let email = client.get_user_email().await?;
			let password = client.get_user_password().await?;
			let extra_data = client.get_user_extra_data().await?;

			// step 2: process extra data if necessary
			client.process_extra_data_pre(&extra_data).await?;

			// step 3: generate keypair and salt
			let Keypair { public_key, private_key } = Keypair::generate();
			let salt = Salt::generate();

			// step 4: hash and salt (argon2) to get the password key
			let password_key = PasswordKey::from_pw_and_salt(&password, &salt)?;

			// step 5: hash (blake3 key derivation) password key to get password verifier
			let password_verifier = PasswordVerifier::from_password_key(&password_key);

			// step 6: encrypt private key with password key (step 4)
			let encrypted_private_key = EncryptedPrivateKey::from_private_key_and_password_key(
				&private_key,
				&password_key
			)?;

			// step 7: submit email, salt, password verifier, public key,
			// encrypted private key to the server. If extra data should be sent/processed
			// do it now too
			let signup_data = SignupData {
				email,
				salt,
				password_verifier,
				public_key,
				encrypted_private_key
			};
			client.submit_request(&signup_data, &extra_data).await?;
			// continued in server trait, until step 15

			// step 16: finalise if necessary
			client.finalise().await?;
			Ok(())
		}

		SealedFutureImpl::new(self, run_signup)
	}
}

pub trait ClientRequestVerificationEmail: Sized {
	type Error: From<crate::Error>;
	type ExtraData;

	fn get_user_email(&mut self) -> impl Future<Output = Result<Email, Self::Error>>;
	fn get_user_extra_data(&mut self) -> impl Future<Output = Result<Self::ExtraData, Self::Error>>;
	fn submit_request(&mut self, email: &Email, extra_data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>>;
	fn finalise(self) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}

	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
		async fn run_request_verification_email<C: ClientRequestVerificationEmail>(
			mut client: C
		) -> Result<(), C::Error> {
			let email = client.get_user_email().await?;
			let extra_data = client.get_user_extra_data().await?;

			client.submit_request(&email, &extra_data).await?;

			client.finalise().await?;
			Ok(())
		}

		SealedFutureImpl::new(self, run_request_verification_email)
	}
}

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

// pub trait ClientSigninS1: Sized {
// 	type Error: From<crate::Error>;

// 	fn get_user_email(&mut self) -> impl Future<Output = Result<Email, Self::Error>>;
// 	// fn send_request_stage1(&mut self) -> impl Future<Output = Result<

// 	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
// 		async fn run_signin_s1<C: ClientSigninS1>(
// 			mut client: C
// 		) -> Result<(), C::Error> {
// 			let email = client.get_user_email().await?;

// 			todo!()
// 		}

// 		SealedFutureImpl::new(self, run_signin_s1)
// 	}
// }

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
