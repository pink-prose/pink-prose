#![allow(
	clippy::should_implement_trait,
	clippy::inherent_to_string
)]
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;
pub mod client;
pub mod server;
pub mod shared_structs;
mod util;

pub use self::error::*;
