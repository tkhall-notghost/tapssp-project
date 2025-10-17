/*

Overtopr - Rust system monitor. Student project by Tessa Hall.
MIT License 2025

 */

use std::{thread, time::Duration};

mod system_base;
use crate::system_base::SystemBase;

fn refresh_and_print(base: &mut SystemBase) {
	// run a refresh, update all SystemBase values to reflect current system stats
	SystemBase::refresh(base);
	println!("Units and further formatting to come later...");
	println!("CPU avg: {}", base.get_cpu_avg());
	println!("Memory Available: {}", base.get_mem_avail());
	println!("Memory Used: {}", base.get_mem_used());
	println!("Memory Free: {}", base.get_mem_free());
	println!("Swap Used: {}", base.get_cpu_avg());
	println!("Network Interface Stats ------------");
	for iface in base.get_network_interfaces() {
			println!("interface: {} - MAC: {}", iface.name, iface.mac);
			println!("  tx bytes: {} - rx bytes: {}",iface.tx_bytes,iface.rx_bytes);
			for network in iface.networks {
					println!("  IP: {}",network);
			}
	}
}

fn main() {
	println!("overtopr:");
	// create a generic SystemBase which represents our gathered System information
	let mut base = SystemBase::new();
	loop {
		println!("Use Ctrl-C to exit at any time. Some metrics are refined by further refreshes.");
		// refresh system info and print it
		refresh_and_print(&mut base);
		// system stats refresh delay
		thread::sleep(Duration::from_secs(2));
		clearscreen::clear().expect("failed to clear screen");
	}
}
