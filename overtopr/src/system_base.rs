// general cross-platform library for system information
use std::path::Path;
use core::net::IpAddr;
use std::ffi::OsStr;
use sysinfo::{Components, MacAddr, Networks, System, Disks};
use byte_unit::{Byte,UnitType};


// Used to print out a byte value in an appropriate unit. Not for bits! Binary only!
// also preserves original bytes count as u64 in result tuple
fn get_prettybytes(bytes:u64) -> (u64, String) {
	let clonedbytes = bytes.clone();
	let byteunit = Byte::from_u64(clonedbytes).get_appropriate_unit(UnitType::Binary);
	let roundedstr = format!("{byteunit:.2}");
	(clonedbytes,roundedstr)
}
/*
let diskname = disk.name();
				let disk_kind = disk.kind();
				let diskfs = disk.file_system();
				let diskmnt = disk.mount_point();
				let disk_total = disk.total_space();
				let disk_avail = disk.available_space();
				let disk_read = disk.usage().read_bytes;
let disk_written = disk.usage().written_bytes;
 */
// Information about a disk from a single refresh interval
pub struct DiskInfo {
		pub name:String,
		pub fs:String,
		pub mnt:String,
		pub total:u64,
		pub avail:u64,
		pub read:u64,
		pub written:u64,
}

#[derive(Clone)]
pub struct NetIfaceInfo {
	pub name: String,
	pub tx_bytes: (u64,String),
	pub rx_bytes: (u64,String),
	pub mac: MacAddr,
	pub networks: Vec<IpAddr>,
}

#[derive(Clone)]
pub struct CoreInfo {
	pub name: String,
	pub brand: String,
	pub freq: u64,
	pub usage: f32,
}

pub struct SystemBase {
	// these types aren't final
	sys: System,
	mem_used: (u64,String),
	mem_avail: (u64,String),
	mem_free: (u64,String),
	swap_used: u64,
	cpu_avg: f32,
	net_interfaces: Vec<NetIfaceInfo>,
	cores: Vec<CoreInfo>,
	sys_networks: Networks,
	sys_components: Components,
	component_temps: Vec<(String, f32)>,
	disks: Disks,
	diskinfos: Vec<DiskInfo>,
}

impl SystemBase {
	pub fn new() -> SystemBase {
		SystemBase {
			sys: System::new_all(),
			mem_used: (0,String::from("N/A")),
			mem_avail: (0,String::from("N/A")),
			mem_free: (0,String::from("N/A")),
			swap_used: 0,
			cpu_avg: 0.0,
			net_interfaces: Vec::new(),
			cores: Vec::new(),
			sys_networks: Networks::new_with_refreshed_list(),
			sys_components: Components::new_with_refreshed_list(),
			component_temps: Vec::new(),
			disks: Disks::new_with_refreshed_list(),
			diskinfos: Vec::new(),
		}
	}
	pub fn refresh(&mut self) -> &SystemBase {
		// TODO: trigger refresh of platform-specific system stats
		// in a refresh function call to a sub-type

		// start refreshing values from high-compatibility system module:
		// self.sys.refresh_all(); // limit refresh to only used fields, as demanded later
		self.refresh_memory(); // update memory and swap stats
		self.refresh_components(); // used for component temperatures
		self.refresh_networks(); // used for network interfaces and usage stats
		self.refresh_cpu(); // used for cpu usage metrics
		self.refresh_disks(); // used for disk usage metrics
		self
	}
	// example getter:
	pub fn get_cpu_avg(&mut self) -> f32 {
		self.cpu_avg
	}
	pub fn get_mem_used(&mut self) -> (u64,String) {
		self.mem_used.clone()
	}
	pub fn get_mem_avail(&mut self) -> (u64,String) {
		self.mem_avail.clone()
	}
	pub fn get_mem_free(&mut self) -> (u64,String) {
		self.mem_free.clone()
	}
	pub fn get_swap_used(&mut self) -> u64 {
		self.swap_used
	}
	pub fn get_network_interfaces(&mut self) -> Vec<NetIfaceInfo> {
		self.net_interfaces.clone()
	}
	pub fn get_cores(&mut self) -> Vec<CoreInfo> {
		self.cores.clone()
	}
	pub fn get_comp_temps(&mut self) -> Vec<(String, f32)> {
		self.component_temps.clone()
	}
	fn refresh_disks(&mut self) -> () {
		self.disks.refresh(true);
		// Update disk stats:
		let mut ret_disks = Vec::new();
		let mut disknum = 0;
		for disk in self.disks.list() {
				let usage = disk.usage();
				// TODO: convert the below to Strings
				let _name = disk.name().to_str();
				let _fs = disk.file_system().to_str();
				let _mnt = disk.mount_point().to_str();
				// TODO: actually use accurate strings
				let dinfo = DiskInfo {
						name: String::from("diskname"),
						fs: String::from("fstype"),
						mnt: String::from("mnt"),
						total: disk.total_space(),
						avail: disk.available_space(),
						read: usage.read_bytes,
						written: usage.written_bytes,
				};
				ret_disks.push(dinfo);
				disknum = disknum+1;
		}
		self.diskinfos = ret_disks;
	}
	fn refresh_networks(&mut self) -> () {
		// Network stats:
		self.sys_networks.refresh(true);
		let mut net_ifaces = Vec::new();
		for (interface_name, net_data) in &self.sys_networks {
			let nets = net_data.ip_networks();
			let mut ret_networks = Vec::new();
			for n in nets {
				ret_networks.push(n.addr);
			}
			let tx = net_data.transmitted();
			let rx = net_data.transmitted();
			let iface_info = NetIfaceInfo {
				name: interface_name.to_string(),
				tx_bytes: get_prettybytes(tx),
				rx_bytes: get_prettybytes(rx),
				mac: net_data.mac_address(),
				networks: ret_networks,
			};
			net_ifaces.push(iface_info);
		}
		self.net_interfaces = net_ifaces;
	}
	fn refresh_components(&mut self) -> () {
		// Components:
		self.sys_components.refresh(true);
		let mut temp_components = Vec::new();
		for component in &self.sys_components {
			if let Some(temperature) = component.temperature() {
				temp_components.push((component.label().to_string(), temperature));
			}
		}
		self.component_temps = temp_components;
	}
	fn refresh_memory(&mut self) -> () {
		self.sys.refresh_memory();
		// Memory and Swap:
		// for each value, return tuple of raw bytes count as u64 and a pretty string
		self.mem_used = get_prettybytes(self.sys.used_memory());
		self.mem_avail = get_prettybytes(self.sys.available_memory());
		self.mem_free = get_prettybytes(self.sys.free_memory());
		self.swap_used = self.sys.used_swap();
	}
	fn refresh_cpu(&mut self) -> () {
		self.sys.refresh_cpu_all();
		// CPU:
		self.cpu_avg = self.sys.global_cpu_usage();
		let mut cores = Vec::new();
		for cpu in self.sys.cpus() {
			let core = CoreInfo {
				name: cpu.name().to_string(),
				brand: cpu.brand().to_string(),
				freq: cpu.frequency(),
				usage: cpu.cpu_usage(),
			};
			cores.push(core);
		}
		self.cores = cores.clone();
	}
}
