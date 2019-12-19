// Provides definitions for our development board
use dwm1001::{
    DWM1001,
};

use nrf52832_hal::{
    prelude::*,
    twim::{self, Twim},
    pwm::{self, Pwm, WaveCounterMode, DecoderLoad, DecoderMode},

};
//se crate::lib::types::{LEDs};

pub fn buzzer (mut pulse: Pwm, co2: &f32) -> Pwm {

    static sequence: [u16; 4] = [9000, 15000, 10000, 0x3333];

    if co2 < &1000_f32 {
        pulse.stop_task();


    } else if co2 < &2000_f32 && co2 > &1000_f32 {
        pulse.stop_task();


    } else if co2 > &2000_f32 {
        pulse.set_decoder(DecoderLoad::Individual, DecoderMode::RefreshCount)
            .set_wavecounter(WaveCounterMode::Up)
            .disable_loop()
            .set_sequence_0(sequence, 0, 0)
            .start_sequence_0();
    };

    pulse
}
