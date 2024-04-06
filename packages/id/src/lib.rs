#![allow(dead_code, unused_imports, unused_mut, unused_variables)]
#![allow(
	clippy::new_without_default,
	clippy::should_implement_trait
)]

use ::rand::{ Rng, rngs::ThreadRng, thread_rng };
use ::std::time::{ SystemTime, UNIX_EPOCH };

/// 46 bits gives space for >2000years with millisecond precision.
/// 18 bits remaining after this
const TIMESTAMP_SHIFT: u64 = 64 - 46;

/// to make way for 4 lower bits for the random component. Keep count in the next
/// 14 bits after the timestamp. 14 bits gives 16384 IDs/ms
const COUNT_SHIFT: u64 = 4;

/// one more than 14 lower bits filled. count must be / will always be below
/// this value
const MAX_COUNT: u32 = 1 << 14;

/// keep only 4 lower bits of random byte.
const RANDOM_COMPONENT_MASK: u8 = 0b1111;

/// counting from the most significant to least significant bit, bits 1 to 64:
/// - (1-46) 46 bits for timestamp (this is >2000years with millisecond precision,
///   up to year 4199)
/// - (47-60) 14 bits for increment (this is max 16384 IDs/ms, or 16M IDs/s)
/// - (61-64) last 4 bits for randomness (so IDs within one ms aren't just increments)
/// IDs generated from one single factory are guaranteed to monotonically increase.
pub struct IDGen {
	/// unix epoch time
	last_generated_time: u64,
	// 19 bits fits in u32 (duh)
	count: u32,
	rng: ThreadRng
}

#[repr(transparent)]
pub struct GeneratedID {
	uint: u64
}

impl IDGen {
	pub fn new() -> Self {
		let last_generated_time = 0;
		let count = 0;
		let rng = thread_rng();
		Self { last_generated_time, count, rng }
	}

	pub fn next(&mut self) -> Option<GeneratedID> {
		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("we are before 01 jan 1970 lol?")
			.as_millis() as u64;

		if now > self.last_generated_time {
			self.last_generated_time = now;
			self.count = 0;
		}

		(self.count < MAX_COUNT).then(|| {
			let now = self.last_generated_time << TIMESTAMP_SHIFT;
			let random = (self.rng.gen::<u8>() & RANDOM_COMPONENT_MASK) as u64;

			// guaranteed to fit within 14 bits, as checked by
			// bool statement before this closure
			let increment = (self.count << COUNT_SHIFT) as u64;
			self.count += 1;

			let id = now | random | increment;
			GeneratedID { uint: id }
		})
	}
}

impl GeneratedID {
	#[inline]
	pub fn unix_time(&self) -> u64 {
		self.uint >> TIMESTAMP_SHIFT
	}

	#[inline]
	pub fn as_iint(&self) -> i64 {
		self.uint as i64
	}

	#[inline]
	pub fn as_uint(&self) -> u64 {
		self.uint
	}
}
