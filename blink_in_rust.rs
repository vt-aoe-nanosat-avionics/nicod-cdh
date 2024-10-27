#![no_std] // Used to indicate that a crate (a package of Rust code) does not link to the Rust standard library
#![no_main] // Inform the Rust compiler that you will provide your own entry point for the program. This is essential in environments where you are not using the Rust standard library

use core::ptr; //Brings in the ptr module which helps with memory mapped I/Os - adds the read and write volatile

// Constants
const PERIPH_BASE: usize = 0x4000_0000; //Constant means immutable (Cannot be changed)
const PERIPH_BASE_AHB2: usize = 0x4800_0000; // Usize represents an unsigned integer that’s the same size as the platform’s pointer size
const PERIPH_BASE_AHB1: usize = PERIPH_BASE + 0x20000;
const GPIO_PORT_C_BASE: usize = PERIPH_BASE_AHB2 + 0x0800;
const GPIOC: usize = GPIO_PORT_C_BASE;
const RCC_BASE: usize = PERIPH_BASE_AHB1 + 0x1000;
const GPIO10: u32 = 1 << 10; // This is a left bit shift operation that moves 1, 10 bits to the left
const GPIO12: u32 = 1 << 12; 
const GPIO_MODE_OUTPUT: u32 = 0x1;// u32 means it's a constant 32-bit unsigned integer
const GPIO_PUPD_NONE: u32 = 0x0;
const RCC_AHB2ENR_OFFSET: u32 = 0x4C;

// Macro which lets me convert the address to a into a mutable raw pointer 32-bit unsigned integer
macro_rules! mmio32 {
    ($addr:expr) => {
        unsafe { &mut *(($addr) as *mut u32) }
    };
}

fn reg_bit(base: u32, bit: u32) -> u32 {
    (base << 5) + bit
}

// RCC Peripheral Clock Enable Enum
#[repr(u32)]
enum RccPeriphClken {
    RCC_GPIOC = reg_bit(RCC_AHB2ENR_OFFSET, 2),
}

// Function Declarations
unsafe fn rcc_periph_clock_enable(clken: RccPeriphClken) {
    *mmio32!(RCC_BASE + (clken as u32 >> 5)) |= 1 << (clken as u32 & 0x1F);
}

unsafe fn gpio_mode_setup(gpioport: usize, mode: u8, pull_up_down: u8, gpios: u16) {
    let mut moder = *mmio32!(gpioport + 0x00);
    let mut pupd = *mmio32!(gpioport + 0x0C);

    for i in 0..16 {
        if (1 << i) & gpios == 0 {
            continue;
        }

        moder &= !((0x3 << (2 * i)) as u32);
        moder |= (mode as u32) << (2 * i);
        pupd &= !((0x3 << (2 * i)) as u32);
        pupd |= (pull_up_down as u32) << (2 * i);
    }

    *mmio32!(gpioport + 0x00) = moder;
    *mmio32!(gpioport + 0x0C) = pupd;
}

unsafe fn gpio_set(gpioport: usize, gpios: u16) {
    *mmio32!(gpioport + 0x18) = gpios as u32;
}

unsafe fn gpio_clear(gpioport: usize, gpios: u16) {
    *mmio32!(gpioport + 0x18) = (gpios as u32) << 16;
}

unsafe fn gpio_toggle(gpioport: usize, gpios: u16) {
    let port = *mmio32!(gpioport + 0x14);
    *mmio32!(gpioport + 0x18) = ((port & gpios as u32) << 16) | (!port & gpios as u32);
}

// Entry Point - Running the code
#[no_mangle] // Prevent the Rust compiler from mangling the name of a function while compiling 
pub extern "C" fn main() -> ! {
    unsafe {
        rcc_periph_clock_enable(RccPeriphClken::RCC_GPIOC);
        gpio_mode_setup(GPIOC, GPIO_MODE_OUTPUT as u8, GPIO_PUPD_NONE as u8, GPIO10 as u16);
        gpio_mode_setup(GPIOC, GPIO_MODE_OUTPUT as u8, GPIO_PUPD_NONE as u8, GPIO12 as u16);
        gpio_set(GPIOC, GPIO10 as u16);
        gpio_clear(GPIOC, GPIO12 as u16);
    }

    loop {
        for _ in 0..400_000 {
            // No operation (NOP)
            unsafe {
                core::arch::asm!("nop"); //allows you to write inline assembly (nop means no operation)
            }
        }
        unsafe {
            gpio_toggle(GPIOC, GPIO10 as u16);
            gpio_toggle(GPIOC, GPIO12 as u16);
        }
    }
}

