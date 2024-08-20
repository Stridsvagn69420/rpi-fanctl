# rpi-fanctl
Raspberry Pi GPIO PWM fan controller

A program that uses the hardware PWM of a Raspberry Pi's GPIO 18 pin to properly control a 5V fan. Currently it's just a quick test that uses the `rppal` crate, but in the future this project is supposed to be a daemon that features:
- A fan curve (X: temperature; Y: duty cycle)
- IPC to manually set the fan speed or reload the config
- IPC/pseudo files to read stats during runtime
- Dedicated daemon user (does not run as root)
- Support for Systemd, OpenRC and runit
- Automatically check if `/boot/firmware/config.txt` is configured correctly

This is sufficient for my needs, but here are some things I might add to this project:
- Point out that the fan can *technically* be any voltage as long as the MOSFET gate switches with just 3.3V and supports the fan's voltage
- Make use of the second hardware PWM channel (e.g. secondary fan)
- Read the tach value on 4-pin PWM fans