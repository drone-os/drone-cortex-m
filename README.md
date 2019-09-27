[![crates.io](https://img.shields.io/crates/v/drone-cortex-m.svg)](https://crates.io/crates/drone-cortex-m)
![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# drone-cortex-m

ARM® Cortex®-M platform crate for Drone, an Embedded Operating System.

## Supported Cores

| Architecture | Core name             | Cargo features                      | Rust target             |
|--------------|-----------------------|-------------------------------------|-------------------------|
| ARMv7-M      | ARM® Cortex®-M3 r0p0  | `cortex_m3_r0p0`                    | `thumbv7m-none-eabi`    |
| ARMv7-M      | ARM® Cortex®-M3 r1p0  | `cortex_m3_r1p0`                    | `thumbv7m-none-eabi`    |
| ARMv7-M      | ARM® Cortex®-M3 r1p1  | `cortex_m3_r1p1`                    | `thumbv7m-none-eabi`    |
| ARMv7-M      | ARM® Cortex®-M3 r2p0  | `cortex_m3_r2p0`                    | `thumbv7m-none-eabi`    |
| ARMv7-M      | ARM® Cortex®-M3 r2p1  | `cortex_m3_r2p1`                    | `thumbv7m-none-eabi`    |
| ARMv7E-M     | ARM® Cortex®-M4 r0p0  | `cortex_m4_r0p0`                    | `thumbv7em-none-eabi`   |
| ARMv7E-M     | ARM® Cortex®-M4 r0p1  | `cortex_m4_r0p1`                    | `thumbv7em-none-eabi`   |
| ARMv7E-M     | ARM® Cortex®-M4F r0p0 | `cortex_m4f_r0p0`, `fpu` (optional) | `thumbv7em-none-eabihf` |
| ARMv7E-M     | ARM® Cortex®-M4F r0p1 | `cortex_m4f_r0p1`, `fpu` (optional) | `thumbv7em-none-eabihf` |

**NOTE** Cargo features for `drone-cortex-m` dependency and target triple
for the resulting binary should be selected for a particular core according
this table.

## Documentation

- [Drone Book](https://book.drone-os.com/)
- [API documentation](https://api.drone-os.com/drone-cortex-m/0.10)

## Usage

Place the following to the Cargo.toml:

```toml
[dependencies]
drone-cortex-m = { version = "0.10.1", features = [...] }
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
