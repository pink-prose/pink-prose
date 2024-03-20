#![allow(
	clippy::should_implement_trait,
	clippy::inherent_to_string
)]
#![allow(dead_code, unused_imports, unused_variables)]

#[cfg(feature = "client")]
mod client;

#[cfg(feature = "server")]
mod server;

mod error;
mod sealed_future;
pub mod structs;

pub use crate::error::Error;
