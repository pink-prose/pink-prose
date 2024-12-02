#[cfg(feature = "backend")]
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
	pink_prose_backend::run().await
}

#[cfg(not(feature = "backend"))]
fn main() {
	panic!("perhaps you forgot to turn on the `server` feature of `pink-prose-leptos`?");
}
