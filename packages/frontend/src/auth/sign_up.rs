// use ::anyhow::{ Error, Result };
// use ::pinkprose_auth::signup::*;
// use ::pinkprose_auth::structs::*;

// pub struct Signup {
// 	pub email: String,
// 	pub password: String
// }

// impl ClientSignup for Signup {
// 	type Error = Error;
//
// 	async fn get_signup_form(&mut self) -> Result<SignupForm> {
// 		let email = Email::from_str(&self.email)?;
// 		let password = Password::from_str(&self.email)?;
// 		Ok(SignupForm { email, password })
// 	}
//
// 	async fn submit_request(&mut self, signup_request: &SignupRequest) -> Result<SignupResponse> {
// 		todo!()
// 	}
//
// 	async fn finalise(self) -> Result<()> { Ok(()) }
// }
