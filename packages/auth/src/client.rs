use crate::util::{ *, signup::* };
use ::std::future::Future;

/// Implement this for the clientside of the signup flow
pub trait ClientSignup: Sized {
	/// Error type.
	type Error: From<crate::Error>;

	/// Returns user email. Return `Err` to abort the signup process
	///
	/// note: _technically_ this could be something else (username, phone number, etc),
	/// internally it is not changed, transformed, or checked, only passed around,
	/// so it doesn't _actually_ matter what this is
	fn email_from_user(&mut self) -> impl Future<Output = Result<String, Self::Error>>;

	/// Returns user password (ex. from a sign up form)
	fn password_from_user(&mut self) -> impl Future<Output = Result<String, Self::Error>>;

	fn run_client_signup(mut self, _: private::NoOverriding) -> impl Future<Output = Result<(), Self::Error>> {
		async move {
			let email = self.email_from_user().await?;
			let password = self.password_from_user().await?;
			let Keypair { public_key, private_key } = Keypair::generate();
			let salt = Salt::generate();

			let password_key = PasswordKey::from_password_and_salt(&password, &salt)?;
			let password_verifier = PasswordVerifier::from_password_key(&password_key);
			let encrypted_private_key = EncryptedPrivateKey::from_private_key_and_password_key(private_key, &password_key)?;

			todo!()
		}
	}
}

pub struct UserSubmittedInfo {
	pub user_id: String,
	pub password: String
}

mod private {
	pub struct NoOverriding;
}
