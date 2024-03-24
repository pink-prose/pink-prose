use crate::error::*;
use super::{ Generatable, StructsCommon, z85_to_array };
use ::argon2::{ Algorithm, Version, ParamsBuilder };
use ::rand::{ Rng, rngs::OsRng };
use ::wiwi::z85::{ encode_z85, decode_z85 };

pub struct Argon2 {
	alg: Algorithm,
	version: Version,
	m_cost: u32,
	t_cost: u32,
	p_cost: u32,
	output_len: usize,
	bytes: [u8; Self::OUTPUT_LEN]
}

impl StructsCommon for Argon2 {
	fn to_string(&self) -> Result<String> {
		let Self {
			alg,
			version,
			m_cost,
			t_cost,
			p_cost,
			output_len,
			bytes
		} = self;

		let version = *version as u32;
		let bytes = encode_z85(bytes);

		// comma is not used in z85
		Ok(format!("{alg},{version},{m_cost},{t_cost},{p_cost},{output_len},{bytes}"))
	}

	fn from_str(s: &str) -> Result<Self> {
		let mut iter = s.split(',');

		/// am lazy, don't mind me
		macro_rules! try_next {
			() => { iter.next().ok_or_else(|| Error::ParseArgon2)? }
		}

		let alg = try_next!().parse()?;
		let version: u32 = try_next!().parse()?;
		let version = version.try_into()?;
		let m_cost = try_next!().parse()?;
		let t_cost = try_next!().parse()?;
		let p_cost = try_next!().parse()?;
		let output_len = try_next!().parse()?;
		let bytes = decode_z85(try_next!().as_bytes())?
			.try_into()
			.map_err(|_| Error::TryIntoArray)?;
		if iter.next().is_some() { return Err(Error::ParseArgon2) }

		Ok(Self {
			alg,
			version,
			m_cost,
			t_cost,
			p_cost,
			output_len,
			bytes
		})
	}
}

impl Argon2 {
	fn default_with_zeroed_bytes() -> Self {
		Self {
			alg: Self::ALG,
			version: Self::VERSION,
			m_cost: Self::M_COST,
			t_cost: Self::T_COST,
			p_cost: Self::P_COST,
			output_len: Self::OUTPUT_LEN,
			bytes: [0u8; Self::OUTPUT_LEN]
		}
	}

	fn get_hasher(&self) -> Result<::argon2::Argon2<'static>> {
		Ok(::argon2::Argon2::new(
			self.alg,
			self.version,
			ParamsBuilder::new()
				.m_cost(self.m_cost)
				.t_cost(self.t_cost)
				.p_cost(self.p_cost)
				.output_len(self.output_len)
				.build()?
		))
	}

	pub(super) fn hash_and_salt(
		to_hash: &[u8],
		salt: &Salt
	) -> Result<Self> {
		let mut argon2 = Self::default_with_zeroed_bytes();

		argon2.get_hasher()?.hash_password_into(
			to_hash,
			&salt.0,
			&mut argon2.bytes
		)?;

		Ok(argon2)
	}

	pub(super) fn to_hash_bytes(&self) -> &[u8; Self::OUTPUT_LEN] {
		&self.bytes
	}
}

impl Argon2 {
	const ALG: Algorithm = Algorithm::Argon2id;
	const VERSION: Version = Version::V0x13;
	const M_COST: u32 = 0x10000;
	const T_COST: u32 = 4;
	const P_COST: u32 = 2;
	const OUTPUT_LEN: usize = 32;
}

pub struct Salt([u8; 32]);

impl StructsCommon for Salt {
	fn to_string(&self) -> Result<String> {
		Ok(encode_z85(&self.0))
	}

	fn from_str(s: &str) -> Result<Self> {
		z85_to_array(s, Self)
	}
}

impl Generatable for Salt {
	fn generate() -> Self {
		let mut buf = [0u8; 32];
		OsRng.fill(&mut buf as &mut [u8]);
		Self(buf)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn roundtrip_salt_string() {
		let salt = Salt::generate();
		let string = salt.to_string().unwrap();
		let parsed_salt = Salt::from_str(&string).unwrap();
		assert_eq!(salt.0, parsed_salt.0);
	}
}
