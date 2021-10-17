![CI](https://github.com/hgrsd/nibble8/actions/workflows/ci.yaml/badge.svg)

# nibble8

![an image showing the output of a chip 8 rom, i.e. the letters C8](assets/ch8.png)

Yet another [Rust crate](https://crates.io/crates/nibble8) for a Chip-8 interpreter, using [sdl2](https://crates.io/crates/sdl2).

### Prerequisites

The Rust toolchain should be installed.

SDL2 development library >= 2.0.5 must also be installed. See [here](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries) for installation instructions.

### Usage

#### From source
```sh
git clone https://github.com/hgrsd/nibble8
cargo run --release <path_to_rom.ch8>
```

#### Install binary
```sh
cargo install nibble8
nibble8 <path_to_rom.ch8>
```

### Running the test suite
`cargo test`

### Literature
See the following articles on the Chip 8 instruction set.
- https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM


### Contributions
Very welcome! Feel free to open a PR or to open a new issue.