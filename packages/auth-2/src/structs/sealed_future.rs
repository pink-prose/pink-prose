use ::std::future::{ IntoFuture, Future };

// to make this work, this is relying on two conditions:
// 1. SealedFuture is sealed, so not implementable outside the crate
// 2. SealedFutureImpl, the only implementor, is not constructable outside the crate either

/// See docs on [`SealedFuture`] for more info
pub struct SealedFutureImpl<V, F> {
	/// the single param to the function
	value: V,
	/// Function that returns future
	function: F
}

impl<V, F, Fu> SealedFutureImpl<V, F>
where
	F: FnOnce(V) -> Fu,
	Fu: Future
{
	#[inline]
	pub(crate) fn new(value: V, function: F) -> Self {
		Self { value, function }
	}
}

impl<V, F, Fu> IntoFuture for SealedFutureImpl<V, F>
where
	F: FnOnce(V) -> Fu,
	Fu: Future
{
	type Output = Fu::Output;
	type IntoFuture = Fu;

	#[inline]
	fn into_future(self) -> Self::IntoFuture {
		(self.function)(self.value)
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

impl<V, F, Fu> private::Sealed for SealedFutureImpl<V, F>
where
	F: FnOnce(V) -> Fu,
	Fu: Future
{}

impl<V, F, Fu> SealedFuture<Fu::Output> for SealedFutureImpl<V, F>
where
	F: FnOnce(V) -> Fu,
	Fu: Future
{}

mod private {
	pub trait Sealed {}
}
