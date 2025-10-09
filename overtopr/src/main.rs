/*

Overtopr - Rust system monitor. Student project by Tessa Hall.
MIT License 2025

 */

mod system_base;

use crate::system_base::SystemBase;

fn main() {
    println!("overtopr:");
		// create a generic SystemBase which represents our gathered System information
		let mut base = SystemBase::new();
		// run a refresh, update all SystemBase values to reflect current system stats
		SystemBase::refresh(&mut base);
}
