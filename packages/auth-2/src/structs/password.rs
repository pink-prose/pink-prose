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
