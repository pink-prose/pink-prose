use crate::error::*;
use super::{ PublicKey, PrivateKey, StructsCommon, Generatable };
use ::p384::ecdsa::Signature;
use ::rand::{ Rng as _, rngs::OsRng };

pub struct TextChallenge([u8; 64]);

impl StructsCommon for TextChallenge {
	fn to_string(&self) -> Result<String> {
		Ok(::wiwi::hex::encode_hex(&self.0 as &[u8]))
	}

	fn from_str(s: &str) -> Result<Self> {
		let decoded = ::wiwi::hex::decode_hex(s.as_bytes())?
			.try_into()
			.map_err(|_| Error::TryIntoArray)?;
		Ok(Self(decoded))
	}
}

impl Generatable for TextChallenge {
	fn generate() -> Self {
		let mut bytes = [0u8; 64];
		OsRng.fill(&mut bytes);
		Self(bytes)
	}
}

impl TextChallenge {
	pub(crate) fn sign(&self, private_key: &PrivateKey) -> Result<TextChallengeSignature> {
		Ok(TextChallengeSignature(private_key.sign_bytes(&self.0)?))
	}

	pub(crate) fn verify(&self, public_key: &PublicKey, signature: &TextChallengeSignature) -> bool {
		public_key.verify_signature(&self.0, &signature.0)
	}
}

pub struct TextChallengeSignature(Signature);

impl StructsCommon for TextChallengeSignature {
	fn to_string(&self) -> Result<String> {
		Ok(self.0.to_string())
	}

	fn from_str(s: &str) -> Result<Self> {
		Ok(Self(s.parse()?))
	}
}
