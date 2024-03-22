#![allow(
	clippy::should_implement_trait,
	clippy::inherent_to_string
)]
#![allow(dead_code, unused_imports, unused_variables)]

// TODO: enable these
#[cfg(feature = "client")]
pub mod client;

// TODO: enable these
#[cfg(feature = "server")]
pub mod server;

mod error;
mod sealed_future;
pub mod structs;

pub use crate::error::Error;
