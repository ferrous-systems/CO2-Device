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
        spim::{self, Spim},
        gpio::Level::{High, Low},
        delay,
    },
    DWM1001,
};

use embedded_graphics::{
    coord::Coord,
    fonts::{Font12x16, Font6x8},
    prelude::*,
    primitives::{Circle, Line, Triangle },
    Drawing,
};

use embedded_hal as hal;
use hal::blocking::delay::DelayMs;
use epd_waveshare::{
    epd4in2::{Display4in2, EPD4in2},
    graphics::{Display, DisplayRotation},
    prelude::*,
};

pub mod lib;




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
    // let scl = board.pins.GPIO_8.into_floating_input().degrade();
    // let sda = board.pins.GPIO_15.into_floating_input().degrade();
    //
    // let pins = twim::Pins { scl, sda };
    // let mut i2c = Twim::new(board.TWIM0, pins, twim::Frequency::K100);

    // configure SPI

    let cs = board.pins.SPIS_MISO.into_push_pull_output(Low);
    let din = board.pins.SPIS_MOSI.into_push_pull_output(Low).degrade();
    let clk = board.pins.SPIS_CLK.into_push_pull_output(Low).degrade();
    let dc = board.pins.READY.into_push_pull_output(Low);
    let rst = board.pins.GPIO_8.into_push_pull_output(Low);
    let busy = board.pins.RESETn.into_floating_input();

    let pins = spim::Pins {
        sck: clk,
        miso: None,
        mosi: Some(din),
    };


    let mut spi = Spim::new(board.SPIM0, pins, spim::Frequency::K500, spim::MODE_0, 0);

    // instantiate ePaper
    // struct Delay;
    // impl hal::blocking::delay::DelayMs<u8> for Delay {
    //
    //     fn delay_ms(&mut self, n: u8) {
    //
    //         unimplemented!()
    //         // |n| {
    //         //     timer.delay(n)
    //         // }
    //     }
    // };

    let mut delay = delay::Delay::new(board.SYST);
    let mut epd4in2 = EPD4in2::new(&mut spi, cs, busy, dc, rst, &mut delay).expect("eink initalize error");

    let mut display = Display4in2::default();

    let c1 = Circle::new(Coord::new(150, 50), 30)
        .with_stroke(Some(Color::Black))
        .with_fill(Some(Color::Black))
        .into_iter();

    let c2 = Circle::new(Coord::new(210, 50), 30)
        .with_stroke(Some(Color::Black))
        .with_fill(Some(Color::Black))
        .into_iter();

    let t1 = Triangle::new(Coord::new(239, 60), Coord::new(121, 60), Coord::new(180, 140))
        .with_stroke(Some(Color::Black))
        .with_stroke_width(2)
        .with_fill(Some(Color::Black))
        .into_iter();



    display.draw(c2);
    display.draw(c1);
    display.draw(t1);



    epd4in2.update_frame(&mut spi, &display.buffer()).unwrap();
    epd4in2
        .display_frame(&mut spi)
        .expect("display frame new graphics");
    delay.delay_ms(5000u16);

    // timer.delay(2_000_000);

    // send command to the sensor
    // lib::start_measuring(address, &mut i2c).unwrap();

    'ready: loop {
    //     // blink red LED for not ready status
    //     timer.delay(2_000_000);
    //     board.leds.D11.enable();
    //     timer.delay(250_000);
    //     board.leds.D11.disable();
    //
    //     let measurement_status = lib::data_ready(address, &mut i2c).unwrap();
    //     if measurement_status == true {
    //         write!(&mut s, "Measurement ready. \r\n").unwrap();
    //         board.uart.write(s.as_bytes()).unwrap();
    //
    //         // green LED for ready status
    //         board.leds.D9.enable();
    //         timer.delay(600_000);
    //         board.leds.D9.disable();
    //         break 'ready;
    //     }
    // }
    //
    let mut toggle = false;
    //
    // 'measuring: loop {
    //     s.clear();
    //
    //     // send command to get measurement
    //     // receives floats from bytes
    //     let result = lib::get_measurement(address, &mut i2c).unwrap();
    //
    //     //Basic LED alert
    //     //leds = led::traffic_light(leds, &result.co2);
    //
    //
    //
    //     let co2 = result.co2;
    //     let temp = result.temperature;
    //     let humidity = result.humidity;
    //
    //     write!(
    //         &mut s,
    //         "CO2 {:.2} ppm \r\nTemperature {:.2} C \r\nHumidity {:.2} % \r\n\r\n",
    //         co2, temp, humidity
    //     )
    //
    //     .unwrap();
    //     board.uart.write(s.as_bytes()).unwrap();

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
