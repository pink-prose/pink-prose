use ::std::future::{ IntoFuture, Future };

// to make this work, this is relying on two conditions:
// 1. SealedFuture is sealed, so not implementable outside the crate
// 2. SealedFutureImpl, the only implementor, is not constructable outside the crate either

/// See docs on [`SealedFuture`] for more info
pub struct SealedFutureImpl<F> {
	/// future
	future: F
}

impl<F> SealedFutureImpl<F>
where
	F: Future
{
	#[inline]
	pub(crate) fn new(future: F) -> Self {
		Self { future }
	}
}

impl<F> IntoFuture for SealedFutureImpl<F>
where
	F: Future
{
	type Output = F::Output;
	type IntoFuture = F;

	#[inline]
	fn into_future(self) -> Self::IntoFuture {
		self.future
	}
}

/// This trait was made to hopefully avoid the need to type that function generic
/// in [`SealedFutureImpl`], and it seems to work lol?
///
/// This is a type that is unconstructable by downstream code. It also implements
/// [`IntoFuture`], so you can just `.await` the function as if it were an async fn.
/// This is used as a return value for several trait functions in this crate to
/// ensure they cannot be overridden by downstream users, forcing downstream
/// code to rely on the default impl.
pub trait SealedFuture<T>: IntoFuture<Output = T> + private::Sealed {}

impl<F> private::Sealed for SealedFutureImpl<F>
where
	F: Future
{}

impl<F> SealedFuture<F::Output> for SealedFutureImpl<F>
where
	F: Future
{}

mod private {
	pub trait Sealed {}
}
