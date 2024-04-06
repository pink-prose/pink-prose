use crate::error::*;
use super::{ StructsCommon, Generatable };
use ::rand::{ Rng, rngs::OsRng };

pub struct PasswordResetToken([u8; 32]);

impl StructsCommon for PasswordResetToken {
	fn to_string(&self) -> Result<String> {
		Ok(::wiwi::hex::encode_hex(&self.0 as &[u8]))
	}

	fn from_str(s: &str) -> Result<Self> {
		let decoded = ::wiwi::hex::decode_hex(s.as_bytes())?
			.try_into()
			.map_err(|_| Error::TryIntoArray)?;
		Ok(Self(decoded))
	}
}

impl Generatable for PasswordResetToken {
	fn generate() -> Self {
		let mut bytes = [0u8; 32];
		OsRng.fill(&mut bytes);
		Self(bytes)
	}
}
