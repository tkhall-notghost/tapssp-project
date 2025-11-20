/*

Overtopr - Rust system monitor. Student project by Tessa Hall.
MIT License 2025

 */
use ctrlc;
use std::{thread, time::Duration};

mod system_base;
use crate::system_base::SystemBase;
use crossterm::style::Stylize;

use std::sync::atomic::{AtomicBool, Ordering};

// TODO: take an f32 representing a percentage (say, CPU core utilization)
// Use crossterm stylize to print it as a styled percentage
// with a color from green to red indicating utilization level
fn print_percent(usage:f32) -> () {
		// TODO: write and use this, no println!, only print!
		let round = usage.round();
		let usgstr = round.to_string();
		if round > 50.0 {
				print!("{}%", usgstr.red());
		} else if round > 25.0 {
				print!("{}%", usgstr.yellow());
		} else {
				print!("{}%", usgstr.green());
		}
}
// TODO: take two tuples with u64's representing bytes from measurements
// where the first is the number of bytes used
// and the second is the total or available
// then print them in an appropriate color from green to red
// indicating the amount used of the total
// using the Strings from the tuples
fn print_fraction((used,usedstr):(u64,String),(avail,availstr):(u64,String)) -> () {
	// TODO: write and use this, no println!, only print!
}

// this actually prints and then refreshes, to avoid waiting to print
// while fetching system stats after the screen has just been cleared
// which causes an annoying flashing effect
// which is part of why the first print has no readouts
// the other part being stats that require more than one sample
fn refresh_and_print(base: &mut SystemBase) {
	println!("{} --------------------------------------------------------------{}","CPU".bold(),"overtopr".bold());
	print!("CPU avg: ");
	print_percent(base.get_cpu_avg());
	println!("");
	println!("CPU Cores information:");
	let mut brand = String::new();
	let mut i: u32 = 0;
	// TODO: Make this look a lot better
	for c in base.get_cores() {
		brand = c.brand.clone();
		print!("[ {} - ", c.name);
		print_percent(c.usage);
		print!(" ] ");
		if i != 0 && ((i % 3) == 0) {
			println!("");
		}
		i += 1;
	}
	println!("  brand: {} - {} cores", brand, i);
	println!("{} --------------------------------------------------------------","RAM and Swap".bold());
	println!(
		"Used: {} / Available: {} - Free: {}",
		base.get_mem_used().1,
		base.get_mem_avail().1,
		base.get_mem_free().1
	);
	let cswap = base.get_swap_used();
	print!("Swap Used: ");
	// Highlight any swap usage with red
	if cswap > 0 {
		print!("{}%",cswap.to_string().red());
	} else {
		print!("{}%",cswap.to_string().green());
	}
	println!("");
	// thermals are not supported on some machines/OSs (Windows!), so be prepared to drop them
	let mut thermalstats = base.get_comp_temps().clone();
	if thermalstats.len() != 0 {
			println!("{} ---------------------------------------------------------","Thermal".bold());
			thermalstats.sort_by(|a, b| b.0.cmp(&a.0));
			for (component_string, celsius) in thermalstats {
					println!("{:.1} C - {} ", celsius, component_string);
			}
	}
	println!("{} ---------------------------------------------------------","Disk".bold());
	println!("Name - filesystem - mountpoint ");
	println!(" available / total");
	println!(" live usage stats: read/write");
	for disk in base.get_disks() {
			println!("{} - {} - {} ", disk.name, disk.fs, disk.mnt);
			println!("  {} / {} total", disk.avail.1, disk.total.1);
			println!("  r:{} / w:{}", disk.read.1, disk.written.1);
	}
	println!("{} ---------------------------------------------------------","Network Interface".bold());
	let mut ifaces = base.get_network_interfaces().clone();
	ifaces.sort_by(|a, b| b.name.cmp(&a.name));
	let mut firstiface = true;
	for iface in ifaces {
		if !firstiface {println!("");}
		println!(" {} - {}", iface.name, iface.mac);
		println!(
			"  tx: {} - rx: {} ",
			iface.tx_bytes.1, iface.rx_bytes.1
		);
		let mut networks = iface.networks.clone();
		networks.sort();
		for network in networks {
			print!("  IP: {},", network);
		}
		firstiface = false;
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
