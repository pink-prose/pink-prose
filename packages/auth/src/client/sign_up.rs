use ::std::future::Future;

/// Implement this for the clientside of the signup flow
pub trait ClientSignup: Sized {
	type Error;

	fn run_client_signup(self, _: private::NoOverriding) -> impl Future<Output = Result<(), Self::Error>> {
		async {
			todo!()
		}
	}
}

mod private {
	pub struct NoOverriding;
}
