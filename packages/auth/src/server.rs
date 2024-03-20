use crate::sealed_future::*;
use ::std::future::{ Future, IntoFuture };

pub trait ServerSignup: Sized {
	type Error: From<crate::Error>;
	type ExtraData;

	// fn receive_signup_from_client(&mut self) -> impl Future<Output = Result<>>;

	fn run(self) -> impl SealedFuture<Result<(), Self::Error>> {
		SealedFutureImpl::new(self, run)
	}
}

async fn run<S: ServerSignup>(server: S) -> Result<(), S::Error> {
	todo!()
}
