//! HAL interface to the PWM peripheral
use core::mem::size_of;
use core::ops::Deref;
use core::sync::atomic::{compiler_fence, Ordering::SeqCst};



use crate::target::{
    pwm0,
    P0,
    PWM0,
};

#[cfg(any(feature = "52832", feature = "52840"))]
// use crate::target::PWM0;

use crate::{
    gpio::{Output, Pin, PushPull, Input, Floating},
    // slice_in_ram_or,
    target_constants::EASY_DMA_SIZE,
};

pub use pwm0::prescaler::PRESCALERW as Prescaler;

/// Interface to a PWM instance

pub struct Pwm<T>(T);



impl<T> Pwm<T>
where
    T: Instance,
{
    pub fn new(pwm: T, channels: Channels, prescaler: Prescaler) -> Self {

        for &pin in &[channels.pwm_ch0.pin, channels.pwm_ch1.pin, channels.pwm_ch2.pin] {
            unsafe { &*P0::ptr() }.pin_cnf[pin as usize].write(|w|
                w
                    .dir().output()
                    .input().connect()
                    .pull().disabled()
                    .drive().s0s1()
                    .sense().disabled()
            );
        }
        //Duty cycles
        //let pwm_seq0: [u16; 4] = [20, 30, 50, 100];
        //static mut pwm_seq0: [u16; 4] = [9000, 15000, 10000, 0x3333];

        // Select pins
        pwm.psel.out[0].write(|w| {
            let w = unsafe { w.pin().bits(channels.pwm_ch0.pin) };
            w.connect().connected()
        });

        pwm.psel.out[1].write(|w| {
            let w = unsafe { w.pin().bits(channels.pwm_ch1.pin) };
            w.connect().connected()
        });

        pwm.psel.out[2].write(|w| {
            let w = unsafe { w.pin().bits(channels.pwm_ch2.pin) };
            w.connect().connected()
        });

        // pwm.psel.out[3].write(|w| {
        //     let w = unsafe { w.pin().bits(channels.pwm_ch3.pin) };
        //     w.connect().connected()
        // });


        // Enable PWM instance
        pwm.enable.write(|w| w.enable().enabled());

        // Operating Mode of wavecounter
        // pwm.mode.write(|w| w.updown().up());

        // Configure frequency
        pwm.prescaler.write(|w| w.prescaler().variant(prescaler));

        //Countertop
        unsafe {
        pwm.countertop.write(|w| w.countertop().bits(16000_u16));
        }

        //loop disable
        // pwm.loop_.write (|w| w.cnt().disabled());

        // Decoder
        // pwm.decoder.write (|w| w.load().individual().mode().refresh_count());

        // sequence
        // unsafe {
        // pwm.seq0.ptr.write (|w| w.ptr().bits(pwm_seq0.as_ptr() as u32));
        // pwm.seq0.cnt.write (|w| w.cnt().bits((size_of::<[u16; 4]>() / size_of::<u16>()) as u16));
        //
        // pwm.seq0.refresh.write (|w| w.cnt().bits(0_u32));
        // pwm.seq0.enddelay.write (|w| w.cnt().bits(0_u32));
        //
        // // pwm.seq1.ptr.write (|w| w.ptr().bits(pwm_seq1.as_ptr() as u32));
        // // pwm.seq1.cnt.write (|w| w.cnt().bits((size_of::<[u16; 4]>() / size_of::<u16>()) as u16));
        // // pwm.seq1.refresh.write (|w| w.cnt().bits(0_u32));
        // // pwm.seq1.enddelay.write (|w| w.cnt().bits(0_u32));
        //
        // pwm.tasks_seqstart[0].write (|w| w.bits(1_u32));
        // }


        Pwm(pwm)
    }

    pub fn set_countertop(&mut self, countertop: u16) {
        //Countertop
        unsafe {
        self.0.countertop.write(|w| w.countertop().bits(countertop));
        }
    }

    pub fn set_wavecounter(&mut self, mode: WaveCounterMode) {

        match mode {
            WaveCounterMode::Up => self.0.mode.write(|w| w.updown().up()),
            WaveCounterMode::UpAndDown => self.0.mode.write(|w| w.updown().up_and_down()),
        }
    }

    pub fn set_decoder(&mut self, load: DecoderLoad, mode: DecoderMode) {
        match mode {
            DecoderMode::NextStep => {
                match load {
                    DecoderLoad::Common => self.0.decoder.write (|w| w.load().common().mode().next_step()),
                    DecoderLoad::Grouped => self.0.decoder.write (|w| w.load().grouped().mode().next_step()),
                    DecoderLoad::Individual => self.0.decoder.write (|w| w.load().individual().mode().next_step()),
                    DecoderLoad::WaveForm => self.0.decoder.write (|w| w.load().wave_form().mode().next_step()),
                }
            } DecoderMode::RefreshCount => {
                match load {
                    DecoderLoad::Common => self.0.decoder.write (|w| w.load().common().mode().refresh_count()),
                    DecoderLoad::Grouped => self.0.decoder.write (|w| w.load().grouped().mode().refresh_count()),
                    DecoderLoad::Individual => self.0.decoder.write (|w| w.load().individual().mode().refresh_count()),
                    DecoderLoad::WaveForm => self.0.decoder.write (|w| w.load().wave_form().mode().refresh_count()),
                }
            }
        }
    }

    pub fn disable_loop (&mut self) {
        self.0.loop_.write (|w| w.cnt().disabled());
    }

    pub fn set_sequence_0(&mut self, sequence: [u16; 4], refresh_cnt: u32, enddelay_cnt: u32) {

        static mut seq:[u16; 4] = [9000, 15000, 10000, 0x3333];

        unsafe {
        self.0.seq0.ptr.write (|w| w.ptr().bits(seq.as_ptr() as u32));
        self.0.seq0.cnt.write (|w| w.cnt().bits((size_of::<[u16; 4]>() / size_of::<u16>()) as u16));

        self.0.seq0.refresh.write (|w| w.cnt().bits(refresh_cnt));
        self.0.seq0.enddelay.write (|w| w.cnt().bits(enddelay_cnt));
        }

    }

    pub fn set_sequence_1(&mut self, sequence: [u16; 4], refresh_cnt: u32, enddelay_cnt: u32) {

        static mut seq:[u16; 4] = [9000, 15000, 10000, 0x3333];

        unsafe {
        self.0.seq1.ptr.write (|w| w.ptr().bits(seq.as_ptr() as u32));
        self.0.seq1.cnt.write (|w| w.cnt().bits((size_of::<[u16; 4]>() / size_of::<u16>()) as u16));

        self.0.seq1.refresh.write (|w| w.cnt().bits(refresh_cnt));
        self.0.seq1.enddelay.write (|w| w.cnt().bits(enddelay_cnt));
        }

    }

    pub fn start_sequence_0(&mut self) {

        unsafe {
            self.0.tasks_seqstart[0].write (|w| w.bits(1_u32));
        }
    }

    pub fn stop_task(&mut self) {

        unsafe {
            self.0.tasks_stop.write (|w| w.bits(1_u32));
        }
    }


}

/// The pins used by the pwm peripheral
///
/// Currently, only P0 pins are supported.
pub struct Channels {

    pub pwm_ch0: Pin<Input<Floating>>,
    pub pwm_ch1: Pin<Input<Floating>>,
    pub pwm_ch2: Pin<Input<Floating>>,
    //pub pwm_ch3: Pin<Input<Floating>>,

}

pub enum WaveCounterMode {
    Up,
    UpAndDown,
}

pub enum DecoderLoad {
    Common,
    Grouped,
    Individual,
    WaveForm,
}

pub enum DecoderMode {
    RefreshCount,
    NextStep,
}

//Change
#[derive(Debug)]
pub enum Error {
    TxBufferTooLong,
    RxBufferTooLong,
    Transmit,
    Receive,
    DMABufferNotInDataMemory,
}

/// Implemented by all PWMS instances
pub trait Instance: Deref<Target = pwm0::RegisterBlock> {}

impl Instance for PWM0 {}
