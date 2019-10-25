use dwm1001::nrf52832_hal::{
    prelude::*,
    twim::{Error, Twim},
};

use crc_all::Crc;

pub struct SensorData {
    pub co2: f32,
    pub temperature: f32,
    pub humidity: f32,
}

pub fn start_measuring<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<(), Error> {
    let pressure = 0_u16;

    let mut wr_buffer = [0u8; 5];
    wr_buffer.copy_from_slice(&[0x00, 0x10, 0x00, 0x00, 0x00]);

    let mut crc = Crc::<u8>::new(0x31, 8, 0xff, 0x00, false);

    crc.update(&pressure.to_be_bytes());
    wr_buffer[4] = crc.finish();

    i2c.write(address, &wr_buffer).unwrap();

    Ok(())
}

pub fn data_ready<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<bool, Error> {
    let mut wr_buffer = [0u8; 2];
    wr_buffer.copy_from_slice(&[0x02, 0x02]);
    let mut rd_buffer = [0u8; 3];

    i2c.write(address, &wr_buffer).unwrap();
    i2c.read(address, &mut rd_buffer).unwrap();

    Ok(u16::from_be_bytes([rd_buffer[0], rd_buffer[1]]) == 1)
}

pub fn get_measurement<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<SensorData, Error> {
    let mut wr_buffer = [0u8; 2];
    wr_buffer.copy_from_slice(&[0x03, 0x00]);
    let mut rd_buffer = [0u8; 18];

    i2c.write(address, &wr_buffer).unwrap();
    i2c.read(address, &mut rd_buffer).unwrap();

    let data = SensorData {
        co2: f32::from_bits(u32::from_be_bytes([
            rd_buffer[0],
            rd_buffer[1],
            rd_buffer[3],
            rd_buffer[4],
        ])),
        temperature: f32::from_bits(u32::from_be_bytes([
            rd_buffer[6],
            rd_buffer[7],
            rd_buffer[9],
            rd_buffer[10],
        ])),
        humidity: f32::from_bits(u32::from_be_bytes([
            rd_buffer[12],
            rd_buffer[13],
            rd_buffer[15],
            rd_buffer[16],
        ])),
    };

    Ok(data)
}

pub fn activate_self_calibration<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<(), Error> {
    let mut wr_buffer = [0u8; 4];
    wr_buffer.copy_from_slice(&[0x53, 0x06, 0x00, 0x00]);

    i2c.write(address, &wr_buffer).unwrap();

    Ok(())
}

pub fn get_self_calibration_status<T: TwimExt>(
    address: u8,
    i2c: &mut Twim<T>,
) -> Result<bool, Error> {
    let mut wr_buffer = [0u8; 2];
    wr_buffer.copy_from_slice(&[0x53, 0x06]);
    let mut rd_buffer = [0u8; 3];

    i2c.write(address, &wr_buffer).unwrap();
    i2c.read(address, &mut rd_buffer).unwrap();

    Ok(u16::from_be_bytes([rd_buffer[0], rd_buffer[1]]) == 1)
}

pub fn get_frc_value<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<u16, Error> {
    let mut wr_buffer = [0u8; 2];
    wr_buffer.copy_from_slice(&[0x52, 0x04]);
    let mut rd_buffer = [0u8; 2];

    i2c.write(address, &wr_buffer).unwrap();
    i2c.read(address, &mut rd_buffer).unwrap();

    Ok(u16::from_be_bytes(rd_buffer))
}

pub fn get_measurement_interval<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<u16, Error> {
    let mut wr_buffer = [0u8; 2];
    wr_buffer.copy_from_slice(&[0x46, 0x00]);
    let mut rd_buffer = [0u8; 2];

    i2c.write(address, &wr_buffer).unwrap();
    i2c.read(address, &mut rd_buffer).unwrap();

    Ok(u16::from_be_bytes(rd_buffer))
}

pub fn get_firmware<T: TwimExt>(address: u8, i2c: &mut Twim<T>) -> Result<u16, Error> {
    let mut wr_buffer = [0u8; 2];
    wr_buffer.copy_from_slice(&[0xd1, 0x00]);
    let mut rd_buffer = [0u8; 2];

    i2c.write(address, &wr_buffer).unwrap();
    i2c.read(address, &mut rd_buffer).unwrap();

    Ok(u16::from_be_bytes(rd_buffer))
}
