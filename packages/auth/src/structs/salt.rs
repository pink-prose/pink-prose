use ::rand::{ Rng, rngs::OsRng };

pub struct Salt([u8; 64]);

impl Salt {
	pub fn generate() -> Self {
		let mut salt = [0u8; 64];
		OsRng.fill(&mut salt);
		Self(salt)
	}

	pub fn as_bytes(&self) -> &[u8; 64] {
		&self.0
	}

	pub fn to_string(&self) -> String {
		::hex::encode(&self.0 as &[u8])
	}
}
