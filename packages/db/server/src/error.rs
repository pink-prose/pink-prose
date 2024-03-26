#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Surreal(#[from] ::surrealdb::Error)
}

pub type Result<T = (), E = Error> = ::std::result::Result<T, E>;
