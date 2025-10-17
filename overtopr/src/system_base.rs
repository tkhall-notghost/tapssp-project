// general cross-platform library for system information
use sysinfo::{Networks, System};
// networking information from systemd-networkd dbus manager proxy object
use futures::executor;
use zbus::Connection;
use zbus_systemd::network1::{LinkProxy, ManagerProxy};

pub struct SystemBase {
	// these types aren't final
	sys: System,
	mem_used: u64,
	mem_avail: u64,
	mem_free: u64,
	swap_used: u64,
	cpu_avg: f32,
	net_interfaces: String,
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
			net_interfaces: "N/A".to_string(),
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
		let mut names = String::new();
		for (interface_name, _) in &networks {
			// return string of interfaces in net_interfaces
			names.push_str(interface_name);
			names.push(' ');
		}
		self.net_interfaces = names;
		// TODO: get a struct filled with systemd-fetched information from this future
		// let systemd_info = executor::block_on(self.systemd_refresh());
		self
	}
	// get information from systemd dbus proxy, if possible
	// TODO: make the return type Option<SystemDFetchedInfoStruct> or something
	async fn systemd_refresh(&mut self) -> Option<i32> {
		let maybe_connection = Connection::session().await;
		match maybe_connection {
			Err(_) => None,
			Ok(connection) => {
				// docs for network manager: https://docs.rs/zbus_systemd/latest/zbus_systemd/network1/struct.ManagerProxy.html
				// provides proxy of: https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.network1.html
				// let netmanager = ManagerProxy::new(&connection);
				let netmanager = executor::block_on(ManagerProxy::new(&connection));
				match netmanager {
					Err(_) => (),
					Ok(manager) => {
						// In this case dbus was able to get a connection and create a manager proxy
						// so here is where the work of actually getting network info can begin
						let maybe_links = manager.list_links();
						// linkvec is empty if there is an error
						let linkvec = executor::block_on(maybe_links).unwrap_or(Vec::new());
						for link in linkvec {
							// presumably these contain useful info...
							// gotta read some docs to find out what exactly...
							let link_index = link.0; // index of interface?
							let link_name = link.1; // interface name string?
							let link_oopath = link.2; // dbus internal identifier object?
							let maybe_linkobject = executor::block_on(LinkProxy::new(&connection, link_oopath));
							match maybe_linkobject {
								Err(_) => (),
								Ok(lo) => {
									// lo is a: https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.network1.html#Link%20Object
									// addrinfo may be blank if it can't be found
									let addrinfo = executor::block_on(lo.address_state()).unwrap_or(String::from(""));
								}
							}
						}
					}
				}
				Some(41)
			}
		}
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
