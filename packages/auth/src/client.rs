use crate::sealed_future::*;
use ::std::future::{ Future, IntoFuture };
use crate::structs::{ Email, EncryptedPrivateKey, Keypair, Password, PasswordKey, PasswordVerifier, Salt, SignupData };

pub trait ClientSignup: Sized {
	type Error: From<crate::Error>;
	type ExtraData;

	fn get_user_email(&mut self) -> impl Future<Output = Result<String, Self::Error>>;
	fn get_user_password(&mut self) -> impl Future<Output = Result<String, Self::Error>>;
	fn get_user_extra_data(&mut self) -> impl Future<Output = Result<Self::ExtraData, Self::Error>>;
	fn send_signup_to_server(&mut self, signup_data: SignupData<Self::ExtraData>) -> impl Future<Output = Result<(), Self::Error>>;

	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
		SealedFutureImpl::new(self, run_signup)
	}
}

async fn run_signup<C: ClientSignup>(mut client: C) -> Result<(), C::Error> {
	let email = Email::from_string(client.get_user_email().await?);
	let password = Password::from_string(client.get_user_password().await?);
	let extra_data = client.get_user_extra_data().await?;

	let Keypair { public_key, private_key } = Keypair::generate();
	let salt = Salt::generate();

	let password_key = PasswordKey::from_pw_and_salt(&password, &salt)?;
	let password_verifier = PasswordVerifier::from_password_key(&password_key);
	let encrypted_private_key = EncryptedPrivateKey::from_private_key_and_password_key(
		&private_key,
		&password_key
	)?;

	let signup_data = SignupData {
		email: email.into_string(),
		salt: salt.to_string(),
		password_verifier: password_verifier.to_string(),
		public_key: public_key.to_string()?,
		encrypted_private_key: encrypted_private_key.to_string(),
		extra_data
	};

	client.send_signup_to_server(signup_data).await?;

	Ok(())
}
