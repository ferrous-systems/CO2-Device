#![no_std]
#![no_main]


// Panic provider crate
use panic_halt as _;

// String formatting
use core::fmt::Write as writefmt;
use heapless::String as HString;

// Used to set the program entry point
use cortex_m_rt::entry;

// Provides definitions for our development board
use dwm1001::{
    embedded_hal::blocking::i2c::{Read, Write},
    nrf52832_hal::{
        twim::{self, Twim},
        gpio::*,
        prelude::*,
    },
    DWM1001,
};

use scd30::scd30::{Scd30, Measurement};


#[entry]
fn main() -> ! {
    let mut board  = DWM1001::take().unwrap();
    let mut timer  = board.TIMER0.constrain();

    let mut s: HString<heapless::consts::U1024> = HString::new();
    let address = 0x61;

    let scl = board.pins.GPIO_8.into_floating_input().degrade();
    let sda = board.pins.GPIO_15.into_floating_input().degrade();

    let pins = twim::Pins { scl, sda };

    let i2c = Twim::new(board.TWIM0, pins, twim::Frequency::K100);

    let mut scd = Scd30::new_with_address(i2c, address);




    let mut toggle = false;

    loop {
        s.clear();
        // scd.start_measuring();
        // let data = scd.read();



        //
        // match data {
        //     Ok(datas) => {
        //         match datas {
        //             Some(i) => {
        //                 let co2 = i.co2;
        //                 let value = co2.to_bits();
        //
        //
        //                 write!(&mut s, "{:?}, \n", value).unwrap();
        //                 board.uart.write(s.as_bytes()).unwrap();
        //             }
        //
        //
        //             None => {
        //                 write!(&mut s, "0, \n").unwrap();
        //                 board.uart.write(s.as_bytes()).unwrap();;
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         write!(&mut s, "{:?}, \n", e).unwrap();
        //         board.uart.write(s.as_bytes()).unwrap();
        //     }
        // }

        write!(&mut s, "1\n").unwrap();
        board.uart.write(s.as_bytes()).unwrap();

        // board.leds.D9  - Top LED GREEN
        // board.leds.D12 - Top LED RED
        // board.leds.D11 - Bottom LED RED
        // board.leds.D10 - Bottom LED BLUE
        if toggle {
            board.leds.D10.enable();
        } else {
            board.leds.D10.disable();
        }

        toggle = !toggle;

        timer.delay(250_000);
    }

}
