use core::net::IpAddr;
// general cross-platform library for system information
use sysinfo::{Components, Disks, MacAddr, Networks, System};
// used for get_prettybytes
use byte_unit::{Byte,UnitType};


// Used to print out a rounded (two decimals) byte value in an appropriate unit.
// Not for bits! Binary only!
// also preserves original bytes count as u64 in result tuple
fn get_prettybytes(bytes:u64) -> (u64, String) {
	let clonedbytes = bytes.clone();
	let byteunit = Byte::from_u64(clonedbytes).get_appropriate_unit(UnitType::Binary);
	let roundedstr = format!("{byteunit:.2}");
	(clonedbytes,roundedstr)
}
// Used for creating indexed placeholder strings
fn placeholdertitle(index:u16,title:String) -> String {
	let mut retstring = String::from(title);
	let indexstr = index.to_string();
	retstring.push_str(indexstr.as_str());
	return retstring
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
#[derive(Clone)]
pub struct DiskInfo {
		pub name:String,
		pub fs:String,
		pub mnt:String,
		pub total:(u64,String),
		pub avail:(u64,String),
		pub read:(u64,String),
		pub written:(u64,String),
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
	swap_used: (u64,String),
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
			swap_used: (0,String::from("N/A")),
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
	pub fn get_swap_used(&mut self) -> (u64,String) {
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
	fn refresh_disks(&mut self) -> () {
		self.disks.refresh(true);
		// Update disk stats:
		let mut ret_disks = Vec::new();
		// NOTE: Can't zip disks with the usual "enumerate" here since usize isn't specific enough and I need a specific int size for enumeration
		// NOTE: If your number of mounted disks can't fit in a u16, seek help (65535 mounted disks, not including network mounts, required to panic here)
		let dlength:u16 = self.disks.list().len() as u16;
		let dnums:Vec<u16> = (0..dlength).into_iter().collect();
		let enumdisks = self.disks.list().iter().zip(dnums);
		// i for current (u16) disk index as enumerated fallback printout for strings
		for (disk,i) in enumdisks {
				let usage = disk.usage();
				// convert the names below to Strings, allowing failures
				let name = match disk.name().to_str() {
						Some(n) => {
								// luks mapped disks can have excessive names that are just a full UUID
								if n.contains("luks") {String::from("luks disk")}
								else if n == "" {placeholdertitle(i,String::from("Disk "))}
								else {String::from(n)}
						},
						None => { // Fallback string (should not occur on a typical OS)
								placeholdertitle(i,String::from("Disk "))
						},
				};
				let fs = match disk.file_system().to_str() {
						Some(f) => String::from(f),
						None => placeholdertitle(i,String::from("unknown fs "))
				};
				// luks mounts will have long uuid strings, so shorten those
				let mnt = match disk.mount_point().to_str() {
						Some (m) => String::from(m),
						None => placeholdertitle(i,String::from("mount "))
				};
				let dinfo = DiskInfo {
						name: String::from(name),
						fs: String::from(fs),
						mnt: String::from(mnt),
						total: get_prettybytes(disk.total_space()),
						avail: get_prettybytes(disk.available_space()),
						read: get_prettybytes(usage.read_bytes),
						written: get_prettybytes(usage.written_bytes),
				};
				ret_disks.push(dinfo);
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
		self.swap_used = get_prettybytes(self.sys.used_swap());
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
