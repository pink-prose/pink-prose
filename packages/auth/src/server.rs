use crate::sealed_future::*;
use crate::structs::{
	Email,
	EmailVerificationToken,
	EncryptedPrivateKey,
	Generatable as _,
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

	fn receive_signup_from_client(&mut self) -> impl Future<Output = Result<SignupData<Self::ExtraData>, Self::Error>>;
	fn process_extra_data_pre(&mut self, data: &Self::ExtraData) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}
	fn ensure_email_unique_and_reserve(&mut self, email: &str) -> impl Future<Output = Result<(), Self::Error>>;
	fn generate_email_verification_token(&mut self, email: &str) -> impl Future<Output = Result<EmailVerificationToken, Self::Error>>;
	fn store_unverified_user_data(&mut self, data: &StoredSignupData<Self::ExtraData>) -> impl Future<Output = Result<(), Self::Error>>;
	fn send_verification(&mut self, email: &Email, email_verification_token: &EmailVerificationToken) -> impl Future<Output = Result<(), Self::Error>>;
	fn finalise_signup(&mut self) -> impl Future<Output = Result<(), Self::Error>> {
		async { Ok(()) }
	}

	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
		SealedFutureImpl::new(self, run_signup)
	}
}

async fn run_signup<S: ServerSignup>(mut server: S) -> Result<(), S::Error> {
	let SignupData {
		email,
		salt,
		password_verifier,
		public_key,
		encrypted_private_key,
		extra_data
	} = server.receive_signup_from_client().await?;

	server.process_extra_data_pre(&extra_data).await?;
	server.ensure_email_unique_and_reserve(email.as_str()).await?;

	let verifier_salt = Salt::generate();
	let hashed_password_verifier = HashedPasswordVerifier::from_password_verifier_and_salt(&password_verifier, &salt)?;

	let email_verification_token = server.generate_email_verification_token(email.as_str()).await?;

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

	server.send_verification(&data.email, &data.email_verification_token).await?;

	server.finalise_signup().await?;
	Ok(())
}
