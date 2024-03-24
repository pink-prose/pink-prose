// Types have methods to serialise/deserialise themselves to/from string, and
// have methods to make _themself_ from others. Ex. `from*` methods, but not `to*`
// methods. Don't expose more than needed outside this module (pub(super)) or
// this crate (pub(crate)).

pub mod argon2;
pub use argon2::*;

pub mod blake3;
pub use blake3::*;

pub mod chacha20poly1305;
pub use chacha20poly1305::*;

pub mod email;
pub use email::*;

pub mod email_verification_token;
pub use email_verification_token::*;

pub mod hashed_password_verifier;
pub use hashed_password_verifier::*;

pub mod keypair;
pub use keypair::*;

pub mod password;
pub use password::*;

pub mod password_key;
pub use password_key::*;

pub mod password_reset_token;
pub use password_reset_token::*;

pub mod password_verifier;
pub use password_verifier::*;

pub mod request_verification_email;
pub use request_verification_email::*;

pub mod session;
pub use session::*;

pub mod signin_attempt_id;
pub use signin_attempt_id::*;

pub mod signing_challenge;
pub use signing_challenge::*;

mod util;
pub use self::util::{ Generatable, StructsCommon };
use util::*;
