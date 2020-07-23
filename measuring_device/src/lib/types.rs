use dwm1001::{
    nrf52832_hal::{
        gpio::{Output, PushPull, p0::{P0_06, P0_07, P0_04},},
    },
};


pub struct LEDs {
    pub red: P0_06<Output<PushPull>>,
    pub green: P0_07<Output<PushPull>>,
    pub blue: P0_04<Output<PushPull>>,

}
