#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Surreal(#[from] ::surrealdb::Error)
}

pub type Result<T = ()> = ::std::result::Result<T, Error>;
