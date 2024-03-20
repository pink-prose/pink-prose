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

	fn get_user_email(&mut self) -> impl Future<Output = Result<Email, Self::Error>>;
	fn get_user_password(&mut self) -> impl Future<Output = Result<Password, Self::Error>>;
	fn get_user_extra_data(&mut self) -> impl Future<Output = Result<Self::ExtraData, Self::Error>>;
	fn process_extra_data_pre(&mut self, extra_data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}
	fn send_signup_to_server(&mut self, signup_data: &SignupData<Self::ExtraData>) -> impl Future<Output = Result<(), Self::Error>>;
	fn finalise_signup(&mut self) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}

	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
		SealedFutureImpl::new(self, run_signup)
	}
}

async fn run_signup<C: ClientSignup>(mut client: C) -> Result<(), C::Error> {
	let email = client.get_user_email().await?;
	let password = client.get_user_password().await?;
	let extra_data = client.get_user_extra_data().await?;

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

	client.send_signup_to_server(&signup_data).await?;

	client.finalise_signup().await?;
	Ok(())
}
