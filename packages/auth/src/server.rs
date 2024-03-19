use crate::shared_structs::*;
use crate::util::{ *, signup::* };
use ::std::future::Future;

/// Implement this for the serverside of the signup flow
pub trait ServerSignup: Sized {
	type Error: From<crate::Error>;
	/// Additional data you would like to send to the server, unprocessed,
	/// on submit. Can be `()` if you have nothing else other than what this lib requires.
	/// Should be the same as the paired client impl
	type ExtraData;

	/// Receives data from client for backend processing
	fn receive_signup_from_client(&mut self) -> impl Future<Output = Result<SubmitData<Self::ExtraData>, Self::Error>>;

	/// Processes extra data, before doing anything else. Return this value back
	/// if you want to continue to use to use this data after processing the rest of
	/// the data.
	///
	/// The default implementation passes extra_data back, so you can skip implementing
	/// this and only implement [`process_extra_data_post_processing`] if that's
	/// what you need.
	///
	/// [`process_extra_data_post_processing`]: ServerSignup::process_extra_data_post_processing
	fn process_extra_data_pre_processing(&mut self, extra_data: Self::ExtraData) -> impl Future<Output = Result<Option<Self::ExtraData>, Self::Error>> {
		async { Ok(Some(extra_data)) }
	}

	/// Sends an email, or do something else to handle verification. Return when
	/// the email has been sent, but don't wait for ex. the link in the email to be clicked.
	fn send_verification_request(&mut self, email: String) -> impl Future<Output = Result<String, Self::Error>>;

	fn store_data(&mut self, data: ToStore) -> impl Future<Output = Result<(), Self::Error>>;

	/// Process extra data, after doing everything else.
	///
	/// The default implementation is a no-op. You can ignore this if you don't need it,
	/// or only implement [`process_extra_data_pre_processing`] if that's what you need.
	///
	/// [`process_extra_data_pre_processing`]: ServerSignup::process_extra_data_pre_processing
	fn process_extra_data_post_processing(&mut self, extra_data: Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}

	/// Finalise the sign up by sending success response to the user.
	fn finalise_signup(&mut self) -> impl Future<Output = Result<(), Self::Error>>;

	fn run_server_signup(mut self, _: private::NoOverriding) -> impl Future<Output = Result<(), Self::Error>> {
		async move {
			let SubmitData {
				email,
				user_salt,
				password_verifier,
				public_key,
				encrypted_private_key,
				extra_data
			} = self.receive_signup_from_client().await?;

			let extra_data = self.process_extra_data_pre_processing(extra_data).await?;

			let password_verifier = PasswordVerifier::from_string(&password_verifier)?;

			let password_verifier_salt = Salt::generate();
			let hashed_password_verifier = HashedPasswordVerifier::from_password_verifier(&password_verifier, &password_verifier_salt)?;

			let email_verification_token = self.send_verification_request(email.clone()).await?;

			let hashed_password_verifier = hashed_password_verifier.to_string();
			let password_verifier_salt = password_verifier_salt.to_string();

			// let thig = (email, salt, hashed_password_verifier, public_key, encrypted_private_key, email_verification_token);
			self.store_data(ToStore {
				email,
				user_salt,
				hashed_password_verifier,
				password_verifier_salt,
				public_key,
				encrypted_private_key,
				email_verification_token
			}).await?;

			if let Some(extra_data) = extra_data {
				self.process_extra_data_post_processing(extra_data).await?;
			}

			self.finalise_signup().await?;

			Ok(())
		}
	}
}

pub struct ToStore {
	pub email: String,
	pub user_salt: String,
	pub hashed_password_verifier: String,
	pub password_verifier_salt: String,
	pub public_key: String,
	pub encrypted_private_key: String,
	pub email_verification_token: String
	// TODO: last time verification token generated at
}

mod private {
	pub struct NoOverriding;
}
