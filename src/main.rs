use clap::Parser;
use linecount::count_lines;
use std::process::{
	Command,
	Stdio,
};
use sysinfo::{
	CpuExt,
	System,
	SystemExt,
};

#[derive(Parser, Default, Debug)]
struct Arguments {
	device: String,
	#[clap(short, long)]
	info: String,
}

fn main() {
	let sys = System::new();

	let args = Arguments::parse();

	parse_args(args, sys);
}

fn parse_args(args: Arguments, sys: System) {
	match args.device.as_str() {
		"cpu" => {
			if args.info == "usage" {
				get_cpu_usage(sys);
			} else {
				println!("Unknown info requested")
			}
		}
		"memory" | "ram" => {
			if args.info == "usage" {
				get_memory_usage(sys);
			}
		}
		"os" => {
			if args.info == "updates" {
				get_updates();
			} else if args.info == "distro" {
				get_distro(sys)
			}
		}
		"network" => {
			if args.info == "status" {
				let status = get_network_status();
				if status == 1 {
					println!("1");
				} else {
					println!("0");
				}
			}
		}
		&_ => {
			println!("Unknown device!");
		}
	}
}

fn get_cpu_usage(mut sys: System) {
	sys.refresh_cpu();
	println!("{}", sys.global_cpu_info().cpu_usage());
}

fn get_memory_usage(mut sys: System) {
	sys.refresh_memory();
	println!("{}", ((sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0).round());
}

fn get_updates() {
	let pkg_list = Command::new("pacman")
		.arg("-Qu")
		.stdout(Stdio::piped())
		.spawn()
		.expect("Couldn't query `pacman` to get updates.");

	let pkg_count = count_lines(pkg_list.stdout.unwrap()).unwrap();
	println!("{}", pkg_count);
}

fn get_distro(sys: System) {
	println!("{}", sys.distribution_id());
}

fn get_network_status() -> i32 {
	let servers = ["archlinux.org", "github.com", "google.com"];

	for server in &servers {
		let ping = Command::new("ping")
			.arg("-c")
			.arg("1")
			.arg(server)
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.status();

		match ping {
			Ok(status) => {
				if status.success() {
					return 1;
				}
			}
			Err(_) => return 0,
		}
	}
	return 0;
}
