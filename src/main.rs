extern crate docopt;
extern crate i2cdev;
#[macro_use]
extern crate serde_derive;
extern crate bme280_rs;

use docopt::Docopt;

#[cfg(target_os = "linux")]
use i2cdev::linux::*;

use bme280_rs::*;

const USAGE: &'static str = "
Reading BME280 sensor value

Usage:
  bme280 <device> [--address <addr>] [--temperature] [--humidity] [--pressure]
  bme280 (-h | --help)
  bme280 (-v | --version)

Options:
  -h --help    Show this help text.
  --address <addr>     I2C device address [default: 119] (=0x77)
  --temperature    Show temperature in degree Celsius.
  --humidity    Show humidity in %.
  --pressure    Show pressure in hPa.
  -v --version    Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_device: String,
    flag_address: u16,
    flag_version: bool,
    flag_temperature: bool,
    flag_humidity: bool,
    flag_pressure: bool,
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("This program can run only on Linux")
}

#[cfg(target_os = "linux")]
fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("bme280 {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let address = args.flag_address;
    let config: Config = Config {
        mode: Mode::Force,
        oversampling_temperature: Oversampling::X1,
        oversampling_pressure: Oversampling::X1,
        oversampling_humidity: Oversampling::X2,
        standby_time: StandbyTime::Ms1,
        filter_coeff: IIRFilterCoeff::OFF,
        spi3w_enabled: false,
    };

    let dev = LinuxI2CDevice::new(args.arg_device, address).unwrap();
    let mut bme280 = BME280::new(dev, config).unwrap();

    bme280.oneshot_measure().unwrap();

    if args.flag_temperature {
        println!("{}", bme280.temperature().unwrap());
    }
    if args.flag_humidity {
        println!("{:.2}", bme280.humidity().unwrap());
    }
    if args.flag_pressure {
        println!("{:.2}", bme280.pressure().unwrap() / 100.0);
    }
}
