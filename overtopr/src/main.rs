/*

Overtopr - Rust system monitor. Student project by Tessa Hall.
MIT License 2025

 */
use ctrlc;
use std::{thread, time::Duration};

mod system_base;
use crate::system_base::SystemBase;

use std::sync::atomic::{AtomicBool, Ordering};

// this actually prints and then refreshes, to avoid waiting to print
// while fetching system stats after the screen has just been cleared
// which causes an annoying flashing effect
fn refresh_and_print(base: &mut SystemBase) {
	// this printing is all still very simple and I plan on changing it.
	println!("CPU Stats --------------------------------------------------------------overtopr");
	println!("CPU avg: {}%", base.get_cpu_avg().round());
	println!("CPU Cores information: [number - frequency - utilization]");
	let mut brand = String::new();
	let mut i: u32 = 0;
	for c in base.get_cores() {
		brand = c.brand.clone();
		print!("[ {} - {}", c.name, c.freq);
		print!(" - {}% ] ", c.usage.round());
		if i != 0 && ((i % 3) == 0) {
			println!("");
		}
		i += 1;
	}
	println!("  brand: {} - {} cores", brand, i);
	println!("RAM and Swap Stats --------------------------------------------------------------");
	println!(
		"Memory Available: {} Memory Used: {} Memory Free: {}",
		base.get_mem_avail().1,
		base.get_mem_used().1,
		base.get_mem_free().1
	);
	println!("Swap Used: {}%", base.get_swap_used());
	println!("Thermal Stats ---------------------------------------------------------");
	let mut thermalstats = base.get_comp_temps().clone();
	thermalstats.sort_by(|a, b| b.0.cmp(&a.0));
	for (component_string, celsius) in thermalstats {
		println!("{} - {:.1} celsius", component_string, celsius);
	}
	println!("Disk Stats ---------------------------------------------------------");
	println!("Name - filesystem - mountpoint ");
	println!(" available / total");
	println!(" live usage stats: read/write");
	for disk in base.get_disks() {
			println!("{} - {} - {} ", disk.name, disk.fs, disk.mnt);
			println!("  {} / {} total", disk.avail.1, disk.total.1);
			println!("  r:{} / w:{}", disk.read.1, disk.written.1);
	}
	println!("Network Interface Stats ---------------------------------------------------------");
	let mut ifaces = base.get_network_interfaces().clone();
	ifaces.sort_by(|a, b| b.name.cmp(&a.name));
	for iface in ifaces {
		println!("");
		println!("interface: {} - {}", iface.name, iface.mac);
		println!(
			"  tx: {} - rx: {} ",
			iface.tx_bytes.1, iface.rx_bytes.1
		);
		let mut networks = iface.networks.clone();
		networks.sort();
		for network in networks {
			print!("  IP: {},", network);
		}
	}
	// end of output
	println!("");
	println!("---------------Ctrl-C to exit-----------");
	// run a refresh, update all SystemBase values to reflect current system stats
	SystemBase::refresh(base);
}

// global atomic boolean only set by sigint handler (ctrlc)
// checked between refreshes to cleanly terminate overtopr
static BEGIN_EXIT: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), ctrlc::Error> {
    ctrlc::set_handler(|| {
				// immediately clear screen to hide the "^C" that was just injected into the tty
				clearscreen::clear().expect("failed to clear screen");
        println!("Exiting!!");
				// set the static atomic bool which indicates to stop refresh-looping
				BEGIN_EXIT.store(true,Ordering::Relaxed);
    })?;
	// create a generic SystemBase which represents our gathered System information
	let mut base = SystemBase::new();
	while BEGIN_EXIT.load(Ordering::Relaxed) == false {
		// refresh system info and print it
		refresh_and_print(&mut base);
		// system stats refresh delay
		thread::sleep(Duration::from_secs(2));
		clearscreen::clear().expect("failed to clear screen");
	}
		Ok(())
}
