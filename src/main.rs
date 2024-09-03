use std::process::ExitCode;
use rppal::system::DeviceInfo;
use apputils::{Colors, paintln};

mod meta;
use meta::APP_NAME;

#[cfg(target_family("unix"))]
fn main() -> ExitCode {
	let Ok(devinfo) = DeviceInfo::new() else {
		paintln!(Colors::Red, "{APP_NAME} is only supported on Raspberry Pi SBCs!");
		return ExitCode::FAILURE;
	};
	paintln!(Colors::Green, "SBC: {}", devinfo.model());
	paintln!(Colors::Red, "SoC: {}", devinfo.soc());

	// TODO: Add test logic here (see https://github.com/golemparts/rppal/blob/master/examples/pwm_blinkled.rs and https://github.com/golemparts/rppal/blob/master/examples/pwm_servo.rs)

	ExitCode::SUCCESS
}