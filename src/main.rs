use clap::Parser;
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
	let mut sys = System::new();

	let args = Arguments::parse();

	parse_args(args, sys);
}

fn parse_args(args: Arguments, mut sys: System) {
	match args.device.as_str() {
		"cpu" => {
			if args.info.as_str() == "usage" {
				get_cpu_usage(sys);
			} else if args.info.as_str() == "info" {
				todo!();
			}
		}
		&_ => {
			println!("Unknown device!");
		}
	}
}

fn get_cpu_usage(mut sys: System) {
	sys.refresh_cpu();
	println!("{}%", sys.global_cpu_info().cpu_usage());
}
