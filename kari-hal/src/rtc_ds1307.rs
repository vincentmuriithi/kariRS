use ds323x::interface::I2cInterface;
use ds1307::{DateTimeAccess, Ds1307, NaiveDate, NaiveDateTime};
pub use rtcc::Rtcc;
use crate::{KariRTC, kariI2C, timeData};
use rtcc::Hours;


#[allow(non_camel_case_types)]
pub struct kariDS1307<I2C> {
    pub instance: Ds1307<I2C>
}


impl<I2C> kariDS1307<I2C> 
where 
    I2C: kariI2C + embedded_hal::i2c::I2c
{
    pub fn new(i2c: I2C) -> Self{
        let instance = Ds1307::new(i2c);
        Self { instance }
    }

    pub fn enable_square_wave_output(&mut self) {
        self.instance.enable_square_wave_output().unwrap();
    }

    pub fn disable_square_wave_output(&mut self) {
        self.instance.disable_square_wave_output().unwrap();
    }

    pub fn get_time(&mut self) -> NaiveDateTime {
        self.instance.datetime().unwrap()
        
    }

    pub fn get_hours(&mut self) -> u8 {
        let hour = match self.instance.hours().unwrap() {
            Hours::AM(h) => h,
            Hours::PM(h) => h,
            Hours::H24(h) => h,
        };
        hour
    }

    pub fn get_minutes(&mut self) -> u8 {
        self.instance.minutes().unwrap()
    }

    pub fn get_seconds(&mut self) -> u8 {
        self.instance.seconds().unwrap()
    }

    pub fn set_time(&mut self, time: timeData) -> Option<()>{
        let new_time = 
        NaiveDate::from_ymd_opt(time.year.into(), time.month.into(), time.day.into()).unwrap()
        .and_hms_opt(time.hour.into(), time.minute.into(), time.second.into()).unwrap();

        match self.instance.set_datetime(&new_time) {
            Ok(_) => Some(()),
            Err(_) => None
        }
    }

}

impl<I2C> KariRTC for kariDS1307<I2C> 
where 
    I2C: kariI2C + embedded_hal::i2c::I2c
{
    fn to_seconds(&self, hour: u8, minutes: u8, seconds: u8) -> u32 {
        hour as u32 * 3600 + minutes as u32 * 60 + seconds as u32
    }
}
