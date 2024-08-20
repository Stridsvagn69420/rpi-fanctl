use std::process::ExitCode;
use rppal::system::DeviceInfo;
use apputils::{Colors, paintln};

fn main() -> ExitCode {
	let Ok(devinfo) = DeviceInfo::new() else {
		paintln!(Colors::Red, "Your system does not seem to be a Raspberry Pi!");
		return ExitCode::FAILURE;
	};
	paintln!(Colors::Green, "SBC: {}", devinfo.model());
	paintln!(Colors::Red, "SoC: {}", devinfo.soc());
	ExitCode::SUCCESS
}