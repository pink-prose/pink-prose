pub struct UserEmail(String);

impl UserEmail {
	pub fn from_string(s: String) -> Self {
		Self(s)
	}
}
