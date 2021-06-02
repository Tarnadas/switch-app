# Switch app

An attempt to build a Nintendo Switch homebrew app with Rust.

Uses [egui with SDL2 and OpenGL](https://github.com/ArjunNair/egui_sdl2_gl.git).

## Setup

- Download [Rust toolchain](https://github.com/Tarnadas/switch-app/releases/tag/0.0.1) and extract it somewhere (I only compiled for Linux x86_64.
  If you need the toolchain on another platform, please clone [this repository](https://gitlab.com/NX-rs/rust),
  then run `./x.py build -i --stage 2 library/core library/std src/tools/cargo src/tools/rustdoc` and link the `build/<your_arch>/stage2` folder as a toolchain)
- Link the toolchain via Rustup: `rustup toolchain link horizon path/to/toolchain`
- Install [devkitpro](https://devkitpro.org/wiki/Getting_Started) and install required libraries: `sudo dkp-pacman -S switch-dev switch-sdl2`
- Build ELF from source: `cargo +horizon build --target aarch64-unknown-horizon --release`
- Install [linkle](https://github.com/MegatonHammer/linkle): `cargo install --features=binaries linkle`
- Create an NRO: `linkle nro --nacp-path nacp.json target/aarch64-unknown-horizon/release/switch-app output.nro`
