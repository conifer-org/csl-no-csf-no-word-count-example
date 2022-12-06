use std::any::Any;
use std::ops::Deref;
use std::slice;
use std::str;
use serde::{Serialize, Deserialize};

use postcard::{from_bytes, to_allocvec};

/// This is a Boot Electron. Call it as Boot electron to see what happens!
/// THis is CSL generated
#[no_mangle]
#[cfg(feature = "decorate_string_atom1")]
pub extern "C" fn boot_enjoy_decorating(_: i32, _: i32) -> i32 {

	enjoy_decorating();

	// Boot Electron returning `0` indicates that Everything is alright
	return 0;
}

/// Let's enjoy doing some decorations
/// THis is App-dev defined
#[cfg(feature = "decorate_string_atom1")]
fn enjoy_decorating() {
	let input = String::from("Tarun likes doing this");
	let name = format!("( ͡° ͜ʖ ͡°)_/¯ {input} ( ͡° ͜ʖ ͡°)_/¯");
	for i in 0..200 {
		csl::print(&name);
	}
}
