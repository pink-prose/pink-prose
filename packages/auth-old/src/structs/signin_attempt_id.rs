use crate::error::*;
use super::{ StructsCommon, Generatable };
use ::rand::{ Rng, rngs::OsRng };

pub struct SigninAttemptID([u8; 32]);

impl StructsCommon for SigninAttemptID {
	fn to_string(&self) -> Result<String> {
		Ok(::hex::encode(&self.0 as &[u8]))
	}

	fn from_str(s: &str) -> Result<Self> {
		let decoded = ::hex::decode(s.as_bytes())?
			.try_into()
			.map_err(|_| Error::TryIntoArray)?;
		Ok(Self(decoded))
	}
}

impl Generatable for SigninAttemptID {
	fn generate() -> Self {
		let mut salt = [0u8; 32];
		OsRng.fill(&mut salt);
		Self(salt)
	}
}
