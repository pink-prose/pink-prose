use ::thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("{0}")]
	Argon2(::argon2::Error)
}

impl From<::argon2::Error> for Error {
	fn from(error: ::argon2::Error) -> Self {
		Self::Argon2(error)
	}
}
