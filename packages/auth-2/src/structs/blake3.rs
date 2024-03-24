use crate::error::*;
use super::{ StructsCommon, z85_to_array };
use ::blake3::Hasher as Blake3Hasher;
use ::wiwi::z85::{ encode_z85, decode_z85 };

pub struct Blake3([u8; 32]);

impl StructsCommon for Blake3 {
	fn to_string(&self) -> Result<String> {
		Ok(encode_z85(&self.0))
	}

	fn from_str(s: &str) -> Result<Self> {
		z85_to_array(s, Self)
	}
}

impl Blake3 {
	pub(super) fn hash(bytes: &[u8]) -> Self {
		let mut hasher = Blake3Hasher::new();
		hasher.update(bytes);
		let bytes = *hasher.finalize().as_bytes();
		Self(bytes)
	}

	pub(super) fn hash_key_derivation(
		// should be hardcoded
		context: &'static str,
		bytes: &[u8]
	) -> Self {
		let mut hasher = Blake3Hasher::new_derive_key(context);
		hasher.update(bytes);
		let bytes = *hasher.finalize().as_bytes();
		Self(bytes)
	}
}
