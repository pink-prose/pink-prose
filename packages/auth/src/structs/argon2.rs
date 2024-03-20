use crate::error::*;
use crate::structs::{ Password, Salt };
use ::argon2::{ Algorithm, Version, ParamsBuilder };

pub struct Argon2 {
	m_cost: u32,
	t_cost: u32,
	p_cost: u32,
	output_len: usize,
	bytes: [u8; Self::OUT_BYTES]
}

impl Argon2 {
	const ARGON2_M_COST: u32 = 0x10000;
	const ARGON2_T_COST: u32 = 4;
	const ARGON2_P_COST: u32 = 2;
	const OUT_BYTES: usize = 32;

	fn new_hasher() -> Result<::argon2::Argon2<'static>> {
		Ok(::argon2::Argon2::new(
			Algorithm::Argon2id,
			Version::default(),
			ParamsBuilder::new()
				.m_cost(Self::ARGON2_M_COST)
				.t_cost(Self::ARGON2_T_COST)
				.p_cost(Self::ARGON2_P_COST)
				.build()?
		))
	}

	pub fn hash_and_salt(
		password: &[u8],
		salt: &Salt
	) -> Result<Self> {
		let hasher = Self::new_hasher()?;
		let mut bytes = [0u8; Self::OUT_BYTES];

		hasher.hash_password_into(
			password,
			salt.as_bytes(),
			&mut bytes
		)?;

		Ok(Self {
			m_cost: Self::ARGON2_M_COST,
			t_cost: Self::ARGON2_T_COST,
			p_cost: Self::ARGON2_P_COST,
			output_len: Self::OUT_BYTES,
			bytes
		})
	}

	pub fn as_bytes(&self) -> &[u8; Self::OUT_BYTES] {
		&self.bytes
	}

	pub fn to_string(&self) -> String {
		let hex = ::hex::encode(&self.bytes as &[u8]);
		let Self { m_cost, t_cost, p_cost, output_len, .. } = self;
		format!("{m_cost}-{t_cost}-{p_cost}-{output_len}-{hex}")
	}

	// pub fn from_str(s: &str) -> Result<Self> {}
}
