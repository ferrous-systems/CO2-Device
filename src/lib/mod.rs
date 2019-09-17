use dwm1001::{
    embedded_hal::blocking::i2c::{Read, Write},
    nrf52832_hal::{
        prelude::*,
    },
    DWM1001,
};
use heapless::{Vec, ArrayLength};
use heapless::consts::*;
use crc_all::Crc;

pub enum Command {
    StartContinuousMeasurement  = 0x0010,
    StopContinuousMeasurement   = 0x0104,
    SetMeasurementInterval      = 0x4600,
    GetDataReadyStatus          = 0x0202,
    ReadMeasurement             = 0x0300,
    SetAutomaticSelfCalibration = 0x5306,
    SetForcedRecalibrationValue = 0x5204,
    SetTemperatureOffset        = 0x5403,
    SetAltitude                 = 0x5102,
    ReadFirmwareVersion         = 0xd100,
    SoftReset                   = 0xd304,
}

const EXPECT_MSG: &str = "Vec was not large enough";
const ADDRESS: u8 = 0x61 << 1;

pub struct Scd30<T> {
    comm:    T,
    address: u8,
}

#[derive(Debug)]
pub struct Measurement {
    pub co2:         f32,
    pub humidity:    f32,
    pub temperature: f32,
}

/// See the [datasheet] for I²c parameters.
///
/// [datasheet]: https://www.sensirion.com/fileadmin/user_upload/customers/sensirion/Dokumente/9.5_CO2/Sensirion_CO2_Sensors_SCD30_Interface_Description.pdf
impl<T, E> Scd30<T> where T: Read<Error = E> + Write<Error = E> {

    fn add_argument<N>(&mut self, buf: &mut Vec<u8, N>, data: &[u8]) -> Result<(), ()> where N: ArrayLength<u8> {
        buf.extend_from_slice(data)?;
        let mut crc = Crc::<u8>::new(0x31, 8, 0xff, 0, false);
        crc.update(data);
        buf.push(crc.finish()).map_err(|_| ())
    }

    /// Returns an [Scd30] instance with the default address 0x61 shifted one place to the left.
    /// You may or may not need this bitshift depending on the byte size of
    /// your [I²c](embedded_hal::blocking::i2c) peripheral.
    pub fn new(i2c: T) -> Self {
        Scd30 {
            comm: i2c,
            address: ADDRESS
        }
    }

    /// Returns an [Scd30] instance with the specified address.
    pub fn new_with_address(i2c: T, address: u8) -> Self {
        Scd30 {
            comm: i2c,
            address
        }
    }

    pub fn soft_reset(&mut self) -> Result<(), E> {
        self.comm.write(self.address, &(Command::SoftReset as u16).to_be_bytes())
    }

    pub fn stop_measuring(&mut self) -> Result<(), E> {
        self.comm.write(self.address, &(Command::StopContinuousMeasurement as u16).to_be_bytes())
    }

    /// Enable or disable automatic self calibration (ASC).
    ///
    /// According to the datasheet, the sensor should be active continously for at least
    /// 7 days to find the initial parameter for ASC. The sensor has to be exposed to
    /// at least 1 hour of fresh air (~400ppm CO₂) per day.
    pub fn set_automatic_calibration(&mut self, enable: bool) -> Result<(), E> {
        let mut vec: Vec<u8, U5> = Vec::new();
        vec.extend_from_slice(&(Command::SetAutomaticSelfCalibration as u16).to_be_bytes()).expect(EXPECT_MSG);
        self.add_argument(&mut vec, &(enable as u16).to_be_bytes()).expect(EXPECT_MSG);
        self.comm.write(self.address, &vec)
    }

    pub fn set_forced_recalibration_value(&mut self, co2: u16) -> Result<(), E> {
        let mut vec: Vec<u8, U5> = Vec::new();
        vec.extend_from_slice(&(Command::SetForcedRecalibrationValue as u16).to_be_bytes()).expect(EXPECT_MSG);
        self.add_argument(&mut vec, &co2.to_be_bytes()).expect(EXPECT_MSG);
        self.comm.write(self.address, &vec)
    }

    /// Start measuring without mbar compensation.
    pub fn start_measuring(&mut self) -> Result<(), E> {
        self.start_measuring_with_mbar(0)
    }

    pub fn set_measurement_interval(&mut self, seconds: u16) -> Result<(), E> {
        let mut vec: Vec<u8, U5> = Vec::new();
        vec.extend_from_slice(&(Command::SetMeasurementInterval as u16).to_be_bytes()).expect(EXPECT_MSG);
        self.add_argument(&mut vec, &seconds.to_be_bytes()).expect(EXPECT_MSG);
        self.comm.write(self.address, &vec)
    }

    /// Start measuring with mbar (pressure) compensation.
    pub fn start_measuring_with_mbar(&mut self, pressure: u16) -> Result<(), E> {
        let mut vec: Vec<u8, U5> = Vec::new();
        vec.extend_from_slice(&(Command::StartContinuousMeasurement as u16).to_be_bytes()).expect(EXPECT_MSG);
        self.add_argument(&mut vec, &pressure.to_be_bytes()).expect(EXPECT_MSG);
        self.comm.write(self.address, &vec)
    }

    pub fn data_ready(&mut self) -> Result<bool, E> {
        let mut buf = [0u8; 2];
        self.comm.write(self.address, &(Command::GetDataReadyStatus as u16).to_be_bytes())?;
        self.comm.read(self.address, &mut buf)?;
        Ok(u16::from_be_bytes(buf) == 1)
    }

    pub fn read(&mut self) -> Result<Option<Measurement>, E> {
        match self.data_ready() {
            Ok(true) => {
                let mut buf = [0u8; 6 * 3];

                self.comm.write(self.address, &(Command::ReadMeasurement as u16).to_be_bytes())?;
                self.comm.read(self.address, &mut buf)?;

                Ok(Some(Measurement {
                    co2:         f32::from_bits(u32::from_be_bytes([ buf[0],  buf[1],  buf[3],  buf[4] ])),
                    temperature: f32::from_bits(u32::from_be_bytes([ buf[6],  buf[7],  buf[9],  buf[10] ])),
                    humidity:    f32::from_bits(u32::from_be_bytes([ buf[12], buf[13], buf[15], buf[16] ])),
                }))
            },
            Ok(false) => Ok(None),
            Err(e) => Err(e),
        }
    }

}
