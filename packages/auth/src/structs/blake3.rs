use crate::error::*;
use super::StructsCommon;
use ::blake3::Hasher;

pub struct Blake3([u8; 32]);

impl StructsCommon for Blake3 {
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

impl Blake3 {
	// pub fn hash(bytes: &[u8]) -> Self {
	// 	let mut hasher = Hasher::new();
	// 	hasher.update(bytes);
	// 	let bytes = *hasher.finalize().as_bytes();
	// 	Self(bytes)
	// }
	//
	// pub fn hash_key_derivation(context: &'static str, bytes: &[u8]) -> Self {
	// 	let mut hasher = Hasher::new_derive_key(context);
	// 	hasher.update(bytes);
	// 	let bytes = *hasher.finalize().as_bytes();
	// 	Self(bytes)
	// }
	//
	// pub fn to_string(&self) -> String {
	// 	::hex::encode(&self.0 as &[u8])
	// }
	//
	// pub fn from_str(s: &str) -> Result<Self> {
	// 	// hmm.. a little weird to undecode a hash, but I guess it works lol
	// 	let decoded = ::hex::decode(s.as_bytes())?
	// 		.try_into()
	// 		.map_err(|_| Error::TryIntoArray)?;
	// 	Ok(Self(decoded))
	// }
	//
	// pub fn as_bytes(&self) -> &[u8; 32] {
	// 	&self.0
	// }
}
