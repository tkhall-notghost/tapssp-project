// general cross-platform library for system information
use core::net::IpAddr;
use sysinfo::{Components, MacAddr, Networks, System};

#[derive(Clone)]
pub struct NetIfaceInfo {
	pub name: String,
	pub tx_bytes: u64,
	pub rx_bytes: u64,
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
	mem_used: u64,
	mem_avail: u64,
	mem_free: u64,
	swap_used: u64,
	cpu_avg: f32,
	net_interfaces: Vec<NetIfaceInfo>,
	cores: Vec<CoreInfo>,
	sys_networks: Networks,
	sys_components: Components,
	component_temps: Vec<(String, f32)>,
}

impl SystemBase {
	pub fn new() -> SystemBase {
		SystemBase {
			sys: System::new_all(),
			mem_used: 0,
			mem_avail: 0,
			mem_free: 0,
			swap_used: 0,
			cpu_avg: 0.0,
			net_interfaces: Vec::new(),
			cores: Vec::new(),
			sys_networks: Networks::new_with_refreshed_list(),
			sys_components: Components::new_with_refreshed_list(),
			component_temps: Vec::new(),
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
		self
	}
	// example getter:
	pub fn get_cpu_avg(&mut self) -> f32 {
		self.cpu_avg
	}
	pub fn get_mem_used(&mut self) -> u64 {
		self.mem_used
	}
	pub fn get_mem_avail(&mut self) -> u64 {
		self.mem_avail
	}
	pub fn get_mem_free(&mut self) -> u64 {
		self.mem_free
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
			let iface_info = NetIfaceInfo {
				name: interface_name.to_string(),
				tx_bytes: net_data.transmitted(),
				rx_bytes: net_data.received(),
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
		self.mem_used = self.sys.used_memory();
		self.mem_avail = self.sys.available_memory();
		self.mem_free = self.sys.free_memory();
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
