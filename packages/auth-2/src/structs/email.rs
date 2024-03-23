use crate::error::*;
use super::StructsCommon;
use ::validator::ValidateEmail as _;

pub struct Email(String);

impl StructsCommon for Email {
	fn to_string(&self) -> Result<String> {
		Ok(self.0.clone())
	}

	fn from_str(s: &str) -> Result<Self> {
		if !s.validate_email() { return Err(Error::InvalidEmail(s.into())) }
		Ok(Self(s.into()))
	}
}
