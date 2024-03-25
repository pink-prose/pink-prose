use super::{ Email, Password, Salt, SigninAttemptID };

pub struct SigninForm {
	pub email: Email,
	pub password: Password
}

pub struct SigninS1Request {
	pub email: Email
}

pub struct SigninS1Response {
	pub salt: Salt,
	pub signin_attempt_id: SigninAttemptID
}

pub enum SigninS1GetSalt {
	Verified(Salt),
	NotVerified,
	InvalidEmail
}

pub struct SigninS1InProgress {
	pub email: Email,
	pub signin_attempt_id: SigninAttemptID
}
