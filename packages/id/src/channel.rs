use crate::{ IDGenerator, GeneratedID };
use ::crossbeam_channel::{ bounded, Receiver, TrySendError };
use ::std::thread::spawn;
use ::std::time::Duration;
use ::spin_sleep::SpinSleeper;

pub fn spawn_generator_thread() -> impl Fn() -> GeneratedID + Clone {
	let sleeper = SpinSleeper::default();
	let sleep_duration = Duration::from_micros(250);
	let (sender, receiver) = bounded(0);

	spawn(move || {
		let mut idgen = IDGenerator::new();

		loop {
			match idgen.next() {
				Some(id) => match sender.try_send(id) {
					// a receiver was there to accept the ID
					// nothing to do here
					Ok(_) => {}

					// no receiver to receive the ID
					Err(TrySendError::Full(_)) => { sleeper.sleep(sleep_duration) }

					// all receivers have dropped, this generator is now
					// inaccessible, break the loop to stop this thread
					Err(TrySendError::Disconnected(_)) => { break }
				}

				// exhausted all IDs for the current millisecond
				None => { sleeper.sleep(sleep_duration) }
			}
		}
	});

	#[inline]
	move || unsafe {
		// SAFETY: this will only error if the channel is disconnected. We
		// won't disconnect from sender side, the sender will only stop when
		// all receivers are dropped. Therefore this will never return an error.
		receiver.recv().unwrap_unchecked()
	}
}
