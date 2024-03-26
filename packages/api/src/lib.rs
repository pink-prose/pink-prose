// TODO: remove later when more complete
#![allow(dead_code, unused_imports, unused_variables)]

// pub mod auth;

mod internal_prelude {
	pub use ::bitcode::{ Decode, Encode };
	pub use ::serde::{ Deserialize, Serialize };
}
