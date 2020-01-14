// Provides definitions for our development board
use dwm1001::{
    nrf52832_hal::{
        prelude::*,
        twim::{self, Twim},
        gpio::Level::{High, Low},
    },
    DWM1001,
};
use embedded_graphics::{
    coord::Coord,
    fonts::{Font12x16, Font6x8,
        font_builder::{FontBuilder, FontBuilderConf}},
    prelude::*,
    primitives::{Circle, Line},
    Drawing,
};
use embedded_hal::prelude::*;
use epd_waveshare::{
    epd4in2::{Display4in2, EPD4in2},
    graphics::{Display, DisplayRotation},
    prelude::*,
};
use crate::lib::types::{LEDs};

#[derive(Debug, Copy, Clone)]
pub enum Font24x32Conf {}
impl FontBuilderConf for Font24x32Conf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("link to file");
    const CHAR_HEIGHT: u32 = 48;
    const CHAR_WIDTH: u32 = 32;
    const FONT_IMAGE_WIDTH: u32 = 32;
    fn char_offset(c: char) -> u32 {
        let fallback = '?' as u32 - ' ' as u32;
        if c < ' ' {
            return fallback;
        }
        if c <= '~' {
            return c as u32 - ' ' as u32;
        }
        fallback
    }
}

pub type Font24x32<'a, C> = FontBuilder<'a, C, Font24x32Conf>;
