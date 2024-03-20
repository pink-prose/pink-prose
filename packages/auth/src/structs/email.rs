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

impl Email {
	// pub fn from_string(s: String) -> Self {
	// 	Self(s)
	// }
	//
	// pub fn into_string(self) -> String {
	// 	self.0
	// }
	//
	// pub fn as_str(&self) -> &str {
	// 	&self.0
	// }
}
