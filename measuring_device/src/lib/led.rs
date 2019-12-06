// Provides definitions for our development board
use dwm1001::{
    nrf52832_hal::{
        prelude::*,
        twim::{self, Twim},
        gpio::Level::{High, Low},
    },
    DWM1001,
};
use crate::lib::types::{LEDs};

pub fn traffic_light (mut leds: LEDs, co2: &f32) -> LEDs {
    // leds.red = leds.red.into_push_pull_output(Low);
    // leds.green = leds.green.into_push_pull_output(High);
    // leds.blue = leds.blue.into_push_pull_output(Low);

    if co2 < &1000_f32 {
        leds.red = leds.red.into_push_pull_output(Low);
        leds.green = leds.green.into_push_pull_output(High);
        leds.blue = leds.blue.into_push_pull_output(Low);
        // board.pins.SPIS_MOSI.into_push_pull_output(Low);
        // board.pins.SPIS_MISO.into_push_pull_output(High);
        // board.pins.SPIS_CLK.into_push_pull_output(Low);
    } else if co2 < &2000_f32 && co2 > &1000_f32 {
        leds.red = leds.red.into_push_pull_output(High);
        leds.green = leds.green.into_push_pull_output(High);
        leds.blue = leds.blue.into_push_pull_output(Low);
        // board.pins.SPIS_MOSI.into_push_pull_output(High);
        // board.pins.SPIS_MISO.into_push_pull_output(High);
        // board.pins.SPIS_CLK.into_push_pull_output(Low);
    } else if co2 > &2000_f32 {
        leds.red = leds.red.into_push_pull_output(High);
        leds.green = leds.green.into_push_pull_output(Low);
        leds.blue = leds.blue.into_push_pull_output(Low);
        // board.pins.SPIS_MOSI.into_push_pull_output(High);
        // board.pins.SPIS_MISO.into_push_pull_output(Low);
        // board.pins.SPIS_CLK.into_push_pull_output(Low);
    };

    leds
}
