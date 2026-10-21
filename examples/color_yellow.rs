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
stack_size! {0x100}

const COLOR: &str = "33mYellow";

fn main() {

    let mut console_writer = Console::writer();
	loop { 
		//let ms = Alarm::get_milliseconds().unwrap();
	    //let _ = writeln!(console_writer, "\x1b[0;{}!\t{}\x1b[0m", COLOR,ms);

	    let _ = writeln!(console_writer, "\x1b[0;{}!!\x1b[0m", COLOR);

		Alarm::sleep_for(Milliseconds(1200)).unwrap();
	}
}
