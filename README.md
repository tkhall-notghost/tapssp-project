# overtopr
Overtopr will be a system monitor written in Rust by **Tessa Hall**.

[Final project](https://fpl.cs.depaul.edu/cpitcher/courses/csc363/worksheets/project.html) (DePaul University internal link) for for my Theory and Practice of Safe Systems Programming CSC-463 class.


# Summary overview

I will try to implement as many of the below features as possible. However unlike "top" or similair system process monitors I want a CLI/TUI system overview from a more abstract level, without individual processes. Instead the most granular element of the system to inspect in the monitor will be the status of various system services, catering towards systemd units, but written with traits for extensibility for services of different init systems. The rest of the system monitor output will be typical (if more visually concise) system resource utilization you would expect from a normal system resource monitor.

Since I don't intend on setting up a TUI unless time permits, the initial command will output the full system monitor summary to be piped into a pager or saved to a file. Then later I could add a flag to enable the TUI.

I will be developing this on my Arch Linux machine targeting a typical Systemd environment. I hope that compatibilty will be broader than my distrobution, but I might not be able to make guarantees. In any case, anything which fails should fail safely.

# Feature roadmap
I am including as much as I can think of under this feature roadmap in order of which features should be implemented first. This is not to be taken as my realistic expectations for what I may actually be able to finish before the project is due, but just an outline of what more could be done.

Key:
- [ ] unimplemented
- [x] implemented
- [x] :white_check_mark: implemented & pretty

- [x] List memory utilization (RAM)
  - [x] Used
  - [x] Available
  - [x] Free
  - [x] Swap usage
- List CPU utilization
  - [ ] Totals
	- [ ] Temperature
	- [x] Average CPU core utilization %
	- [ ] Number of cores available
	- [ ] CPU Model
	- [ ] CPU Frequency
  - [ ] CPU utilization % per-core
- [ ] list current network namespaces and hardware network interfaces
  - [x] list network interfaces
  - [ ] up/down status
  - [ ] IP address (inet) (if applicable)
  - [ ] MAC address (link) (if applicable)
  - [ ] Network interface statistics
- [ ] (stretch-goal) list system services (systemd units by default)
  - Active
	- Running (resource utilization summary)
	- Exited (and why it exited)
  - Inactive
  - Enabled/Disabled state
  - Root or User service? (if supported by init system)
- [ ] (stretch goal) TUI with Ncurses or something similar if time permits

Basically, this would serve as an okay if very basic and broad Linux server monitor.

# Potentially relevant Rust crates

- [sysinfo crate](https://doc.cuprate.org/sysinfo/index.html)
  - supports linux/bsd/windows/$LATESTNAMEFORAPPLEOS
  - basic system utilization info
  - limited network monitoring utility
- [zbus_systemd crate](https://docs.rs/zbus_systemd/latest/zbus_systemd/)
  - interface with all elements of systemd, but...
  - I'm interested in [list_units](https://docs.rs/zbus_systemd/latest/zbus_systemd/systemd1/struct.ManagerProxy.html#method.list_units) which meets the corresponding [ListUnits() from the freedesktop spec](https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.systemd1.html#ListUnits()).
  - Also has more detailed network interface information for Linux systems supporting systemd
- [systemstat crate](https://codeberg.org/valpackett/systemstat)
  - Fallback system stats crate. Looks less maintained.

# An Aside on permissions

This program will inevitably need root read and execute permissions to work correctly on a Linux machine. I will be using established crates for interfacing with system resources when possible, and in cases where it is not I will open files as read-only to prevent any possible system breakage from logic errors. So no matter what this should be safe to execute regardless of the required permissions. I understand if that claim might not persuade whoever has to grade this though and I can package the end product with a demo in a linux container if requested. Email me at thall42@depaul.edu or message me on Discord (Tessa Hall) if necessary.
