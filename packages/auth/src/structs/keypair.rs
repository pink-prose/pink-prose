use ::p384::{ PublicKey, SecretKey as PrivateKey };
use ::rand::rngs::OsRng;

pub struct Keypair {
	public_key: PublicKey,
	private_key: PrivateKey
}

impl Keypair {
	pub fn generate() -> Self {
		let private_key = PrivateKey::random(&mut OsRng);
		let public_key = private_key.public_key();
		Self { public_key, private_key }
	}
}
