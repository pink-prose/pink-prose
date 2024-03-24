use crate::error::*;
use ::wiwi::z85::{ encode_z85, decode_z85 };

pub trait StructsCommon: Sized {
	fn to_string(&self) -> Result<String>;
	fn from_str(s: &str) -> Result<Self>;
}

pub trait Generatable: Sized {
	fn generate() -> Self;
}

/// saves some boilerplate typing, nothing else
#[inline]
pub fn z85_to_array<T, F, const N: usize>(s: &str, f: F) -> Result<T>
where
	F: FnOnce([u8; N]) -> T
{
	// TODO: const array impl of z85 in wiwi crate?
	let decoded = decode_z85(s.as_bytes())?
		.try_into()
		.map_err(|_| Error::TryIntoArray)?;
	Ok(f(decoded))
}
