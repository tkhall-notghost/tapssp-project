# overtopr
Overtopr will be a system monitor written in Rust by **Tessa Hall**. Still an early work in progress. See roadmap for actual progress, screenshot is outdated.

![Work in progress screenshot](early-wip.png)

[Final project](https://fpl.cs.depaul.edu/cpitcher/courses/csc363/worksheets/project.html) (DePaul University internal link) for for my Theory and Practice of Safe Systems Programming CSC-463 class.

## compilation/usage

- `cd overtop` (from root of this repository)
- `cargo build`
- `cargo run` to test. Ctrl-C to quit for now.

## Feature roadmap
I've changed this a bit, aside from the core feature milestones, which have all already been met.

Key:
- [ ] unimplemented
- [x] implemented
- [x] :white_check_mark: implemented & pretty

- [x] :white_check_mark: List memory utilization (RAM)
  - [x] Used
  - [x] Available
  - [x] Free
  - [x] Swap usage
- [x] :white_check_mark: List CPU utilization
  - [x] Totals
	- [x] Temperature
	- [x] Average CPU core utilization %
	- [x] CPU Model
  - [x] Number of cores available
  - [x] CPU utilization % per-core
  - [x] CPU Frequency per-core
- [x] :white_check_mark: list current network interfaces
  - [x] list network interfaces
  - [x] IP address (inet) (if applicable)
  - [x] MAC address (link) (if applicable)
  - [x] Network interface statistics
- [x] :white_check_mark: Disk Utilization
  - [x] Disk metadata
  - [x] Disk utilization
  - [x] Disk bytes I/O in last refresh
- [ ] Clean Ctrl-C interrupt, exit code zero
- [ ] (stretch goal) Terminal Decoration
  - color-code system monitor readouts between green, yellow, and red based on thresholds, red indicating "bad"/"high-utilization"
  - prettier general printout, maybe with some TUI framework?

Basically, this would serve as an okay if very basic and broad Linux server monitor.

# Potentially relevant Rust crates

- [sysinfo crate](https://doc.cuprate.org/sysinfo/index.html) Using this.
  - supports linux/bsd/windows/$LATESTNAMEFORAPPLEOS
  - basic system utilization info
  - limited network monitoring utility
- [byte-unit](https://lib.rs/crates/byte-unit) Using this.
  - For dynamically selecting the right display SI prefix for displayed byte values
- [ctrl-c crate](https://docs.rs/ctrlc/latest/ctrlc/) Not using this yet
