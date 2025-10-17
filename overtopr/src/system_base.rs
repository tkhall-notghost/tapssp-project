// general cross-platform library for system information
use sysinfo::{Networks, System,MacAddr};
use core::net::IpAddr;

#[derive(Clone)]
pub struct NetIfaceInfo {
		pub name: String,
		pub tx_bytes: u64,
		pub rx_bytes: u64,
		pub mac: MacAddr,
		pub networks: Vec<IpAddr>,
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
	sys_networks: Networks,
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
			sys_networks: Networks::new_with_refreshed_list(),
		}
	}
	pub fn refresh(&mut self) -> &SystemBase {
		// TODO: refresh all "base" (platform-generic) system stats here
		// TODO: trigger refresh of platform-specific system stats
		// in a refresh function call to a sub-type

		// start refreshing values from high-compatibility system module:
		self.sys.refresh_all(); // TODO: limit refresh to used fields later
		// Memory and Swap:
		self.mem_used = self.sys.used_memory();
		self.mem_avail = self.sys.available_memory();
		self.mem_free = self.sys.free_memory();
		self.swap_used = self.sys.used_swap();
		// CPU:
		// TODO: get CPU temp
		self.cpu_avg = self.sys.global_cpu_usage();
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
		self.mem_used
	}
	pub fn get_network_interfaces(&mut self) -> Vec<NetIfaceInfo> {
		self.net_interfaces.clone()
	}
}
