use ::thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("{0}")]
	Aead(::chacha20poly1305::aead::Error),
	#[error("{0}")]
	Argon2(::argon2::Error),
	#[error(transparent)]
	P384PKCS8SPKI(#[from] ::p384::pkcs8::spki::Error),
	#[error(transparent)]
	P384PKCS8(#[from] ::p384::pkcs8::Error),
}

impl From<::chacha20poly1305::aead::Error> for Error {
	fn from(error: ::chacha20poly1305::aead::Error) -> Self {
		Self::Aead(error)
	}
}

impl From<::argon2::Error> for Error {
	fn from(error: ::argon2::Error) -> Self {
		Self::Argon2(error)
	}
}
