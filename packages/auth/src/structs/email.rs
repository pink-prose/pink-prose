use crate::internal_prelude::*;
use ::validator::ValidateEmail as _;

pub struct Email(String);

impl StringSerialisable for Email {
	fn to_string(&self) -> Result<String> {
		Ok(self.0.clone())
	}

	fn from_str(s: &str) -> Result<Self> {
		if !s.validate_email() { return Err(Error::InvalidEmail(s.into())) }
		Ok(Self(s.into()))
	}
}

// impl VecSerialisable for Email {
// 	fn to_vec(&self) -> Result<Vec<u8>> {
// 		Ok(self.0.as_bytes().into())
// 	}

// 	fn from_bytes(b: &[u8]) -> Result<Self> {
// 		Ok(Self(String::from_utf8(b.into())?))
// 	}
// }
