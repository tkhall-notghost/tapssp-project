// general cross-platform library for system information
use sysinfo::{Networks, System,MacAddr};
use core::net::IpAddr;

#[derive(Clone)]
pub struct NetIfaceInfo {
		name: String,
		tx_bytes: u64,
		rx_bytes: u64,
		mac: MacAddr,
		networks: Vec<IpAddr>,
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
		let networks = Networks::new_with_refreshed_list();
		let mut net_ifaces = Vec::new();
			for (interface_name, net_data) in &networks {

					let tx_bytes = net_data.transmitted();
					let rx_bytes = net_data.received();
					let mac_addr = net_data.mac_address();
					// this is ALLEGEDLY a struct with an addr field and a prefix field. however I can't access them?
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
		// TODO: use procfs crate to fetch detailed networking info
			self.net_interfaces = net_ifaces;
			/*
		an attempt with procfs, but it seems you can't directly map a destination IP in use to each interface.
		let iface_proc_result = procfs::net::	InterfaceDeviceStatus::current().unwrap_or(panic!("failed to stat the /proc/net interface status file(s)"));
		let iface_statuses = iface_proc_result.0;
		for (iface,dev_status) in iface_statuses {
				// iface is name of interface
				// dev_status is the DeviceStatus struct https://docs.rs/procfs/latest/procfs/net/struct.DeviceStatus.html
	  }
			 */
		
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
