/*

Overtopr - Rust system monitor. Student project by Tessa Hall.
MIT License 2025

 */

// #![cfg(feature = "bracketed-paste")]

mod system_base;
use crate::system_base::SystemBase;

fn main() {
    println!("overtopr:");
		// create a generic SystemBase which represents our gathered System information
		let mut base = SystemBase::new();
		// run a refresh, update all SystemBase values to reflect current system stats
		SystemBase::refresh(&mut base);
		println!("Units and further formatting to come later...");
		println!("CPU avg: {}",base.get_cpu_avg());
		println!("Memory Available: {}",base.get_mem_avail());
		println!("Memory Used: {}",base.get_mem_used());
		println!("Memory Free: {}",base.get_mem_free());
		println!("Swap Used: {}",base.get_cpu_avg());
		println!("Network Interfaces: {}",base.get_network_interfaces());
}
