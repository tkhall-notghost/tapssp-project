use sysinfo::{
    Components, Disks, Networks, System,
};

pub struct SystemBase {
		// these types aren't final
		sys:System,
		mem_used:u64,
		mem_avail:u64,
		mem_free:u64,
		swap_used:u64,
		cpu_avg:f32,
		net_interfaces:String,
}

impl SystemBase {
		pub fn new() -> SystemBase {
				let base_sys = SystemBase {
						sys: System::new_all(),
						mem_used: 0,
						mem_avail: 0,
						mem_free: 0,
						swap_used: 0,
						cpu_avg: 0.0,
						net_interfaces: "N/A".to_string(),
		    };
				base_sys
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
				let mut names = String::new();
				for (interface_name, _) in &networks {
						// return string of interfaces in net_interfaces
						names.push_str(&interface_name);
						names.push_str(" ");
				}
				self.net_interfaces = names;
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
		pub fn get_network_interfaces(&mut self) -> String {
				self.net_interfaces.clone()
		}
}
