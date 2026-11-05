//! An extremely simple libtock-rs example. Just prints out a few numbers using
//! the LowLevelDebug capsule then terminates.

#![no_main]
#![no_std]

use core::fmt::Write;
use libtock::console::Console;
use libtock::alarm::{Alarm, Milliseconds, Ticks};

//use libtock::low_level_debug::LowLevelDebug;
use libtock::runtime::{set_main, stack_size};

set_main! {main}
stack_size! {0x100}

fn milli(ms: u32) -> Ticks {
	Ticks(Alarm::get_frequency().unwrap().0 / 1000 * ms)
}

fn main() {

    let mut console_writer = Console::writer();
	loop { 
	    let _ = writeln!(console_writer, "\x1b[0;33mYellow! {}\x1b[0m", milli(1000).0);
		//Alarm::sleep_for(Ticks(100_000_000)).unwrap();		
		Alarm::sleep_for(milli(1000)).unwrap();		
	}
}
