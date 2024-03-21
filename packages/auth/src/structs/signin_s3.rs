use super::{ PublicKey, SessionID, SigninAttemptID, TextChallengeSignature };

pub struct SigninS3Request<ExtraData> {
	pub signin_attempt_id: SigninAttemptID,
	pub text_challenge_signature: TextChallengeSignature,
	pub session_public_key: PublicKey,
	pub extra_data: ExtraData
}

pub struct SigninS3Response {
	pub session_id: SessionID
}

pub struct SigninS3UserInfo {
	pub public_key: PublicKey
}
