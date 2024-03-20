use ::blake3::Hasher;

pub struct Blake3([u8; 32]);

impl Blake3 {
	pub fn hash(bytes: &[u8]) -> Self {
		let mut hasher = Hasher::new();
		hasher.update(bytes);
		let bytes = *hasher.finalize().as_bytes();
		Self(bytes)
	}

	pub fn hash_key_derivation(context: &'static str, bytes: &[u8]) -> Self {
		let mut hasher = Hasher::new_derive_key(context);
		hasher.update(bytes);
		let bytes = *hasher.finalize().as_bytes();
		Self(bytes)
	}
}
