use sysinfo::{
	CpuExt,
	System,
	SystemExt,
};

fn main() {
	let mut sys = System::new();
	sys.refresh_cpu(); // Refreshing CPU information.
	println!("{}% ", sys.global_cpu_info().cpu_usage());
}
