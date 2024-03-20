pub struct Email(String);

impl Email {
	pub fn from_string(s: String) -> Self {
		Self(s)
	}

	pub fn into_string(self) -> String {
		self.0
	}
}
