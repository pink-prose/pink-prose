use super::{ Email, Salt, SigninAttemptID };

pub struct SigninS1Form<ExtraData> {
	pub email: Email,
	pub extra_data: ExtraData
}

pub struct SigninS1Request<ExtraData> {
	pub email: Email,
	pub extra_data: ExtraData
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
