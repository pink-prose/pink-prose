use parking_lot::Mutex;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws as surreal_ws;
use surrealdb::opt::auth::Root;
use wiwi::id::IDGenerator;

pub struct Db {
	surreal: Surreal<surreal_ws::Client>,
	id_gen: Mutex<IDGenerator>
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

		let id_gen = Mutex::new(IDGenerator::new());

		let this = Self { surreal, id_gen };
		this.run_init().await?;

		Ok(this)
	}

	async fn run_init(&self) -> Result<(), Error> {
		self.surreal
			.query(query!("init"))
			.await?
			.check()?;
		Ok(())
	}
}

pub struct DbNewParams<'h> {
	pub addr: &'h str,
	pub password: &'h str,
	pub ns: &'h str,
	pub db: &'h str
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Surreal(#[from] surrealdb::Error)
}

macro_rules! query {
	($path:literal) => {
		include_str!(concat!("../queries/", $path, ".surql"))
	}
}
use query;
