use ::serde::Serialize;

#[derive(Serialize)]
pub struct UserCreate<'h> {
	pub email: &'h str,
	pub username: &'h str
}
