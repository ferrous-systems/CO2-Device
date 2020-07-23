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
    nrf52832_hal::{
        prelude::*,
        twim::{self, Twim},
        gpio::Level::{Low},
    },
    DWM1001,
};


pub mod lib;
use crate::lib::led;
use crate::lib::types::{LEDs};


#[entry]
fn main() -> ! {
    // instanciate board and timer
    let mut board = DWM1001::take().unwrap();
    let mut timer = board.TIMER0.constrain();

    // let mut pwm = board.PWM0.enable();

    // empty heapless string for serial output
    let mut s: HString<heapless::consts::U1024> = HString::new();

    // address of the sensor
    let address = 0x61;

    // instanciate I2C
    let scl = board.pins.GPIO_8.into_floating_input().degrade();
    let sda = board.pins.GPIO_15.into_floating_input().degrade();

    let pins = twim::Pins { scl, sda };
    let mut i2c = Twim::new(board.TWIM0, pins, twim::Frequency::K100);

    let mut leds = LEDs {
        red: board.pins.SPIS_MOSI.into_push_pull_output(Low),
        green: board.pins.SPIS_MISO.into_push_pull_output(Low),
        blue: board.pins.SPIS_CLK.into_push_pull_output(Low),
    };

    timer.delay(2_000_000);

    // send command to the sensor
    lib::start_measuring(address, &mut i2c).unwrap();

    'ready: loop {
        // blink red LED for not ready status
        timer.delay(2_000_000);
        board.leds.D11.enable();
        timer.delay(250_000);
        board.leds.D11.disable();

        let measurement_status = lib::data_ready(address, &mut i2c).unwrap();
        if measurement_status == true {
            write!(&mut s, "Measurement ready. \r\n").unwrap();
            board.uart.write(s.as_bytes()).unwrap();

            // green LED for ready status
            board.leds.D9.enable();
            timer.delay(600_000);
            board.leds.D9.disable();
            break 'ready;
        }
    }

    let mut toggle = false;

    'measuring: loop {
        s.clear();

        // send command to get measurement
        // receives floats from bytes
        let result = lib::get_measurement(address, &mut i2c).unwrap();

        //Basic LED alert
        leds = led::traffic_light(leds, &result.co2);



        let co2 = result.co2;
        let temp = result.temperature;
        let humidity = result.humidity;

        write!(
            &mut s,
            "CO2 {:.2} ppm \r\nTemperature {:.2} C \r\nHumidity {:.2} % \r\n\r\n",
            co2, temp, humidity
        )

        .unwrap();
        board.uart.write(s.as_bytes()).unwrap();

        // board.leds.D9  - Top LED GREEN
        // board.leds.D12 - Top LED RED
        // board.leds.D11 - Bottom LED RED
        // board.leds.D10 - Bottom LED BLUE
        if toggle {
            board.leds.D9.enable();
        } else {
            board.leds.D9.disable();
        }

        toggle = !toggle;
        timer.delay(4000_000);
    }
}
