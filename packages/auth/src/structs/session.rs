use crate::internal_prelude::*;
use ::wiwi::z85::{ encode_z85, decode_z85 };

pub struct SessionID([u8; 32]);

// impl StructsCommon for SessionID {
// 	fn to_string(&self) -> Result<String> {
// 		Ok(encode_z85(&self.0))
// 	}

// 	fn from_str(s: &str) -> Result<Self> {
// 		z85_to_array(s, Self)
// 	}
// }

impl Generatable for SessionID {
	fn generate() -> Self {
		Self(rand_array())
	}
}
