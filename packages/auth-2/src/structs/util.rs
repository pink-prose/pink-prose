use crate::internal_prelude::*;
use ::wiwi::z85::{ encode_z85, decode_z85 };

pub trait ArraySerialisable<const N: usize>: Sized {
	fn to_array(&self) -> Result<[u8; N]>;
	fn from_array(a: &[u8; N]) -> Result<Self>;
}

pub trait VecSerialisable: Sized {
	fn to_vec(&self) -> Result<Vec<u8>>;
	fn from_bytes(b: &[u8]) -> Result<Self>;
}

pub trait StringSerialisable: Sized {
	fn to_string(&self) -> Result<String>;
	fn from_str(s: &str) -> Result<Self>;
}

pub trait Generatable: Sized {
	fn generate() -> Self;
}

#[inline]
pub fn rand_array<const N: usize>() -> [u8; N] {
	use ::rand::{ Rng, rngs::OsRng };
	let mut buf = [0u8; N];
	OsRng.fill(&mut buf as &mut [u8]);
	buf
}
