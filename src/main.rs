use clap::Parser;
use linecount::count_lines;
use std::{
	io,
	process::{
		Command,
		Stdio,
	},
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
			if args.info.as_str() == "usage" {
				get_cpu_usage(sys);
			} else if args.info.as_str() == "info" {
				todo!();
			}
		}
		"memory" | "ram" => {
			if args.info.as_str() == "usage" {
				get_memory_usage(sys);
			}
		}
		"os" => {
			if args.info.as_str() == "updates" {
				get_updates();
			} else if args.info.as_str() == "distro" {
				get_distro(sys)
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

fn get_distro(mut sys: System) {
	println!("{}", sys.distribution_id());
}
