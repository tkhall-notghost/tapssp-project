/*

Overtopr - Rust system monitor. Student project by Tessa Hall.
MIT License 2025

 */

// clean exiting
use std::sync::atomic::{AtomicBool, Ordering};
// refresh delay
use std::{thread, time::Duration};
// colors and text styles
use crossterm::style::Stylize;
// system monitor data collection internals
mod system_base;
use crate::system_base::SystemBase;

/*
Print a subsections header line of standard length.
Number of chars in title string MUST fit within u16.
Number of chars in title SHOULD NOT exceed 66.
*/
fn print_div(dtitle: String) {
	let charquota: u16 = 68;
	let mut tlen = 0;
	// get length as u16 instead of usize
	for _ in dtitle.chars() {tlen += 1;}
	let mut dashes = charquota - tlen;
	print!("{}", dtitle.bold());
	while dashes > 0 {
		print!("-");
		dashes -= 1;
	}
		println!();
}

/*
Takes an f32 representing a percentage and prints it prettily inline.

Ex: CPU core utilization
Printed without a newline with a color from green to red indicating utilization level.
*/
fn print_percent(usage: f32) {
	// TODO: write and use this, no println!, only print!
	let round = usage.round();
	let usgstr = round.to_string();
	// pick color based on percentage thresholds
	let color: crossterm::style::Color = {
		if round > 50.0 {
			crossterm::style::Color::Red
		} else if round > 25.0 {
			crossterm::style::Color::Yellow
		} else {
			crossterm::style::Color::Green
		}
	};
	print!("{}%", usgstr.with(color));
}
/*
Takes two system-metric tuples from SystemBase and returns a pretty percentage printout.

Junk values are permissible in the tuple string values.
Arguments are only tuples for ease of use with SystemBase.
*/
fn print_fraction((used, _): (u64, String), (avail, _): (u64, String)) {
	// get percentage of bytes used:
	let usedf = used as f32;
	let availf = avail as f32;
	let percent32: f32 = (usedf / availf) * 100.0;
	print_percent(percent32);
}

// print all network statistics for all interfaces
fn print_net(base: &mut SystemBase) {
	print_div(String::from("Network Interfaces"));
	let mut ifaces = base.get_network_interfaces().clone();
	ifaces.sort_by(|a, b| b.name.cmp(&a.name));
	let mut firstiface = true;
	for iface in ifaces {
		if !firstiface {println!();}
		println!(" {} - {}", iface.name, iface.mac);
		println!("  tx: {} - rx: {} ", iface.tx_bytes.1, iface.rx_bytes.1);
		let mut networks = iface.networks.clone();
		networks.sort();
		for network in networks {
			print!("  IP: {},", network);
		}
		firstiface = false;
	}
}

// print all disk stats for all disks
fn print_disk(base: &mut SystemBase) {
	print_div(String::from("Disk"));
	for disk in base.get_disks() {
		println!("{} - {} - {} ", disk.name, disk.fs, disk.mnt);
		print!("  ");
		let used = disk.total.0 - disk.avail.0;
		let usedt = (used, String::from(""));
		print_fraction(usedt, disk.total.clone());
		print!(" used. ");
		println!(" {} Available of {} total", disk.avail.1, disk.total.1);
		println!("  I/O r:{} / w:{}", disk.read.1, disk.written.1);
	}
}

// function to print temperature information
fn print_temp(base: &mut SystemBase) {
	// thermals are not supported on some machines/OSs (Windows!), so be prepared to drop them
	let mut thermalstats = base.get_comp_temps().clone();
	if !thermalstats.is_empty() {
		print_div(String::from("Thermal"));
		thermalstats.sort_by(|a, b| b.0.cmp(&a.0));
		for (component_string, celsius) in thermalstats {
			println!("{:.1}°C - {} ", celsius, component_string);
		}
	}
}
// function to print values pertaining to RAM and SWAP usage
fn print_mem(base: &mut SystemBase) {
	print_div(String::from("RAM and Swap"));
	let used = base.get_mem_used();
	let avail = base.get_mem_avail();
	println!("Used: {}", used.1);
	println!("Available: {}", avail.1);
	println!("Free: {}", base.get_mem_free().1);
	print!("Estimated RAM Usage: ");
	print_fraction(used, avail);
	println!();
	let cswap = base.get_swap_used();
	print!("Swap Used: ");
	// any swap usage should show as red
	if cswap.0 > 0 {
		print!("{}", cswap.1.red());
	} else {
		print!("{}", cswap.1.green());
	}
	println!();
}

// function to print all CPU related metrics
fn print_cpu(base: &mut SystemBase) {
	print_div(String::from("CPU"));
	print!("CPU avg: ");
	print_percent(base.get_cpu_avg());
	println!();
	println!("CPU Cores information:");
	let mut brand = String::new();
	let mut i: u32 = 1;
	// TODO: Make this look a lot better
	for c in base.get_cores() {
		brand = c.brand.clone();
		print!("[ {} - ", c.name);
		print_percent(c.usage);
		print!(" ] ");
		if (i.is_multiple_of(4)) && (i != 1) {
			println!();
		}
		i += 1;
	}
	// there won't be a newline at the end in this case, so add one then:
	if i.is_multiple_of(4) {println!();}
	println!("  brand: {} - {} cores", brand, i-1);
}

/*
Main printing and refreshing code for the core display logic of overtopr.

This actually prints and then refreshes, to avoid waiting to print
while fetching system stats after the screen has just been cleared
which causes an annoying flashing effect.
Part of why the first print has no readouts,
the other part being many stats that require more than one sample
in order to display a meaningful metric.
*/
fn refresh_and_print(base: &mut SystemBase) {
	println!("{}", "overtopr".bold().dark_magenta());
	print_cpu(base);
	print_mem(base);
	print_temp(base);
	print_disk(base);
	print_net(base);
	// end of output
	println!();
	println!("----- Ctrl-C to exit -----");
	// run a refresh, update all SystemBase values to reflect current system stats
	SystemBase::refresh(base);
}

// global atomic boolean only set by sigint handler (ctrlc)
// checked between refreshes to cleanly terminate overtopr
static BEGIN_EXIT: AtomicBool = AtomicBool::new(false);

/*
Main function. Run at program start to handle refreshes, delay, and clean exiting.
*/
fn main() -> Result<(), ctrlc::Error> {
	ctrlc::set_handler(|| {
		// immediately clear screen to hide the "^C" that was just injected into the tty
		clearscreen::clear().expect("failed to clear screen");
		println!("Exiting!!");
		// set the static atomic bool which indicates to stop refresh-looping
		BEGIN_EXIT.store(true, Ordering::Relaxed);
	})?;
	// create a generic SystemBase which represents our gathered System information
	let mut base = SystemBase::new();
	while !BEGIN_EXIT.load(Ordering::Relaxed) {
		// refresh system info and print it
		refresh_and_print(&mut base);
		// system stats refresh delay
		thread::sleep(Duration::from_secs(2));
		clearscreen::clear().expect("failed to clear screen");
	}
	Ok(())
}
