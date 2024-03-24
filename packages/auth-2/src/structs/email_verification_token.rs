use crate::internal_prelude::*;
use ::rand::{ Rng, rngs::OsRng };
use ::wiwi::hex::{ encode_hex, decode_hex };

pub struct EmailVerificationToken([u8; 32]);

impl StructsCommon for EmailVerificationToken {
	fn to_string(&self) -> Result<String> {
		Ok(encode_hex(&self.0))
	}

	fn from_str(s: &str) -> Result<EmailVerificationToken> {
		let decoded = decode_hex(s.as_bytes())?
			.try_into()
			.map_err(|_| Error::TryIntoArray)?;
		Ok(Self(decoded))
	}
}

impl Generatable for EmailVerificationToken {
	fn generate() -> Self {
		Self(rand_array())
	}
}
