use core::net::IpAddr;
// general cross-platform library for system information
use sysinfo::{Components, Disks, MacAddr, Networks, System};
// used for get_prettybytes
use byte_unit::{Byte, UnitType};

/*
Take a byte value stored in a u64 and give it "pretty" String tuple companion.
Used to print out a rounded (two decimals) byte value in an appropriate SI unit.
"Pretty" string will always be rounded to two decimals.
Not for bits! Binary only! Good thing that's the only thing I ever collect!
Also preserves original bytes count as u64 in result tuple for advanced printing logic.
*/
fn get_prettybytes(bytes: u64) -> (u64, String) {
	let clonedbytes = bytes;
	let byteunit = Byte::from_u64(clonedbytes).get_appropriate_unit(UnitType::Binary);
	let roundedstr = format!("{byteunit:.2}");
	(clonedbytes, roundedstr)
}
/*
Take and index integer and a string to create a combined placeholder string.
Used for creating indexed placeholder strings
Ex: placeholdertitle(1,"Disk") -> "Disk1"
*/
fn placeholdertitle(index: u16, title: String) -> String {
	let mut retstring = title;
	let indexstr = index.to_string();
	retstring.push_str(indexstr.as_str());
	retstring
}

// Information about a disk from a single refresh interval
#[derive(Clone)]
pub struct DiskInfo {
	pub name: String,
	pub fs: String,
	pub mnt: String,
	pub total: (u64, String),
	pub avail: (u64, String),
	pub read: (u64, String),
	pub written: (u64, String),
}

// Information about a network interface
#[derive(Clone)]
pub struct NetIfaceInfo {
	pub name: String,
	pub tx_bytes: (u64, String),
	pub rx_bytes: (u64, String),
	pub mac: MacAddr,
	pub networks: Vec<IpAddr>,
}

// Information about a CPU core
#[derive(Clone)]
pub struct CoreInfo {
	pub name: String,
	pub brand: String,
	pub usage: f32,
}

/*
SystemBase is exposed to main for access to collected
system metric values for output.
however there are also values
used solely for collecting/refreshing metrics
*/
pub struct SystemBase {
	// these types aren't final
	sys: System,
	mem_used: (u64, String),
	mem_avail: (u64, String),
	mem_free: (u64, String),
	swap_used: (u64, String),
	cpu_avg: f32,
	net_interfaces: Vec<NetIfaceInfo>,
	cores: Vec<CoreInfo>,
	sys_networks: Networks,
	sys_components: Components,
	component_temps: Vec<(String, f32)>,
	disks: Disks,
	diskinfos: Vec<DiskInfo>,
}
// Many of these junk placeholder values will be displayed on the first refresh
// during program startup and the first usable metrics.
impl SystemBase {
	pub fn new() -> SystemBase {
		SystemBase {
			sys: System::new_all(),
			mem_used: (0, String::from("N/A")),
			mem_avail: (0, String::from("N/A")),
			mem_free: (0, String::from("N/A")),
			swap_used: (0, String::from("N/A")),
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
	// Function to trigger a refresh on all values within the SystemBase struct
	pub fn refresh(&mut self) -> &SystemBase {
		// start refreshing values from high-compatibility system module:
		self.refresh_memory(); // update memory and swap stats
		self.refresh_components(); // used for component temperatures
		self.refresh_networks(); // used for network interfaces and usage stats
		self.refresh_cpu(); // used for cpu usage metrics
		self.refresh_disks(); // used for disk usage metrics
		self
	}
	// getters and setters for use within main
	pub fn get_cpu_avg(&mut self) -> f32 {
		self.cpu_avg
	}
	pub fn get_mem_used(&mut self) -> (u64, String) {
		self.mem_used.clone()
	}
	pub fn get_mem_avail(&mut self) -> (u64, String) {
		self.mem_avail.clone()
	}
	pub fn get_mem_free(&mut self) -> (u64, String) {
		self.mem_free.clone()
	}
	pub fn get_swap_used(&mut self) -> (u64, String) {
		self.swap_used.clone()
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
	pub fn get_disks(&mut self) -> Vec<DiskInfo> {
		self.diskinfos.clone()
	}
	// Function to refresh all the disk statistics
	fn refresh_disks(&mut self) {
		self.disks.refresh(true);
		// Update disk stats:
		let mut ret_disks = Vec::new();
		// NOTE: Can't zip disks with the usual "enumerate" here since usize isn't specific enough and I need a specific int size for enumeration
		// NOTE: If your number of mounted disks can't fit in a u16, seek help (65535 mounted disks, not including network mounts, required to panic here)
		let dlength: u16 = self.disks.list().len() as u16;
		let dnums: Vec<u16> = (0..dlength).collect();
		let enumdisks = self.disks.list().iter().zip(dnums);
		// i for current (u16) disk index as enumerated fallback printout for strings
		for (disk, i) in enumdisks {
			let usage = disk.usage();
			// convert the names below to Strings, allowing failures
			let name = match disk.name().to_str() {
				Some(n) => {
					// luks mapped disks can have excessive names that are just a full UUID
					if n.contains("luks") {
						String::from("luks disk")
					} else if n.is_empty() {
						placeholdertitle(i, String::from("Disk "))
					} else {
						String::from(n)
					}
				}
				None => {
					// Fallback string (should not occur on a typical OS)
					placeholdertitle(i, String::from("Disk "))
				}
			};
			let fs = match disk.file_system().to_str() {
				Some(f) => String::from(f),
				None => placeholdertitle(i, String::from("unknown fs ")),
			};
			// luks mounts will have long uuid strings, so shorten those
			let mnt = match disk.mount_point().to_str() {
				Some(m) => String::from(m),
				None => placeholdertitle(i, String::from("mount ")),
			};
			let dinfo = DiskInfo {
				name,
				fs,
				mnt,
				total: get_prettybytes(disk.total_space()),
				avail: get_prettybytes(disk.available_space()),
				read: get_prettybytes(usage.read_bytes),
				written: get_prettybytes(usage.written_bytes),
			};
			ret_disks.push(dinfo);
		}
		self.diskinfos = ret_disks;
	}

	// Function that handles refreshing network interface information
	fn refresh_networks(&mut self) {
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

	// Function that handles refreshing components, for collecting thermal stats in celsius.
	fn refresh_components(&mut self) {
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

	// Function for refreshing the RAM and SWAP stats
	fn refresh_memory(&mut self) {
		self.sys.refresh_memory();
		// Memory and Swap:
		// for each value, return tuple of raw bytes count as u64 and a pretty string
		self.mem_used = get_prettybytes(self.sys.used_memory());
		self.mem_avail = get_prettybytes(self.sys.available_memory());
		self.mem_free = get_prettybytes(self.sys.free_memory());
		self.swap_used = get_prettybytes(self.sys.used_swap());
	}

	// Function for refreshing the CPU stats including individual cores
	fn refresh_cpu(&mut self) {
		self.sys.refresh_cpu_all();
		// CPU:
		self.cpu_avg = self.sys.global_cpu_usage();
		let mut cores = Vec::new();
		for cpu in self.sys.cpus() {
			let core = CoreInfo {
				name: cpu.name().to_string(),
				brand: cpu.brand().to_string(),
				usage: cpu.cpu_usage(),
			};
			cores.push(core);
		}
		self.cores = cores.clone();
	}
}
