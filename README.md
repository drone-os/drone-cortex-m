# drone-stm32

## [Documentation](https://docs.rs/drone-stm32)

[Drone] implementation for STM32 microcontrollers.

## Installation

Instructions will be given for Debian-based Linux systems.

Install the following packages:

```sh
$ sudo apt-get install build-essential cmake libusb-1.0-0 libusb-1.0-0-dev \
  pandoc gcc-arm-none-eabi gdb-arm-none-eabi qemu-system-arm qemu-user
```

Copy [udev rules][rules.d] for ST-Link programmer to the
`/etc/udev/rules.d/`, and run the following commands:

```sh
$ sudo udevadm control --reload-rules
$ sudo udevadm trigger
```

[OpenOCD] is required. It is recommended to install it from the source,
because repository package is outdated and doesn't contain configuration for
newer chips and boards.

[Drone]: https://github.com/drone-os/drone
[OpenOCD]: http://openocd.org/
[rules.d]: https://github.com/texane/stlink/tree/master/etc/udev/rules.d

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
