# How to run rust in WSL

## Follow these steps to get rust up and running in WSL
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install cargo-binutils
rustup component add llvm-tools
cargo install cargo-generate

```
## More for the ARM cortex microcontrollers
```bash
sudo apt install gdb-multiarch openocd qemu-system-arm # For the microcontrollers
cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart

```
## To edit the rust code drill into src/main.rs file
```bash
cd src
nano main.rs
```
## To run the cargo
```bash
cargo build
# or
cargo build --release # Does not include debugging information and reduces binary file size by a large amount
```
## To optimize the binary size further you can edit the settings in the Cargo.toml file
```bash
nano Cargo.toml
#In the [profile.release] section (at the bottom) you can add these methods or change them as you see fit:

codegen-units = 1 # better optimizations
lto = true # better optimizations
strip = true
opt-level = 'z' # Also an opt-level = 's', check which one works best for your code
# Panic and debug do not do anything to file size
panic = 'abort'
debug = true # symbols are nice and they don't increase the size on Flash
```
