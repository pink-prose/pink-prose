use crate::internal_prelude::*;
use ::wiwi::z85::{ encode_z85, decode_z85 };

pub struct SigningChallenge([u8; 256]);

// impl StructsCommon for SigningChallenge {
// 	fn to_string(&self) -> Result<String> {
// 		Ok(encode_z85(&self.0))
// 	}

// 	fn from_str(s: &str) -> Result<Self> {
// 		z85_to_array(s, Self)
// 	}
// }

impl Generatable for SigningChallenge {
	fn generate() -> Self {
		Self(rand_array())
	}
}
