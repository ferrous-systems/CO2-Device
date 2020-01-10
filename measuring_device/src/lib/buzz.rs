// Provides definitions for our development board
use dwm1001::{
    DWM1001,
};

use nrf52832_hal::{
    prelude::*,
    twim::{self, Twim},
    pwm::{self, Pwm, WaveCounterMode, DecoderLoad, DecoderMode, Instance, Error},

};
//se crate::lib::types::{LEDs};

pub fn buzzer<T: Instance> (pulse: &mut Pwm<T>, co2: &f32) -> Result<(), Error> {


    static sequence: [u16; 4] = [1000, 15000, 10000, 0x3333];

    if co2 < &1000_f32 {
        pulse.stop_task();


    } else if co2 < &2000_f32 && co2 > &1000_f32 {
        pulse.stop_task();


    } else if co2 > &2000_f32 {
        pulse.set_decoder(DecoderLoad::Individual, DecoderMode::NextStep);
        pulse.disable_loop();
        pulse.set_wavecounter(WaveCounterMode::Up);
        pulse.set_sequence_0(sequence, 0, 0);
        pulse.start_sequence_0();
        //timer.delay(250_000);
        //pulse.stop_task();
    };

    Ok(())
}
