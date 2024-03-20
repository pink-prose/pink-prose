use crate::sealed_future::*;
use ::std::future::{ Future, IntoFuture };
use crate::structs::{ Keypair, PasswordKey, PasswordVerifier, Salt, UserEmail, UserPassword };

pub trait ClientSignup: Sized {
	type Error: From<crate::Error>;
	type ExtraData;

	fn get_user_email(&mut self) -> impl Future<Output = Result<String, Self::Error>>;
	fn get_user_password(&mut self) -> impl Future<Output = Result<String, Self::Error>>;
	fn get_user_extra_data(&mut self) -> impl Future<Output = Result<Self::ExtraData, Self::Error>>;
	// fn send_signup_to_server(&mut self) -> impl Future<Output = Result<String, Self::Error>>;

	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
		SealedFutureImpl::new(self, run)
	}
}

async fn run<C: ClientSignup>(mut client: C) -> Result<(), C::Error> {
	let user_email = UserEmail::from_string(client.get_user_email().await?);
	let user_password = UserPassword::from_string(client.get_user_password().await?);
	let extra_data = client.get_user_extra_data().await?;

	let keypair = Keypair::generate();
	let salt = Salt::generate();

	let password_key = PasswordKey::from_pw_and_salt(&user_password, &salt)?;
	let password_verifier = PasswordVerifier::from_password_key(&password_key);

	todo!()
}
