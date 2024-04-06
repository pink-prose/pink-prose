use crate::error::*;
use super::StructsCommon;

pub struct Password(String);

impl StructsCommon for Password {
	fn to_string(&self) -> Result<String> {
		Ok(self.0.clone())
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(s.into()))
	}
}

impl Password {
	pub(crate) fn as_str(&self) -> &str {
		&self.0
	}

	pub(crate) fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}