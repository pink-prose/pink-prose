use crate::sealed_future::*;
use crate::structs::{
	Email,
	EncryptedPrivateKey,
	HashedPasswordVerifier,
	Password,
	PasswordVerifier,
	PublicKey,
	Salt,
	SignupData,
	StoredSignupData
};
use ::std::future::{ Future, IntoFuture };

pub trait ServerSignup: Sized {
	type Error: From<crate::Error>;
	type ExtraData;

	// fn receive_signup_from_client(&mut self) -> impl Future<Output = Result<SignupData<Self::ExtraData>, Self::Error>>;
	// fn process_extra_data_pre(&mut self, data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>>;
	// fn ensure_email_unique_and_reserve(&mut self, email: &str) -> impl Future<Output = Result<(), Self::Error>>;
	// fn generate_email_verification_token(&mut self, email: &str) -> impl Future<Output = Result<String, Self::Error>>;

	// fn store_unverified_user_data(&mut self, data: &StoredSignupData) -> impl Future<Output = Result<(), Self::Error>>;
	// fn send_verification_email(&mut self, email: &str, email_verification_token: &str) -> impl Future<Output = Result<(), Self::Error>>;

	// fn process_extra_data_post(&mut self, data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>>;

	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
		SealedFutureImpl::new(self, run_signup)
	}
}

async fn run_signup<S: ServerSignup>(mut server: S) -> Result<(), S::Error> {
	// let SignupData {
	// 	email,
	// 	salt,
	// 	password_verifier,
	// 	public_key,
	// 	encrypted_private_key,
	// 	extra_data
	// } = server.receive_signup_from_client().await?;
	//
	// let email = Email::from_string(email);
	// let email = email.into_string();
	// let _salt = Salt::from_str(&salt)?;
	// let password_verifier = PasswordVerifier::from_str(&password_verifier)?;
	// let _public_key = PublicKey::from_str(&public_key)?;
	// let _encrypted_private_key = EncryptedPrivateKey::from_str(&encrypted_private_key)?;
	//
	// server.process_extra_data_pre(&extra_data).await?;
	// server.ensure_email_unique_and_reserve(email.as_str()).await?;
	//
	// let verifier_salt = Salt::generate();
	// let hashed_password_verifier = HashedPasswordVerifier::from_password_verifier_and_salt(&password_verifier, &_salt)?;
	//
	// let email_verification_token = server.generate_email_verification_token(email.as_str()).await?;
	//
	// let data = StoredSignupData {
	// 	email: email.clone(),
	// 	salt,
	// 	hashed_password_verifier: hashed_password_verifier.to_string(),
	// 	public_key,
	// 	encrypted_private_key,
	// 	email_verification_token: email_verification_token.clone()
	// };
	//
	// server.store_unverified_user_data(&data).await?;
	// server.send_verification_email(&email, &email_verification_token).await?;
	//
	// server.process_extra_data_post(&extra_data).await?;
	//
	// Ok(())

	todo!()
}
