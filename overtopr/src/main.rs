/*

Overtopr - Rust system monitor. Student project by Tessa Hall.
MIT License 2025

 */

extern crate systemstat;

use std::thread;
use std::time::Duration;
use systemstat::{System, Platform, saturating_sub_bytes};

fn main() {
    println!("overtopr:");
		let sys = System::new(); // System from systemstat crate
		
		/*
		The following example code blocks represent only a few metrics of many more I want to gather
		and needs to be phased out for output that is much more legible
		 */
		
		// example code: print network interface stats
		match sys.networks() {
        Ok(netifs) => {
            println!("\nNetwork interface statistics:");
            for netif in netifs.values() {
                println!("{} statistics: ({:?})", netif.name, sys.network_stats(&netif.name));
            }
        }
        Err(x) => println!("\nNetworks: error: {}", x)
    }
		// example code: print memory usage
		match sys.memory() {
        Ok(mem) => println!("\nMemory: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory),
        Err(x) => println!("\nMemory: error: {}", x)
    }
		// TODO: Make a simple alert when any swap usage is detected, and a large one for max utilization
		// example code: swap usage
		match sys.swap() {
        Ok(swap) => println!("\nSwap: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(swap.total, swap.free), swap.total, swap.total.as_u64(), swap.platform_swap),
        Err(x) => println!("\nSwap: error: {}", x)
    }
		// example code: load average metric
		match sys.load_average() {
        Ok(loadavg) => println!("\nLoad average: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
        Err(x) => println!("\nLoad average: error: {}", x)
    }
		// example code: uptime
		 match sys.uptime() {
        Ok(uptime) => println!("\nUptime: {:?}", uptime),
        Err(x) => println!("\nUptime: error: {}", x)
     }
		// TODO: get per-core CPU metrics
		// example code: avg cpu load
		 match sys.cpu_load_aggregate() {
        Ok(cpu)=> {
						println!("\nMeasuring CPU load...");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
        },
        Err(x) => println!("\nCPU load: error: {}", x)
     }
		// example code: CPU temperature
		match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x)
    }
}
