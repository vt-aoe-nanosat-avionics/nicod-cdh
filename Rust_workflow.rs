/*
 * This file is based on part of the libopencm3 project.
 *
 * Copyright (C) 2010 Gareth McMullin <gareth@blacksphere.co.nz>
 * Copyright (C) 2018 Olivier 'reivilibre' <olivier@librepush.net>
 *
 * This library is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this library.  If not, see <http://www.gnu.org/licenses/>.
 */

#![no_std]
#![no_main]

#![feature(asm)]
#![feature(const_str_as_bytes)]

extern crate libopencm3_sys as cm3;
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_semihosting as semihosting;
extern crate panic_semihosting;

use cortex_m_rt::ExceptionFrame;

//use core::fmt::Write;
use core::ptr;
use core::mem::size_of;
use cm3::raw_c_types;

entry!(entrypoint);



const DEV_DESCR: cm3::usb_device_descriptor = cm3::usb_device_descriptor {
    //bLength: cm3::USB_DT_DEVICE_SIZE, not defined for some reason
    bLength: size_of::<cm3::usb_device_descriptor>() as u8,
    bDescriptorType: cm3::USB_DT_DEVICE as u8,
    bcdUSB: 0x0200,
    bDeviceClass: 0,
    bDeviceSubClass: 0,
    bDeviceProtocol: 0,
    bMaxPacketSize0: 64,
    idVendor: 0x0483,
    idProduct: 0x5710,
    bcdDevice: 0x0200,
    iManufacturer: 1,
    iProduct: 2,
    iSerialNumber: 3,
    bNumConfigurations: 1,
};

const HID_ENDPOINT: cm3::usb_endpoint_descriptor = cm3::usb_endpoint_descriptor {
    bLength: cm3::USB_DT_ENDPOINT_SIZE as u8,
    bDescriptorType: cm3::USB_DT_ENDPOINT as u8,
    bEndpointAddress: 0x81,
    bmAttributes: cm3::USB_ENDPOINT_ATTR_INTERRUPT as u8,
    wMaxPacketSize: 4,
    bInterval: 0x20,
    // descriptor ends here. This is used internally:
    extra: ptr::null(),
    extralen: 0,
};


#[repr(C, packed)]
struct HidReport {
    report_descriptor_type: u8,
    descriptor_length: u16,
}

#[repr(C, packed)]
struct HidFunction {
    hid_descriptor: cm3::usb_hid_descriptor,
    hid_report: HidReport,
}

const HID_FUNCTION: HidFunction = HidFunction {
    hid_descriptor: cm3::usb_hid_descriptor {
        bLength: size_of::<HidFunction>() as u8,
        bDescriptorType: cm3::USB_DT_HID as u8,
        bcdHID: 0x0100,
        bCountryCode: 0,
        bNumDescriptors: 1,
    },
    hid_report: HidReport {
        report_descriptor_type: cm3::USB_DT_REPORT as u8,
        /* unfortunately not const. size_of_val(&HID_REPORT_DESCRIPTOR) as u16 */
        descriptor_length: HID_REPORT_DESCRIPTOR_LENGTH as u16,
    },
};

const HID_REPORT_DESCRIPTOR_LENGTH: usize = 74;

const HID_IFACE: cm3::usb_interface_descriptor = cm3::usb_interface_descriptor {
    bLength: cm3::USB_DT_INTERFACE_SIZE as u8,
    bDescriptorType: cm3::USB_DT_INTERFACE as u8,
    bInterfaceNumber: 0,
    bAlternateSetting: 0,
    bNumEndpoints: 1,
    bInterfaceClass: cm3::USB_CLASS_HID as u8,
    bInterfaceSubClass: 1,
    /* boot */
    bInterfaceProtocol: 2,
    /* mouse */
    iInterface: 0,

    endpoint: &HID_ENDPOINT,

    extra: &HID_FUNCTION as *const HidFunction as *const raw_c_types::c_void,
    extralen: size_of::<HidFunction>() as i32,
};

const IFACES: [cm3::usb_interface; 1] = [
    cm3::usb_interface {
        num_altsetting: 1,
        altsetting: &HID_IFACE,
        cur_altsetting: ptr::null_mut(),
        iface_assoc: ptr::null(),
    },
];

const CONFIG: cm3::usb_config_descriptor = cm3::usb_config_descriptor {
    bLength: cm3::USB_DT_CONFIGURATION_SIZE as u8,
    bDescriptorType: cm3::USB_DT_CONFIGURATION as u8,
    wTotalLength: 0,
//#ifdef INCLUDE_DFU_INTERFACE
//.bNumInterfaces = 2,
//#else
    bNumInterfaces: 1,
//#endif
    bConfigurationValue: 1,
    iConfiguration: 0,
    bmAttributes: 0xC0,
    bMaxPower: 0x32,

    interface: &IFACES[0],
};


static mut USB_STRINGS: [&[u8]; 3] = [
    "Black Sphere Technologies\0".as_bytes(),
    "HID Demo\0".as_bytes(),
    "DEMO\0".as_bytes()
];

static mut USBD_CONTROL_BUFFER: [u8; 128] = [0; 128];

fn entrypoint() -> ! {
    unsafe {
        /*
        let mut stderr = semihosting::hio::hstderr().unwrap();
        writeln!(stderr, "Hello world!").unwrap();

        loop {}
        */


        cm3::rcc_clock_setup_in_hsi_out_48mhz();

// C's enums seem to be encoded from <variant name> to <enum name>_<variant name>
        cm3::rcc_periph_clock_enable(cm3::rcc_periph_clken_RCC_GPIOA);

        cm3::gpio_set_mode(cm3::GPIOA, cm3::GPIO_MODE_OUTPUT_2_MHZ as u8,
                           cm3::GPIO_CNF_OUTPUT_PUSHPULL as u8, cm3::GPIO12 as u16);

        cm3::gpio_clear(cm3::GPIOA, cm3::GPIO12 as u16);

        // THIS DOES NOT WORK IN DEBUG MODE
        for _i in 0..800000 {
            asm!("nop"); // not sure how well this will translate to Rust
        }

        USBD_DEV = cm3::usbd_init(
            &cm3::st_usbfs_v1_usb_driver,
            &DEV_DESCR,
            &CONFIG,
            &mut USB_STRINGS as *mut [&[u8]] as *mut *const i8,
            USB_STRINGS.len() as i32,
            &mut USBD_CONTROL_BUFFER as *mut [u8; 128] as *mut u8,
            USBD_CONTROL_BUFFER.len() as u16);

        cm3::usbd_register_set_config_callback(USBD_DEV, Some(hid_set_config));

        loop {
            cm3::usbd_poll(USBD_DEV);
        }
    }
}


static mut HID_REPORT_DESCRIPTOR: [u8; /*74*/ HID_REPORT_DESCRIPTOR_LENGTH] = [
    0x05, 0x01, /* USAGE_PAGE (Generic Desktop)         */
    0x09, 0x02, /* USAGE (Mouse)                        */
    0xa1, 0x01, /* COLLECTION (Application)             */
    0x09, 0x01, /*   USAGE (Pointer)                    */
    0xa1, 0x00, /*   COLLECTION (Physical)              */
    0x05, 0x09, /*     USAGE_PAGE (Button)              */
    0x19, 0x01, /*     USAGE_MINIMUM (Button 1)         */
    0x29, 0x03, /*     USAGE_MAXIMUM (Button 3)         */
    0x15, 0x00, /*     LOGICAL_MINIMUM (0)              */
    0x25, 0x01, /*     LOGICAL_MAXIMUM (1)              */
    0x95, 0x03, /*     REPORT_COUNT (3)                 */
    0x75, 0x01, /*     REPORT_SIZE (1)                  */
    0x81, 0x02, /*     INPUT (Data,Var,Abs)             */
    0x95, 0x01, /*     REPORT_COUNT (1)                 */
    0x75, 0x05, /*     REPORT_SIZE (5)                  */
    0x81, 0x01, /*     INPUT (Cnst,Ary,Abs)             */
    0x05, 0x01, /*     USAGE_PAGE (Generic Desktop)     */
    0x09, 0x30, /*     USAGE (X)                        */
    0x09, 0x31, /*     USAGE (Y)                        */
    0x09, 0x38, /*     USAGE (Wheel)                    */
    0x15, 0x81, /*     LOGICAL_MINIMUM (-127)           */
    0x25, 0x7f, /*     LOGICAL_MAXIMUM (127)            */
    0x75, 0x08, /*     REPORT_SIZE (8)                  */
    0x95, 0x03, /*     REPORT_COUNT (3)                 */
    0x81, 0x06, /*     INPUT (Data,Var,Rel)             */
    0xc0, /*         END_COLLECTION                     */
    0x09, 0x3c, /*   USAGE (Motion Wakeup)              */
    0x05, 0xff, /*   USAGE_PAGE (Vendor Defined Page 1) */
    0x09, 0x01, /*   USAGE (Vendor Usage 1)             */
    0x15, 0x00, /*   LOGICAL_MINIMUM (0)                */
    0x25, 0x01, /*   LOGICAL_MAXIMUM (1)                */
    0x75, 0x01, /*   REPORT_SIZE (1)                    */
    0x95, 0x02, /*   REPORT_COUNT (2)                   */
    0xb1, 0x22, /*   FEATURE (Data,Var,Abs,NPrf)        */
    0x75, 0x06, /*   REPORT_SIZE (6)                    */
    0x95, 0x01, /*   REPORT_COUNT (1)                   */
    0xb1, 0x01, /*   FEATURE (Cnst,Ary,Abs)             */
    0xc0, /*       END_COLLECTION                       */
];

unsafe extern "C" fn hid_control_request(_usbd_dev: *mut cm3::usbd_device,
                                         req: *mut cm3::usb_setup_data,
                                         buf: *mut *mut u8,
                                         len: *mut u16,
                                         _complete: *mut cm3::usbd_control_complete_callback) -> cm3::usbd_request_return_codes {
    if (*req).bmRequestType != 0x81 || (*req).bRequest != cm3::USB_REQ_GET_DESCRIPTOR as u8 || (*req).wValue != 0x2200 {
        return cm3::usbd_request_return_codes_USBD_REQ_NOTSUPP;
    }

    *buf = &mut HID_REPORT_DESCRIPTOR[0] as *mut u8;
    *len = HID_REPORT_DESCRIPTOR.len() as u16;

    return cm3::usbd_request_return_codes_USBD_REQ_HANDLED;
}

unsafe extern "C" fn hid_set_config(usbd_dev: *mut cm3::usbd_device, _w_value: u16) {
    cm3::usbd_ep_setup(usbd_dev, 0x81, cm3::USB_ENDPOINT_ATTR_INTERRUPT as u8, 4, None);

    cm3::usbd_register_control_callback(
        usbd_dev,
        cm3::USB_REQ_TYPE_STANDARD as u8 | cm3::USB_REQ_TYPE_INTERFACE as u8,
        cm3::USB_REQ_TYPE_TYPE as u8 | cm3::USB_REQ_TYPE_RECIPIENT as u8,
        Some(hid_control_request),
    );

    cm3::systick_set_clocksource(cm3::STK_CSR_CLKSOURCE_AHB_DIV8 as u8);
    /* SysTick interrupt every N clock pulses: set reload to N-1 */
    cm3::systick_set_reload(99999);
    cm3::systick_interrupt_enable();
    cm3::systick_counter_enable();
}

pub static mut USBD_DEV: *mut cm3::usbd_device = ptr::null_mut();

#[no_mangle]
pub extern "C" fn strlen(ptr: *const u8) -> i32 {
    let mut len = 0;
    unsafe {
        while *(ptr.offset(len)) != 0 {
            len += 1;
        }
    }

    // The C library function size_t strlen(const char *str) computes the length of the string
    // str up to, but not including the terminating null character.
    // — https://www.tutorialspoint.com/c_standard_library/c_function_strlen.htm
    // WRONG return len as i32 + 1;

    return len as i32;
}


// unlike in pure opencm3, I think cortex-m-rt might be generating the interrupt table
// MUST declare it using the exception syntax.


struct JigglerState {
    countdown: i16,
    velocity_x: i8,
    velocity_y: i8,
}

static mut JIGGLER_STATE: JigglerState = JigglerState {
    countdown: 42,
    velocity_x: 1,
    velocity_y: 0,
};

exception!(SysTick, sys_tick_handler);

//#[no_mangle]
//pub extern "C" fn sys_tick_handler() {
pub fn sys_tick_handler() {
    let mut buf: [u8; 4] = [0; 4];
    unsafe {
        // buf[1]: x, buf[2]: y
        buf[1] = JIGGLER_STATE.velocity_x as u8;
        buf[2] = JIGGLER_STATE.velocity_y as u8;


        JIGGLER_STATE.countdown -= 1;
        if JIGGLER_STATE.countdown <= 0 {
            let tmp = JIGGLER_STATE.velocity_x;
            JIGGLER_STATE.velocity_x = -JIGGLER_STATE.velocity_y;
            JIGGLER_STATE.velocity_y = tmp;
            JIGGLER_STATE.countdown = 42;
        }


// buf needs to be a void pointer.
// NOTE here: 4 == buf.len()
        cm3::usbd_ep_write_packet(USBD_DEV, 0x81, buf.as_ptr() as *const raw_c_types::c_void, 4);
    }
}




/* Choose this for a more authentic replication of the original C demo.
struct JigglerState {
    x: i16,
    direction: i8,
}

static mut JIGGLER_STATE: JigglerState = JigglerState {
    x: 0,
    direction: 1,
};

exception!(SysTick, sys_tick_handler);

//#[no_mangle]
//pub extern "C" fn sys_tick_handler() {
pub fn sys_tick_handler() {
    let mut buf: [u8; 4] = [0; 4];
    unsafe {
        buf[1] = JIGGLER_STATE.direction as u8;
        JIGGLER_STATE.x += JIGGLER_STATE.direction as i16;
        if JIGGLER_STATE.x > 30 || JIGGLER_STATE.x < -30 {
            JIGGLER_STATE.direction = -JIGGLER_STATE.direction;
        }


// buf needs to be a void pointer.
// NOTE here: 4 == buf.len()
        cm3::usbd_ep_write_packet(USBD_DEV, 0x81, buf.as_ptr() as *const raw_c_types::c_void, 4);
    }
}
*/

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
