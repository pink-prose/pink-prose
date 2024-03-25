use crate::internal_prelude::*;

pub struct Password(String);

impl StringSerialisable for Password {
	fn to_string(&self) -> Result<String> {
		Ok(self.0.clone())
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(s.into()))
	}
}

// impl VecSerialisable for Password {
// 	fn to_vec(&self) -> Result<Vec<u8>> {
// 		Ok(self.0.clone().into_bytes())
// 	}

// 	fn from_bytes(b: &[u8]) -> Result<Self> {
// 		Ok(Self(String::from_utf8(b.into())?))
// 	}
// }

impl Password {
	pub(crate) fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
