//! An extremely simple libtock-rs example. Just prints out a few numbers using
//! the LowLevelDebug capsule then terminates.

#![no_main]
#![no_std]

use core::fmt::Write;
use libtock::console::Console;
use libtock::alarm::{Alarm, Milliseconds};

//use libtock::low_level_debug::LowLevelDebug;
use libtock::runtime::{set_main, stack_size};

set_main! {main}
stack_size! {0x300}

const COLOR: &str = "34m"; //Blue
const DELAY: u32 = 500;

fn get_ms() -> i64 {
    Alarm::get_milliseconds().unwrap() as i64
}

fn main() {
    let mut console_writer = Console::writer();
	let mut ms = 0;
	loop { 
	    let _ = write!(console_writer, "\x1b[0;{}{}!\x1b[0m\t", COLOR, ms);
		let ms_start = get_ms();
		Alarm::sleep_for(Milliseconds(DELAY)).unwrap();		
		ms = get_ms() - ms_start - DELAY as i64;
	}
}
