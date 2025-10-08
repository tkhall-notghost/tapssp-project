
pub struct SystemBase {
		// these types aren't final
		mem_used:f64,
		mem_avail:f64,
		mem_free:f64,
		swap_used:f64,
		cpu_temp:f64,
		cpu_avg:f64,
		net_interfaces:String,
		net_uptotal:u64,
		net_downtotal:u64
}

impl SystemBase {
		pub fn new() -> SystemBase {
				let base_sys = SystemBase {
						mem_used: 0.0,
						mem_avail: 0.0,
						mem_free: 0.0,
						swap_used: 0.0,
						cpu_temp: 0.0,
						cpu_avg: 0.0,
						net_interfaces: "N/A".to_string(),
						net_uptotal: 0,
						net_downtotal: 0
		    };
				base_sys
		}
		pub fn refresh(&mut self) -> &SystemBase {
				// TODO: refresh all "base" (platform-generic) system stats here
				// TODO: trigger refresh of platform-specific system stats
				// in a refresh function call to a sub-type
				self
		}
		// example getter:
		pub fn get_cpu_avg(&mut self) -> f64 {
				self.cpu_avg
		}
}
