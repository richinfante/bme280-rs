# bme280-rs
A library and an executable for using bme280 connected via I2C on Linux.

## Usage
```bash
$ cargo build
$ cargo run -- -h
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
$ cargo run -- /dev/i2c-1 --address 118 --temperature
30.05
$ cargo run -- /dev/i2c-1 --address 118 --humidity
64.05
$ cargo run -- /dev/i2c-1 --address 118 --pressure
1005.42
```
