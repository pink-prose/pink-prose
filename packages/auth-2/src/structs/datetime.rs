use crate::internal_prelude::*;
use ::chrono::{ Local, Utc };

pub struct UTCDateTime(::chrono::DateTime<Utc>);

impl UTCDateTime {
	pub(crate) fn now() -> Self {
		Self(Local::now().into())
	}
}
