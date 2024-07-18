use catppuccin::{ Flavor, PALETTE };
use leptos::*;
use std::ops::{ Deref, DerefMut };

pub type ThemeContext = RwSignal<Theme>;

#[derive(Clone, Copy)]
pub struct Theme {
	inner: &'static Flavor
}

impl Default for Theme {
	fn default() -> Self {
		Self { inner: &PALETTE.mocha }
	}
}

pub fn provide_theme() {
	let context: ThemeContext = create_rw_signal(Theme::default());
	provide_context(context);
}

pub fn use_theme() -> ThemeContext {
	expect_context()
}

impl Deref for Theme {
	type Target = &'static Flavor;
	fn deref(&self) -> &&'static Flavor {
		&self.inner
	}
}

impl DerefMut for Theme {
	fn deref_mut(&mut self) -> &mut &'static Flavor {
		&mut self.inner
	}
}
