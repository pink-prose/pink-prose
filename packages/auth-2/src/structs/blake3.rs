use crate::internal_prelude::*;
use ::blake3::Hasher as Blake3Hasher;
use ::wiwi::z85::{ encode_z85, decode_z85 };

pub struct Blake3([u8; 32]);

// impl ArraySerialisation for Blake3 {
// 	const N: usize = 32;

// 	fn to_array(&self) -> Result<[u8; 32]> {
// 		Ok(encode_z85(&self.0))
// 	}

// 	fn from_array(a: &[u8; 32]) -> Result<Self> {
// 		Ok(a)
// 	}
// }

impl Blake3 {
	pub(crate) fn hash(bytes: &[u8]) -> Self {
		let mut hasher = Blake3Hasher::new();
		hasher.update(bytes);
		let bytes = *hasher.finalize().as_bytes();
		Self(bytes)
	}

	pub(crate) fn hash_key_derivation(
		// should be hardcoded
		context: &'static str,
		bytes: &[u8]
	) -> Self {
		let mut hasher = Blake3Hasher::new_derive_key(context);
		hasher.update(bytes);
		let bytes = *hasher.finalize().as_bytes();
		Self(bytes)
	}

	pub(crate) fn to_hash_bytes(&self) -> &[u8; 32] {
		&self.0
	}
}
