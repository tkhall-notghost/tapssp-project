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
	println!("Units and further formatting to come later...");
	println!("----------------------------------------");
	println!("---------------overtopr-----------------");
	println!("CPU Stats ------------------------------");
	println!("CPU avg: {}", base.get_cpu_avg());
	println!("RAM and Swap Stats ---------------------");
	println!("Memory Available: {}", base.get_mem_avail());
	println!("Memory Used: {}", base.get_mem_used());
	println!("Memory Free: {}", base.get_mem_free());
	println!("Swap Used: {}", base.get_cpu_avg());
	println!("Network Interface Stats ----------------");
	let mut ifaces = base.get_network_interfaces().clone();
	ifaces.sort_by(|a,b| b.name.cmp(&a.name));
	for iface in ifaces {
			println!("interface: {} - {}", iface.name, iface.mac);
			println!("  tx bytes: {}",iface.tx_bytes);
			println!("  rx bytes: {}",iface.rx_bytes);
			let mut networks = iface.networks.clone();
			networks.sort();
			for network in networks {
					println!("  IP: {}",network);
			}
	}
	// end of output
	println!("----------------------------------------");
	// run a refresh, update all SystemBase values to reflect current system stats
	SystemBase::refresh(base);
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
