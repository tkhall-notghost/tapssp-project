/*

Overtopr - Rust system monitor. Student project by Tessa Hall.
MIT License 2025

 */

use std::{thread, time::Duration};

mod system_base;
use crate::system_base::SystemBase;

// this actually prints and then refreshes, to avoid waiting to print
// while fetching system stats after the screen has just been cleared
// which causes an annoying flashing effect
fn refresh_and_print(base: &mut SystemBase) {
	// this printing is all still very simple and I plan on changing it.
	println!("CPU Stats --------------------------------------------------------------overtop");
	println!("CPU avg: {}%", base.get_cpu_avg().round());
	println!("CPU Cores information: [number - frequency - utilization]");
	let mut brand = String::new();
	let mut i:u32 = 0;
	for c in base.get_cores() {
			brand = c.brand.clone();
			print!("[ {} - {}", c.name, c.freq);
			print!(" - {}% ] ", c.usage.round());
			if i!=0 && ((i%3) == 0) {
				println!("");
			}
			i+=1;
	}
		println!("  brand: {} - {} cores",brand,i);
	println!("RAM and Swap Stats --------------------------------------------------------------");
	println!("Memory Available: {} Memory Used: {} Memory Free: {}", base.get_mem_avail(), base.get_mem_used(),
		base.get_mem_free());
	println!("Swap Used: {}%", base.get_cpu_avg().round());
	println!("Network Interface Stats ---------------------------------------------------------");
	let mut ifaces = base.get_network_interfaces().clone();
	ifaces.sort_by(|a,b| b.name.cmp(&a.name));
		for iface in ifaces {
			println!("");
			println!("interface: {} - {}", iface.name, iface.mac);
			println!("  tx bytes: {} - rx bytes: {} ",iface.tx_bytes, iface.rx_bytes);
			let mut networks = iface.networks.clone();
			networks.sort();
			for network in networks {
					print!("  IP: {},",network);
			}
	}
	// end of output
	println!("");
	println!("---------------Ctrl-C to exit-----------");
	// run a refresh, update all SystemBase values to reflect current system stats
	SystemBase::refresh(base);
}

fn main() {
	// create a generic SystemBase which represents our gathered System information
	let mut base = SystemBase::new();
	loop {
		// refresh system info and print it
		refresh_and_print(&mut base);
		// system stats refresh delay
		thread::sleep(Duration::from_secs(2));
		clearscreen::clear().expect("failed to clear screen");
	}
}
