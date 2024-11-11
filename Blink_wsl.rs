#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;


// Constants
const PERIPH_BASE: u32 = 0x4000_0000; //Constant means immutable (Cannot be changed)
const PERIPH_BASE_AHB2: u32 = 0x4800_0000; // Usize represents an unsigned integer that’s the same size as the platform’s pointer size
const PERIPH_BASE_AHB1: u32 = PERIPH_BASE + 0x20000;
const GPIO_PORT_C_BASE: u32 = PERIPH_BASE_AHB2 + 0x0800;
const GPIOC: u32 = GPIO_PORT_C_BASE;
const RCC_BASE: u32 = PERIPH_BASE_AHB1 + 0x1000;
const GPIO10: u32 = 1 << 10; // This is a left bit shift operation that moves 1, 10 bits to the left
const GPIO12: u32 = 1 << 12;
const GPIO_MODE_OUTPUT: u32 = 0x1;// u32 means it's a constant 32-bit unsigned integer
const GPIO_PUPD_NONE: u32 = 0x0;
const RCC_AHB2ENR_OFFSET: u32 = 0x4C;
const RCC_GPIOC: u32 = ((RCC_AHB2ENR_OFFSET) << 5) + (2);

// Macro which lets me convert the address to a into a mutable raw pointer 32-bit unsigned integer
macro_rules! mmio32 {
    ($addr:expr) => {
        unsafe { &mut *(($addr) as *mut u32) }
    };
}

//This is unused
 fn reg_bit(base: u32, bit: u32) -> u32 {
     (base << 5) + bit
}


// Function Declarations
unsafe fn rcc_periph_clock_enable(clken: u32) {
    *mmio32!(RCC_BASE + (clken as u32 >> 5)) |= 1 << (clken as u32 & 0x1F);
}

unsafe fn gpio_mode_setup(gpioport: u32, mode: u8, pull_up_down: u8, gpios: u16) {
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

    *mmio32!(gpioport + 0x00) = moder; //The * dereferences the mutable pointer returned by the mmio32! macro. Accessing the memory at the address and treating it as if it were a u32 value.
    *mmio32!(gpioport + 0x0C) = pupd;
}

unsafe fn gpio_set(gpioport: u32, gpios: u16) {
    *mmio32!(gpioport + 0x18) = gpios as u32;
}

unsafe fn gpio_clear(gpioport: u32, gpios: u16) {
    *mmio32!(gpioport + 0x18) = (gpios as u32) << 16;
}

unsafe fn gpio_toggle(gpioport: u32, gpios: u16) {
    let port = *mmio32!(gpioport + 0x14);
    *mmio32!(gpioport + 0x18) = ((port & gpios as u32) << 16) | (!port & gpios as u32);
}


// Entry Point - Running the code
// Prevent the Rust compiler from mangling the name of a function while compiling
#[entry]
fn main() -> ! {
    unsafe {
        rcc_periph_clock_enable(RCC_GPIOC);
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
