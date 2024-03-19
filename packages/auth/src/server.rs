use ::std::future::Future;

/// Implement this for the serverside of the signup flow
pub trait ServerSignup: Sized {
	type Error;

	fn run_server_signup(self, _: private::NoOverriding) -> impl Future<Output = Result<(), Self::Error>> {
		async move {
			todo!()
		}
	}
}

mod private {
	pub struct NoOverriding;
}
