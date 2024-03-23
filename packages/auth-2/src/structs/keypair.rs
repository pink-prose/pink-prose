use crate::error::*;
use super::{ Generatable, StructsCommon };
use ::p384::{
	PublicKey as P384PublicKey,
	SecretKey as P384SecretKey
};
use ::p384::ecdsa::{
	SigningKey as P384SigningKey,
	VerifyingKey as P384VerifyingKey,
	Signature as P384Signature
};
use ::p384::ecdsa::signature::{ RandomizedSigner, Verifier };
use ::rand::{ Rng, rngs::OsRng };
use ::wiwi::z85::{ encode_z85, decode_z85 };
use ::zeroize::{ Zeroize, Zeroizing };

pub struct PublicKey(P384PublicKey);

impl StructsCommon for PublicKey {
	fn to_string(&self) -> Result<String> {
		let bytes = self.0.to_sec1_bytes();
		Ok(encode_z85(&bytes))
	}

	fn from_str(s: &str) -> Result<Self> {
		let decoded = decode_z85(s.as_bytes())?;
		let key = P384PublicKey::from_sec1_bytes(&decoded)?;
		Ok(Self(key))
	}
}

impl PublicKey {
	pub(super) fn verify(&self, msg: &[u8], signature: &Signature) -> bool {
		let verifying_key = P384VerifyingKey::from(&self.0);
		verifying_key.verify(msg, &signature.0).is_ok()
	}
}

pub struct SecretKey(P384SecretKey);

impl StructsCommon for SecretKey {
	fn to_string(&self) -> Result<String> {
		let bytes = self.0.to_sec1_der()?;
		Ok(encode_z85(&bytes))
	}

	fn from_str(s: &str) -> Result<Self> {
		let decoded = decode_z85(s.as_bytes())?;
		let key = P384SecretKey::from_sec1_der(&decoded)?;
		Ok(Self(key))
	}
}

impl SecretKey {
	pub(super) fn sign(&self, msg: &[u8]) -> Signature {
		let signing_key = P384SigningKey::from(&self.0);
		let signature = signing_key.sign_with_rng(&mut OsRng, msg);
		Signature(signature)
	}
}

pub struct Keypair {
	pub public_key: PublicKey,
	pub secret_key: SecretKey
}

impl Generatable for Keypair {
	fn generate() -> Self {
		let p384_secret_key = P384SecretKey::random(&mut OsRng);
		let p384_public_key = p384_secret_key.public_key();

		let public_key = PublicKey(p384_public_key);
		let secret_key = SecretKey(p384_secret_key);

		Self { public_key, secret_key }
	}
}

pub struct Signature(P384Signature);

impl StructsCommon for Signature {
	fn to_string(&self) -> Result<String> {
		Ok(self.0.to_string())
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(s.parse()?))
	}
}
