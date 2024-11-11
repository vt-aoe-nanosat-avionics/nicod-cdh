# How to Run Rust on the Microcontroller
# This tutorial will be for Linux (This document walks you through the same start-up process)
https://doc.rust-lang.org/stable/embedded-book/intro/index.html

# Getting Started
```bash
sudo apt install curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# We are targeting the M4F Microcontroller
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils
rustup component add llvm-tools
cargo install cargo-generate
sudo apt install gdb-multiarch openocd qemu-system-arm
```
# Udev Rules (This will allow you to write binary output files to the board)
```bash
# Create this path 
/etc/udev/rules.d/
# Create this file
touch 70-st-link.rules
nano 70-st-link.rules
# Add this code to the file:

# STM32F3DISCOVERY rev A/B - ST-LINK/V2
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", TAG+="uaccess"
# STM32F3DISCOVERY rev C+ - ST-LINK/V2-1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", TAG+="uaccess"

sudo udevadm control --reload-rules
lsusb # To see if the board is recognized by the PC
# Make a note of the bus and device number 

```
# Once the Udev rules are set move onto Cargo-generate
```bash
cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
rust_blink
cd rust_blink/src
nano main.rs
# Now you can add your rust code to the main file
# Write out the code and save the file
cd ../.cargo
nano config.toml
# In this file you will want to uncomment the thumbv7em-none-eabihf which is the target for the M4F, I also changed the first line to match this as well
rustup target add thumbv7em-none-eabihf
cargo build

```
# Finding the binary file
```bash
cd target/thumbv7em-none-eabihf/debug
# This is where the binary file outputs

```









