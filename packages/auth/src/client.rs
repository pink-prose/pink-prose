use crate::shared_structs::*;
use crate::util::{ *, signup::* };
use ::std::future::Future;

/// Implement this for the clientside of the signup flow
pub trait ClientSignup: Sized {
	/// Error type.
	type Error: From<crate::Error>;
	/// Additional data you would like to send to the server, unprocessed,
	/// on submit. Can be `()` if you have nothing else other than what this lib requires
	/// Should be the same as the paired client impl
	type ExtraData;

	/// Returns user email. Return `Err` to abort the signup process
	///
	/// note: _technically_ this could be something else (username, phone number, etc),
	/// internally it is not changed, transformed, or checked, only passed around,
	/// so it doesn't _actually_ matter what this is
	fn provide_email_from_user(&mut self) -> impl Future<Output = Result<String, Self::Error>>;

	/// Returns user password (ex. from a sign up form)
	fn provide_password_from_user(&mut self) -> impl Future<Output = Result<String, Self::Error>>;

	fn provide_extra_data_from_user(&mut self) -> impl Future<Output = Result<Self::ExtraData, Self::Error>>;

	/// Sends the computed data to the server for further processing
	fn send_signup_to_server(&mut self, data: SubmitData<Self::ExtraData>) -> impl Future<Output = Result<(), Self::Error>>;

	fn run_client_signup(mut self, _: private::NoOverriding) -> impl Future<Output = Result<(), Self::Error>> {
		async move {
			let email = self.provide_email_from_user().await?;
			let password = self.provide_password_from_user().await?;
			let extra_data = self.provide_extra_data_from_user().await?;

			let Keypair { public_key, private_key } = Keypair::generate();
			let uer_salt = Salt::generate();

			let password_key = PasswordKey::from_password_and_salt(&password, &uer_salt)?;
			let password_verifier = PasswordVerifier::from_password_key(&password_key);
			let encrypted_private_key = EncryptedPrivateKey::from_private_key_and_password_key(private_key, &password_key)?;

			let user_salt = uer_salt.to_string();
			let password_verifier = password_verifier.to_string();
			let public_key = public_key.to_string()?;
			let encrypted_private_key = encrypted_private_key.into_string();

			let data = SubmitData {
				email,
				user_salt,
				password_verifier,
				public_key,
				encrypted_private_key,
				extra_data
			};

			self.send_signup_to_server(data).await?;

			Ok(())
		}
	}
}

mod private {
	pub struct NoOverriding;
}
