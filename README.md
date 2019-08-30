## Chip8 emulator in rust

[![Build Status](https://travis-ci.org/johannst/chip-remu.svg?branch=master)](https://travis-ci.org/johannst/chip-remu)

This is a Chip8 emulator I am going to write in rust in my spare time.

![demo](demo.png)

### build and start the emulator

```rust
cargo build --release
./target/release/chip8-remu roms/demos/Zero_Demo_zeroZshadow_2007.ch8
```

### License

This project is licensed under [MIT](./LICENSE) license.

Files in `./roms` do not fall under this license. These files come from
[dmatlack/chip8](https://github.com/dmatlack/chip8/tree/master/roms).

### Reference

- https://en.wikipedia.org/wiki/CHIP-8
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

