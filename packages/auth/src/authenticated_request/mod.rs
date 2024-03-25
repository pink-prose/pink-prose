pub mod client;
pub mod server;

use crate::internal_prelude::*;

use super::signin::{ SessionClientInfo, SessionServerInfo };

pub struct AuthenticatedRequest {
	session_id: SessionID,
	body_signature: Signature,
	body: Vec<u8>
}

pub struct AuthenticatedResponse {}
