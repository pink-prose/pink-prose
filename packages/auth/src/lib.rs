// TODO: remove later
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;
pub use self::error::*;
pub mod structs;

// TODO: see signup mod for comment. that's how I wanna do it ~vt

// TODO: needs client/server feature gate
pub mod signup;

// TODO: needs client/server feature gate
pub mod signin;

// TODO: needs client/server feature gate
// pub mod verification_email;

// TODO: needs client/server feature gate
// pub mod verification_email_triggered;

// TODO: needs client/server feature gate
// pub mod password_reset_email;

// TODO: needs client/server feature gate
// pub mod password_reset_email_triggered;

// TODO: needs client/server feature gate
pub mod authenticated_request;

/// lazy way of writing `impl Future<Output = Result<..., Self::Error>>`,
/// got sick of it, typed it waaaaaaaaay too many times
macro_rules! fut {
	($($stuff:tt)*) => { impl ::std::future::Future<Output = Result<$($stuff)*, Self::Error>> }
}
use fut;

/// Sealed future impl
macro_rules! sealed_fut {
	($($stuff:tt)*) => { impl $crate::structs::SealedFuture<Result<$($stuff)*, Self::Error>> }
}
use sealed_fut;

macro_rules! seal {
	($v:ident, $f:expr) => { $crate::structs::SealedFutureImpl::new($v, $f) }
}
use seal;

mod internal_prelude {
	pub(crate) use crate::error::*;
	pub(crate) use crate::structs::*;
	pub(crate) use crate::{ fut, seal, sealed_fut };
}
