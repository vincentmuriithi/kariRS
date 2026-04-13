#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
use arduino_hal::I2c;


pub use ds323x::{Alarm1Matching, DateTimeAccess, Ds323x, Hours, NaiveDate, NaiveDateTime, NaiveTime, WeekdayAlarm1, interface::{I2cInterface, SpiInterface}};
pub use ds323x::ic::{DS3231, DS3232, DS3234};
use crate::kariI2C;

/*
Methods that can be called on the instance:
    - set_year()
    - set_month()
    - set_day()
    - set_hour()
    - set_minute()
    - set_second()
    - enable() ->  ensures the clock is ticking.
    - disable() -> stops the oscillator (timekeeping halts).

Reading methods include: 
    - year() 
    - month()
    - day() 
    - hour() 
    - minute() 
    - second()

Alarm methods include:
    - set_alarm1_hms()
    - set_alarm2_hms()
    - set_alarm1_day()
    - set_alarm2_day()
    - has_alarm1_matched()
    - has_alarm2_matched()
    - clear_alarm1_matched_flag()
    - clear_alarm2_matched_flag()
    - set_alarm1_weekday(when, matching)

Status and control methods include:
    - busy() -> Checks if the RTC is busy updating its internal registers (like when it’s rolling over a second).
    - has_been_stopped() -> Checks if the oscillator has stopped (e.g., due to power loss or battery failure).
    - clear_has_been_stopped_flag() -> Clears the “oscillator stopped” flag after you’ve detected it
    - set_aging_offset() -> Adjusts the oscillator’s aging compensation.
    - use_int_sqw_output_as_interrupt() ->  Configures the INT/SQW pin to act as an interrupt output instead of a square‑wave generator.
    - set_square_wave_frequency(freq) -> Configures the INT/SQW pin to output a square wave at a chosen frequency (1Hz, 4kHz, 8kHz, 32kHz).


Types of DS323x RTC chips:
    - DS3231
    - DS3232
    - DS3234

*/

// let date = NaiveDate::from_ymd_opt(2026, 2, 27).unwrap();
// let time = NaiveTime::from_hms_opt(12, 34, 56).unwrap();


#[allow(non_camel_case_types)]
pub struct timeData {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8 
}

pub struct AlarmInput {
    weekday: u8,
    hour: u8,
    minute: u8,
    second: u8,
    format: Format
}

pub enum Format {
    AM,
    PM,
    H24
}

pub trait KariRTC {
    fn to_seconds(&self, hour: u8, minutes: u8, seconds: u8) -> u32;
}

#[allow(non_camel_case_types)]
pub struct kariDS323xI2C<I2C, CHIP> {
    pub instance: Ds323x<I2cInterface<I2C>, CHIP>
}


impl<I2C> kariDS323xI2C<I2C, ds323x::ic::DS3231>
where 
I2C: kariI2C + embedded_hal::i2c::I2c
{
    pub fn new(i2c: I2C) -> Self{
        let instance = Ds323x::new_ds3231(i2c);
        Self { instance }
    }

    pub fn enable(&mut self) {
        self.instance.enable().unwrap();
    }

    pub fn disable(&mut self) {
        self.instance.disable().unwrap();
    }

    pub fn get_time(&mut self) -> NaiveDateTime {
        self.instance.datetime().unwrap()
        
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

    pub fn set_alarm1_hms(&mut self, hour: u32, minute: u32, second: u32) {
        let alarm_time = NaiveTime::from_hms_opt(hour, minute, second).unwrap();
        self.instance.set_alarm1_hms(alarm_time).unwrap();
    }

    pub fn set_alarm2_hm(&mut self, hour: u32, minute: u32) {
        let alarm_time = NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
        self.instance.set_alarm2_hm(alarm_time).unwrap();
    }

    pub fn set_alarm1_weekday(&mut self, alarm: AlarmInput) {
        let hour = self.to_hours(alarm.hour, alarm.format).unwrap();
        let alarm_time = WeekdayAlarm1 {
            weekday: alarm.weekday,
            hour,
            minute: alarm.minute,
            second: alarm.second
        };

        let _ = self.instance.set_alarm1_weekday(alarm_time, Alarm1Matching::HoursMinutesAndSecondsMatch);
    } 

    fn to_hours(&mut self, hour: u8, format: Format) -> Result<Hours, &'static str> {
        match format {
            Format::AM => {
                if (1..=12).contains(&hour) {
                    Ok(Hours::AM(hour))
                } else {
                    Err("Invalid AM hour: must be 1–12")
                }
            }

            Format::PM => {
                if (1..=12).contains(&hour) {
                    Ok(Hours::PM(hour))
                } else {
                    Err("Invalid PM hour: must be 1–12")
                }
            }

            Format::H24 => {
                if hour <= 23 {
                    Ok(Hours::H24(hour))
                } else {
                    Err("Invalid 24h hour: must be 0–23")
                }
            }
        }
    }

    pub fn has_alarm1_matched(&mut self) -> bool{
        if self.instance.has_alarm1_matched().unwrap() {
            self.instance.clear_alarm1_matched_flag().unwrap();
            true
        } else {
            false
        }
    }

    pub fn has_alarm2_matched(&mut self) -> bool{
        if self.instance.has_alarm2_matched().unwrap() {
            self.instance.clear_alarm2_matched_flag().unwrap();
            true
        } else {
            false
        }
    }

    pub fn use_int_sqw_output_as_interrupt(&mut self) {
        self.instance.use_int_sqw_output_as_interrupt().unwrap();
    }
}





impl<I2C> kariDS323xI2C<I2C, ds323x::ic::DS3232>
where 
I2C: kariI2C + embedded_hal::i2c::I2c
{
    pub fn new(i2c:I2C) -> Self{
        let instance = Ds323x::new_ds3232(i2c);
        Self { instance }
    }

    pub fn enable(&mut self) {
        self.instance.enable().unwrap();
    }

    pub fn disable(&mut self) {
        self.instance.disable().unwrap();
    }

    pub fn get_time(&mut self) -> NaiveDateTime {
        self.instance.datetime().unwrap()
        
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

    pub fn set_alarm1_hms(&mut self, hour: u32, minute: u32, second: u32) {
        let alarm_time = NaiveTime::from_hms_opt(hour, minute, second).unwrap();
        self.instance.set_alarm1_hms(alarm_time).unwrap();
    }

    pub fn set_alarm2_hm(&mut self, hour: u32, minute: u32) {
        let alarm_time = NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
        self.instance.set_alarm2_hm(alarm_time).unwrap();
    }

    pub fn set_alarm1_weekday(&mut self, alarm: AlarmInput) {
        let hour = self.to_hours(alarm.hour, alarm.format).unwrap();
        let alarm_time = WeekdayAlarm1 {
            weekday: alarm.weekday,
            hour,
            minute: alarm.minute,
            second: alarm.second
        };

        let _ = self.instance.set_alarm1_weekday(alarm_time, Alarm1Matching::HoursMinutesAndSecondsMatch);
    } 

    fn to_hours(&mut self, hour: u8, format: Format) -> Result<Hours, &'static str> {
        match format {
            Format::AM => {
                if (1..=12).contains(&hour) {
                    Ok(Hours::AM(hour))
                } else {
                    Err("Invalid AM hour: must be 1–12")
                }
            }

            Format::PM => {
                if (1..=12).contains(&hour) {
                    Ok(Hours::PM(hour))
                } else {
                    Err("Invalid PM hour: must be 1–12")
                }
            }

            Format::H24 => {
                if hour <= 23 {
                    Ok(Hours::H24(hour))
                } else {
                    Err("Invalid 24h hour: must be 0–23")
                }
            }
        }
    }

    pub fn has_alarm1_matched(&mut self) -> bool{
        if self.instance.has_alarm1_matched().unwrap() {
            self.instance.clear_alarm1_matched_flag().unwrap();
            true
        } else {
            false
        }
    }

    pub fn has_alarm2_matched(&mut self) -> bool{
        if self.instance.has_alarm2_matched().unwrap() {
            self.instance.clear_alarm2_matched_flag().unwrap();
            true
        } else {
            false
        }
    }

    pub fn use_int_sqw_output_as_interrupt(&mut self) {
        self.instance.use_int_sqw_output_as_interrupt().unwrap();
    }
}



#[allow(non_camel_case_types)]
pub struct kariDS323xSPI<SPI, CHIP> {
    instance: Ds323x<SpiInterface<SPI>, CHIP>
}


impl<SPI> kariDS323xSPI<SPI, ds323x::ic::DS3234>
where 
SPI: embedded_hal::spi::SpiDevice
{
    pub fn new(spi:SPI) -> Self{
        let instance = Ds323x::new_ds3234(spi);
        Self { instance }
    }

    pub fn enable(&mut self) {
        self.instance.enable().unwrap();
    }

    pub fn disable(&mut self) {
        self.instance.disable().unwrap();
    }

    pub fn get_time(&mut self) -> NaiveDateTime {
        self.instance.datetime().unwrap()
        
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

    pub fn set_alarm1_hms(&mut self, hour: u32, minute: u32, second: u32) {
        let alarm_time = NaiveTime::from_hms_opt(hour, minute, second).unwrap();
        self.instance.set_alarm1_hms(alarm_time).unwrap();
    }

    pub fn set_alarm2_hm(&mut self, hour: u32, minute: u32) {
        let alarm_time = NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
        self.instance.set_alarm2_hm(alarm_time).unwrap();
    }

    pub fn set_alarm1_weekday(&mut self, alarm: AlarmInput) {
        let hour = self.to_hours(alarm.hour, alarm.format).unwrap();
        let alarm_time = WeekdayAlarm1 {
            weekday: alarm.weekday,
            hour,
            minute: alarm.minute,
            second: alarm.second
        };

        let _ = self.instance.set_alarm1_weekday(alarm_time, Alarm1Matching::HoursMinutesAndSecondsMatch);
    } 

    fn to_hours(&mut self, hour: u8, format: Format) -> Result<Hours, &'static str> {
        match format {
            Format::AM => {
                if (1..=12).contains(&hour) {
                    Ok(Hours::AM(hour))
                } else {
                    Err("Invalid AM hour: must be 1–12")
                }
            }

            Format::PM => {
                if (1..=12).contains(&hour) {
                    Ok(Hours::PM(hour))
                } else {
                    Err("Invalid PM hour: must be 1–12")
                }
            }

            Format::H24 => {
                if hour <= 23 {
                    Ok(Hours::H24(hour))
                } else {
                    Err("Invalid 24h hour: must be 0–23")
                }
            }
        }
    }

    pub fn has_alarm1_matched(&mut self) -> bool{
        if self.instance.has_alarm1_matched().unwrap() {
            self.instance.clear_alarm1_matched_flag().unwrap();
            true
        } else {
            false
        }
    }

    pub fn has_alarm2_matched(&mut self) -> bool{
        if self.instance.has_alarm2_matched().unwrap() {
            self.instance.clear_alarm2_matched_flag().unwrap();
            true
        } else {
            false
        }
    }

    pub fn use_int_sqw_output_as_interrupt(&mut self) {
        self.instance.use_int_sqw_output_as_interrupt().unwrap();
    }
}