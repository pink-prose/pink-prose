use crate::internal_prelude::*;
use ::wiwi::z85::{ encode_z85, decode_z85 };

pub trait ArraySerialisable<const N: usize>: Sized {
	fn to_array(&self) -> Result<[u8; N]>;
	fn from_array(a: &[u8; N]) -> Result<Self>;

	#[inline]
	fn into_array(self) -> Result<[u8; N]> {
		self.to_array()
	}

	#[inline]
	fn from_owned_array(a: [u8; N]) -> Result<Self> {
		Self::from_array(&a)
	}
}

pub trait VecSerialisable: Sized {
	fn to_vec(&self) -> Result<Vec<u8>>;
	fn from_bytes(b: &[u8]) -> Result<Self>;

	#[inline]
	fn into_vec(self) -> Result<Vec<u8>> {
		self.to_vec()
	}

	#[inline]
	fn from_vec(vec: Vec<u8>) -> Result<Self> {
		Self::from_bytes(&vec)
	}
}

pub trait StringSerialisable: Sized {
	fn to_string(&self) -> Result<String>;
	fn from_str(s: &str) -> Result<Self>;

	#[inline]
	fn into_string(self) -> Result<String> {
		self.to_string()
	}

	#[inline]
	fn from_string(s: String) -> Result<Self> {
		Self::from_str(&s)
	}
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
