use super::{ Email, Salt, SigninAttemptID };

pub struct SigninS1Request {
	pub email: Email
}

pub struct SigninS1Response {
	pub salt: Salt,
	pub signin_attempt_id: SigninAttemptID
}

pub struct SigninS1InProgress {
	pub email: Email,
	pub signin_attempt_id: SigninAttemptID
}
