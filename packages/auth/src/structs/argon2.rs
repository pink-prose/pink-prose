use crate::error::*;
use crate::structs::{ Password, Salt };
use ::argon2::{ Algorithm, Version, ParamsBuilder };

pub struct Argon2 {
	alg: Algorithm,
	m_cost: u32,
	t_cost: u32,
	p_cost: u32,
	output_len: usize,
	bytes: [u8; Self::OUT_BYTES]
}

impl Argon2 {
	// const ALG: Algorithm = Algorithm::Argon2id;
	// const ARGON2_M_COST: u32 = 0x10000;
	// const ARGON2_T_COST: u32 = 4;
	// const ARGON2_P_COST: u32 = 2;
	const OUT_BYTES: usize = 32;
	//
	// fn _new_hasher() -> Result<::argon2::Argon2<'static>> {
	// 	Ok(::argon2::Argon2::new(
	// 		Self::ALG,
	// 		Version::default(),
	// 		ParamsBuilder::new()
	// 			.m_cost(Self::ARGON2_M_COST)
	// 			.t_cost(Self::ARGON2_T_COST)
	// 			.p_cost(Self::ARGON2_P_COST)
	// 			.build()?
	// 	))
	// }
	//
	// pub fn hash_and_salt(
	// 	to_hash: &[u8],
	// 	salt: &Salt
	// ) -> Result<Self> {
	// 	let hasher = Self::_new_hasher()?;
	// 	let mut bytes = [0u8; Self::OUT_BYTES];
	//
	// 	hasher.hash_password_into(
	// 		to_hash,
	// 		salt.as_bytes(),
	// 		&mut bytes
	// 	)?;
	//
	// 	Ok(Self {
	// 		alg: Self::ALG,
	// 		m_cost: Self::ARGON2_M_COST,
	// 		t_cost: Self::ARGON2_T_COST,
	// 		p_cost: Self::ARGON2_P_COST,
	// 		output_len: Self::OUT_BYTES,
	// 		bytes
	// 	})
	// }
	//
	// pub fn as_bytes(&self) -> &[u8; Self::OUT_BYTES] {
	// 	&self.bytes
	// }
	//
	// pub fn to_string(&self) -> String {
	// 	let hex = ::hex::encode(&self.bytes as &[u8]);
	// 	let Self { alg, m_cost, t_cost, p_cost, output_len, bytes: _ } = self;
	// 	format!("{alg}-{m_cost}-{t_cost}-{p_cost}-{output_len}-{hex}")
	// }
	//
	// // pub fn from_str(s: &str) -> Result<Self> {}
}
