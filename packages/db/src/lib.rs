use surrealdb::Surreal;
use surrealdb::engine::remote::ws as surreal_ws;
use surrealdb::opt::auth::Root;

pub struct Db {
	surreal: Surreal<surreal_ws::Client>
}

impl Db {
	pub async fn new(params: DbNewParams<'_>) -> Result<Self, Error> {
		let surreal = Surreal::new::<surreal_ws::Ws>(params.addr)
			.await?;

		surreal
			.signin(Root {
				username: "root",
				password: params.password
			})
			.await?;

		surreal
			.use_ns(params.ns)
			.use_db(params.db)
			.await?;

		Ok(Self { surreal })
	}
}

pub struct DbNewParams<'h> {
	addr: &'h str,
	password: &'h str,
	ns: &'h str,
	db: &'h str
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Surreal(#[from] surrealdb::Error)
}
