use heapless::{String, Vec};
use core::fmt::Write;
use core::time::Duration;
use hd44780_driver::{CursorBlink, HD44780, entry_mode};
use hd44780_driver::bus::I2CBus;
use arduino_hal::i2c;
use arduino_hal::prelude::*;
use arduino_hal::prelude::_embedded_hal_blocking_i2c_Write;



use crate::kari::{map, millis};



#[allow(non_camel_case_types)]
pub struct kariString<const N: usize>{
    string: String<N>,
}


impl<const N: usize> kariString<N> {
    pub fn new<T: core::fmt::Display>(data: T) -> Self {
        let mut string = String::<N>::new();
        write!(string, "{}", data).unwrap();
        kariString { string }
    }

    pub fn as_str(&self) -> &str {
        &self.string.as_str()
    }

    pub fn update<T: core::fmt::Display>(&mut self, data: T) {
        self.string.clear();
        write!(self.string, "{}", data).unwrap();
    }

    pub fn append_str(&mut self, data: &str){
        self.string.push_str(data).unwrap();
    }

    pub fn to_string(data: &str) -> String<N>{
        let mut string = String::new();
        write!(string, "{}", data).unwrap();
        string
    }

    pub fn format_float(&mut self, value: f32, decimals: u16) -> &str {

        if !value.is_finite(){
            let _ = write!(self.string, "NaN");
            return &self.string.as_str();
        }

        self.string.clear();

        if value.is_sign_negative(){
            let _ = write!(self.string, "-");
        }

        let abs = value.abs();
        let int_part = abs as i32;
        let _ = write!(self.string, "{}", int_part);

        if decimals > 0 {
            let _ = write!(self.string, ".");
            let scale = libm::powf(10.0, decimals as f32); //powf uses f32 pow uses f64
            let frac_part = libm::roundf((value - abs) * scale) as u32;
            let _ = write!(self.string, "{:0width$}", frac_part, width = decimals as usize);
        }

        &self.string.as_str()

    }
}

#[allow(non_camel_case_types)]
pub trait kariI2C {
    fn scan(&mut self) -> Vec<u8, 128>;
    fn read_register(&mut self, addr: u8, reg: u8) -> Result<u8, ()>;
}

impl kariI2C for i2c::I2c {
    fn scan(&mut self) -> Vec<u8, 128> {
        let mut devices = Vec::new();
        for addr in 0x08..=0x77 {
            if self.write(addr, &[]).is_ok() {
                let _ = devices.push(addr);
            }
        }
        devices
    }

    fn read_register(&mut self, addr: u8, reg: u8) -> Result<u8, ()> {
        let mut buffer = [0u8, 1];

        if let Err(_) = self.write(addr, &[reg]) {
            return Err(())
        }
        if let Err(_) = self.read(addr, &mut buffer) {
            return Err(())
        }
        Ok(buffer[0])
    }
}

#[allow(non_camel_case_types)]
pub struct kariPID{
    kp: f32,
    ki: f32,
    kd: f32,
    set_point: f32,
    error: f32,
    prev_error: f32,
    integral: f32,
    derivative: f32,
    previous_time: u32

}

impl kariPID {
    pub fn new(set_point: f32, kp: f32, ki: f32, kd: f32,) -> Self{
        kariPID { 
            kp, ki, kd, set_point, error: 0.00, prev_error: 0.00, 
            integral: 0.00, derivative: 0.00, previous_time: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn evaluate(&mut self, feedBack: f32) -> f32 {
        let current_time = millis();
        let dt = current_time.saturating_sub(self.previous_time);

        self.error = self.set_point - feedBack;
        self.integral += self.error * dt as f32;
        self.derivative = if dt > 0 {
            (self.error - self.prev_error) / dt as f32
        } else { 0.0 };

        let correction = (self.kp * self.error) + (self.ki * self.integral) + (self.kd * self.derivative);
        self.prev_error = self.error;
        self.previous_time = current_time;

        correction

    }
}


#[allow(non_camel_case_types)]
    pub struct kariLcd {
        lcd: HD44780<I2CBus<i2c::I2c>>,
        delay: arduino_hal::Delay
    }

    impl kariLcd {
        pub fn new(i2c_instance: i2c::I2c, address: Option<u8>) -> Self {
            let mut delay = arduino_hal::Delay::new();
            let address = address.unwrap_or(0x27);
            let lcd = HD44780::new_i2c(i2c_instance, address, &mut delay).unwrap();
            kariLcd { lcd, delay }
        }

        pub fn clear(&mut self) -> &mut Self{
            self.lcd.clear(&mut self.delay).unwrap();
            self
        }

        pub fn write(&mut self, text: &str) -> &mut Self{
            self.lcd.write_str(&text, &mut self.delay).unwrap();
            self
        }

        pub fn set_cursor(&mut self, position: u8) -> &mut Self{
            self.lcd.set_cursor_pos(position, &mut self.delay).unwrap();
            self
        }

        pub fn shift_display(&mut self, direction:hd44780_driver::Direction) -> &mut Self{
            self.lcd.shift_display(direction, &mut self.delay).unwrap();
            self
        }

        pub fn set_cursor_visibility(&mut self, visibility:hd44780_driver::Cursor) -> &mut Self{
            self.lcd.set_cursor_visibility(visibility, &mut self.delay).unwrap();
            self
        }

        pub fn set_cursor_blink(&mut self, blink:CursorBlink) -> &mut Self{
            self.lcd.set_cursor_blink(blink, &mut self.delay).unwrap();
            self
        }

        pub fn set_cursor_mode(&mut self, mode: entry_mode::CursorMode) -> &mut Self{
            self.lcd.set_cursor_mode(mode, &mut self.delay).unwrap();
            self
        }

        pub fn write_char(&mut self, data: char) -> &mut Self{
            self.lcd.write_char(data, &mut self.delay).unwrap();
            self
        }

    }

#[allow(non_camel_case_types)]
pub struct kariJoyStick {
    read_x:  fn()  -> u16,
    read_y: fn() -> u16,
    sensitivity: i32,
    threshold: i32
}


#[allow(non_snake_case)]
impl kariJoyStick {
    pub fn new(read_x:  fn() -> u16, read_y: fn() -> u16, sensitivity: i32, threshold: Option<i32>,) -> Self {
        let threshold = threshold.unwrap_or(512);
        kariJoyStick { read_x, read_y, sensitivity, threshold }
    }

    pub fn onX<F>(&self, callback: Option<F>, allow_noise: bool) -> &Self
    where 
        F: FnMut(f32)
    {
        if let Some(mut cb) = callback {
            let _x = (self.read_x)() as f32;
            if allow_noise || (_x as i32 - self.threshold).abs() >= self.sensitivity {
                cb(_x);
            }
        }
        self
    }

    pub fn onY<F>(&self, callback: Option<F>, allow_noise: bool) -> &Self
    where 
        F: FnMut(f32)
    {
        if let Some(mut cb) = callback {
            let _y = (self.read_y)() as f32;
            if allow_noise || (_y as i32 - self.threshold).abs() >= self.sensitivity {
                cb(_y);
            }
        }
        self
    }
}

#[allow(non_camel_case_types)]
pub struct kariPH<F>
where F : FnMut() -> u16
{
    ph_data: f32,
    iteration_count: usize,
    read_fn: F
}

#[allow(non_snake_case)]
impl<F> kariPH<F> where F : FnMut() -> u16 {
    pub fn new(read_fn: F, iteration_count: usize) -> Self {
        Self { ph_data: 0.00, iteration_count, read_fn}
    }

    pub fn measure(&mut self) -> &mut Self {
        let mut sum = 0.00;
        for _ in 0..self.iteration_count {
            let raw = (self.read_fn)() as f32;
            sum += raw;
        }

        self.ph_data = sum / self.iteration_count as f32;
        self.ph_data = map(self.ph_data, 0f32, 1023f32, 0f32, 14f32);

        self
    }

    pub fn onMeasure<C>(&self, mut callback: C) -> &Self 
    where 
    C: FnMut(f32)
    {
        callback(self.ph_data);
        self
    }
}


#[allow(non_camel_case_types)]
pub struct kariDrive<SpeedFn1, SpeedFn2, DigitalFn1, DigitalFn2, DigitalFn3, DigitalFn4> 
where 
SpeedFn1: FnMut(u8),
SpeedFn2: FnMut(u8),
DigitalFn1: FnMut(bool),
DigitalFn2: FnMut(bool),
DigitalFn3: FnMut(bool),
DigitalFn4: FnMut(bool),
{
    motor1_en: SpeedFn1, 
    motor1_in_a: DigitalFn1, 
    motor1_in_b: DigitalFn2, 
    
    motor2_en: SpeedFn2, 
    motor2_in_a: DigitalFn3, 
    motor2_in_b: DigitalFn4,
}

impl<SpeedFn1, SpeedFn2, DigitalFn1, DigitalFn2, DigitalFn3, DigitalFn4> 

kariDrive<SpeedFn1, SpeedFn2, DigitalFn1, DigitalFn2, DigitalFn3, DigitalFn4>

where 
SpeedFn1: FnMut(u8),
SpeedFn2: FnMut(u8),
DigitalFn1: FnMut(bool),
DigitalFn2: FnMut(bool),
DigitalFn3: FnMut(bool),
DigitalFn4: FnMut(bool),
{
    pub fn new(
        motor1_en: SpeedFn1, motor1_in_a: DigitalFn1,
        motor1_in_b: DigitalFn2, motor2_en: SpeedFn2, 
        motor2_in_a: DigitalFn3, motor2_in_b: DigitalFn4 
    ) -> Self {
        
        Self { motor1_en, motor1_in_a, motor1_in_b, motor2_en, motor2_in_a, motor2_in_b }
    }

    pub fn drive(&mut self, speed: u8, direction_status: bool) -> &mut Self {
        self.changeover(direction_status);
        self.accelerate(speed);
        self
    }

    pub fn right(&mut self, speed: u8, direction_status: bool) -> &mut Self {
        self.changeover(direction_status);
        self.differentiate(speed, true);
        self
    }

    pub fn left(&mut self, speed: u8, direction_status: bool) -> &mut Self {
        self.changeover(direction_status);
        self.differentiate(speed, false);
        self
    }

    fn accelerate(&mut self, speed: u8) {
        (self.motor1_en)(speed.into());
        (self.motor2_en)(speed.into());
    }

    fn changeover(&mut self, value: bool) {
        if value {
            (self.motor1_in_a)(true);
            (self.motor2_in_a)(true);
            (self.motor1_in_b)(false);
            (self.motor2_in_b)(false);
        } else {
            (self.motor1_in_a)(false);
            (self.motor2_in_a)(false);
            (self.motor1_in_b)(true);
            (self.motor2_in_b)(true);
        }
    }

    fn differentiate(&mut self, speed_difference: u8, direction: bool) {
        if direction {
            (self.motor2_en)(speed_difference.into());
            (self.motor1_en)(20);
        } else {
            (self.motor1_en)(speed_difference.into());
            (self.motor2_en)(20);
        }
    }
}

#[allow(non_camel_case_types)]
pub struct kariPIR<F>
where 
    F: FnMut() -> bool
{
    signal_read_fn: F
}

impl<F> kariPIR<F>
where 
F: FnMut() -> bool
{
    pub fn new(signal_read_fn: F) -> Self {
        Self {signal_read_fn}
    }

    pub fn on_measure<T, G>(&mut self, mut callback: T, fallback: Option<G>) -> &mut Self
    where 
        T: FnMut(),
        G: FnMut()
    {
        if (self.signal_read_fn)(){
            callback();
        } else if let Some(mut fb) = fallback {
            fb();
        }

        self

    }

}


#[allow(non_camel_case_types)]
pub struct kariInfrared<F>
where 
    F: FnMut() -> bool
{
    signal_read_fn: F
}

impl<F> kariInfrared<F>
where 
F: FnMut() -> bool
{
    pub fn new(signal_read_fn: F) -> Self {
        Self {signal_read_fn}
    }

    pub fn on_measure<T>(&mut self, mut callback: T)
    where 
        T: FnMut()
    {
        if (self.signal_read_fn)(){
            callback();
        }

    }

}