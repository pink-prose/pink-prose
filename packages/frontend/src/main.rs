#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
	pink_prose_frontend::ssr_main().await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
	panic!("maybe you wanted to have `ssr` feature enabled to run main?");
}
