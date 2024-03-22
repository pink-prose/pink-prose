use super::Email;

pub struct VerificationEmailForm<ExtraData> {
	pub email: Email,
	pub extra_data: ExtraData
}

pub struct VerificationEmailRequest<ExtraData> {
	pub email: Email,
	pub extra_data: ExtraData
}
