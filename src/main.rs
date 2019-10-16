#![no_std]
#![no_main]


// Panic provider crate
use heapless::consts::U8;
use panic_halt as _;

// String formatting
use core::fmt::Write as writefmt;
use heapless::String as HString;
use heapless::Vec as HVec;

use crc_all::Crc;

// Used to set the program entry point
use cortex_m_rt::entry;

// Provides definitions for our development board
use dwm1001::{
    embedded_hal::blocking::i2c::{Read, Write},
    nrf52832_hal::{
        twim::{self, Twim, Error},
        gpio::*,
        prelude::*,
    },
    DWM1001,
};

use scd30::scd30::{Scd30, Measurement};

// fn add_argument(rd_buffer: [u8; 3], argument: &[u8]) -> Result<(), ()> {
//
// }

fn data_ready<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<bool, Error> {

    let mut wr_buffer = [0u8; 2];
    wr_buffer.copy_from_slice(&[0x02, 0x02]);
    let mut rd_buffer = [0u8; 3];

    i2c.write(address, &wr_buffer).unwrap();
    i2c.read(address, &mut rd_buffer).unwrap();

    Ok(u16::from_be_bytes([rd_buffer[0], rd_buffer[1]]) == 1)

}

fn activate_self_calibration<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<(), Error> {

    let mut wr_buffer = [0u8; 4];
    wr_buffer.copy_from_slice(&[0x53, 0x06, 0x00, 0x00]);

    i2c.write(address, &wr_buffer).unwrap();


    Ok(())

}

fn get_self_calibration_status<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<bool, Error> {

    let mut wr_buffer = [0u8; 2];
    wr_buffer.copy_from_slice(&[0x53, 0x06]);
    let mut rd_buffer = [0u8; 3];

    i2c.write(address, &wr_buffer).unwrap();
    i2c.read(address, &mut rd_buffer).unwrap();

    Ok(u16::from_be_bytes([rd_buffer[0], rd_buffer[1]]) == 1)

}

fn start_measuring<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<(), Error> {

    let pressure = 0_u16;

    let mut wr_buffer = [0u8; 5];
    wr_buffer.copy_from_slice(&[0x00, 0x10, 0x00, 0x00, 0x00]);

    let mut crc = Crc::<u8>::new(0x31, 8, 0xff, 0x00, false);

    crc.update(&pressure.to_be_bytes());
    wr_buffer[4] = crc.finish();

    i2c.write(address, &wr_buffer).unwrap();

    Ok(())

}

fn get_frc_value<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<u16, Error> {

    let mut wr_buffer = [0u8; 2];
    wr_buffer.copy_from_slice(&[0x52, 0x04]);
    let mut rd_buffer = [0u8; 2];


    i2c.write(address, &wr_buffer).unwrap();
    i2c.read(address, &mut rd_buffer).unwrap();

    Ok(u16::from_be_bytes(rd_buffer))

}

fn get_measurement<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<f32, Error> {

    let mut wr_buffer = [0u8; 2];
    wr_buffer.copy_from_slice(&[0x03, 0x00]);
    let mut rd_buffer = [0u8; 18];


    i2c.write(address, &wr_buffer).unwrap();
    i2c.read(address, &mut rd_buffer).unwrap();

    Ok(u32::from_be_bytes([rd_buffer[0],rd_buffer[1], rd_buffer[3], rd_buffer[4]]) as f32)

}

fn get_measurement_interval<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<u16, Error> {

    let mut wr_buffer = [0u8; 2];
    wr_buffer.copy_from_slice(&[0x46, 0x00]);
    let mut rd_buffer = [0u8; 2];


    i2c.write(address, &wr_buffer).unwrap();
    i2c.read(address, &mut rd_buffer).unwrap();

    Ok(u16::from_be_bytes(rd_buffer))



}
fn get_firmware<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<u16, Error> {

    let mut wr_buffer = [0u8; 2];
    wr_buffer.copy_from_slice(&[0xd1, 0x00]);
    let mut rd_buffer = [0u8; 2];


    i2c.write(address, &wr_buffer).unwrap();
    i2c.read(address, &mut rd_buffer).unwrap();

    Ok(u16::from_be_bytes(rd_buffer))



}



#[entry]
fn main() -> ! {
    let mut board  = DWM1001::take().unwrap();
    let mut timer  = board.TIMER0.constrain();

    let mut s: HString<heapless::consts::U1024> = HString::new();

    let address = 0x61;


    let scl = board.pins.GPIO_8.into_floating_input().degrade();
    let sda = board.pins.GPIO_15.into_floating_input().degrade();

    let pins = twim::Pins { scl, sda };



    let mut i2c = Twim::new(board.TWIM0, pins, twim::Frequency::K100);
    timer.delay(2_000_000);



    start_measuring(address, &mut i2c).unwrap();
    timer.delay(2_000_000);

    //activate_self_calibration(address, &mut i2c);


    'ready: loop {
        timer.delay(2_000_000);
        board.leds.D11.enable();
        timer.delay(250_000);
        board.leds.D11.disable();

        // let status = get_self_calibration_status(address, &mut i2c).unwrap();
        let result = data_ready(address, &mut i2c).unwrap();
        if result == true {
            board.leds.D9.enable();
            timer.delay(600_000);
            board.leds.D9.disable();
            break 'ready;
        }

    }

    let result = get_measurement(address, &mut i2c);

    // write!(&mut s, "ok {:?} \n", result).unwrap();
    // board.uart.write(s.as_bytes()).unwrap();





    //let mut scd = Scd30::new_with_address(i2c, address);
    // match bool {
    //     Ok(a) => {
    //         write!(&mut s, "ok {:?} \n", rd_buffer).unwrap();
    //         board.uart.write(s.as_bytes()).unwrap();
    //         //Debug blink bottom red LED
    //         board.leds.D11.enable();
    //         timer.delay(600_000);
    //         board.leds.D11.disable();
    //
    //     }
    //
    //     Err(e) => {
    //         write!(&mut s, "err {:?} \n", e).unwrap();
    //         board.uart.write(s.as_bytes()).unwrap();
    //     }
    // }



    let mut toggle = false;

    loop {
        //Debug blink bottom red LED
        board.leds.D11.enable();
        timer.delay(250_000);
        board.leds.D11.disable();

        s.clear();


        // scd.soft_reset().unwrap();

        //Debug blink top green LED
        // board.leds.D9.enable();
        // timer.delay(250_000);
        // board.leds.D9.disable();
        //
        // scd.start_measuring().unwrap();
        // let data = scd.read();
        //
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

        // write!(&mut s, "1\n").unwrap();
        // board.uart.write(s.as_bytes()).unwrap();

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
