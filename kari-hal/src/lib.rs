#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]

#[cfg(feature = "esp")]
pub use core::cell::OnceCell;
pub use core::marker::PhantomData;
use core::cell::UnsafeCell;
pub use core::sync::atomic::{AtomicU8, Ordering};
pub use core::cell::Cell;
pub use core::mem::size_of;
use critical_section::Mutex;
pub use hd44780_driver::HD44780;
// pub use embedded_hal::pwm::{self, ErrorKind, ErrorType, SetDutyCycle};
pub use libm::*;
#[cfg(feature = "esp")]
pub use embedded_hal_compat::eh1_0::pwm::SetDutyCycle;

#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
pub use arduino_hal::i2c as kariI2c;

#[cfg(feature = "esp")]
pub use esp_hal::i2c as kariI2c;
#[cfg(feature = "esp")]
pub use esp_hal::gpio::DriveMode;

pub use postcard::{to_slice, from_bytes};
pub use core::fmt::Write;
pub use heapless::String;
pub use heapless::Vec;
pub mod servo;
pub use servo::*;
pub mod hal;
pub  use hal::*;
pub mod esp_func;
#[cfg(feature = "esp")]
pub use esp_func::*;
pub mod cores;
pub  use cores::*;
#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
pub mod rtc_ds323x;
#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
pub use rtc_ds323x::*;
#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
pub mod rtc_ds1307;
#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
pub use rtc_ds1307::*;
pub mod dht;
pub use dht::*;
pub mod ultrasonic;
pub use ultrasonic::*;
#[cfg(feature = "esp")]
use esp_hal::time::Instant;


pub static MILLIS_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
pub static mut INIT_REGISTERED: bool = false;
pub static mut LOOP_REGISTERED: bool = false;

#[cfg(feature = "esp")]
static START: Mutex<OnceCell<Instant>> = Mutex::new(OnceCell::new()); 


#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
pub fn hardware_init() {
    unsafe { core::arch::asm!("sei"); }

}

#[cfg(feature = "esp")]
pub fn hardware_init() {
    let cs = unsafe { critical_section::CriticalSection::new() };
    START.borrow(cs).set(Instant::now());
}


pub mod kari {
    use crate::MILLIS_COUNTER;
    #[cfg(feature = "esp")]
use crate::START;
    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    use arduino_hal::hal::port;
    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    use arduino_hal::hal::port::*;
    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    use arduino_hal::hal::wdt::Timeout;
    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    use arduino_hal::port::mode::*;
    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    use arduino_hal::port::Pin;
    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    use arduino_hal::i2c;
    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    use arduino_hal::I2c;
    use critical_section::with;
    #[cfg(feature = "esp")]
    use esp_hal::gpio::OutputPin;
    use hd44780_driver::HD44780;
    pub use hd44780_driver::*;
    use hd44780_driver::bus::I2CBus;
    // use core::sync::atomic::Ordering;
    #[cfg(feature = "esp")]
    use esp_hal::delay::Delay;
    #[cfg(feature = "esp")]
    pub static  _KARI_DELAY: Delay = Delay::new();
    #[cfg(feature = "esp")]
    use esp_hal::time::Rate;
    #[cfg(feature = "esp")]
    use esp_hal::i2c::master::Config;
    #[cfg(feature = "esp")]
    use esp_hal::i2c::master::I2c;
    


    #[cfg(feature = "esp")]
    pub fn delay(milliseconds: u32){
        _KARI_DELAY.delay_millis(milliseconds);
    }

    #[cfg(feature = "esp")]
    #[allow(non_snake_case)]
    pub fn delayMicroseconds(microseconds: u32){
        _KARI_DELAY.delay_micros(microseconds);
    }

    #[cfg(feature = "esp")]
    pub fn millis() -> u32 {
        let cs = unsafe { critical_section::CriticalSection::new() };
        START.borrow(cs).get().unwrap().elapsed().as_millis() as u32
    }

    #[cfg(feature = "esp")]
    pub fn millis_u64() -> u64 {
        let cs = unsafe { critical_section::CriticalSection::new() };
        START.borrow(cs).get().unwrap().elapsed().as_millis() 
    }

    #[cfg(feature = "esp")]
    pub fn micros() -> u32 {
        let cs = unsafe { critical_section::CriticalSection::new() };
        START.borrow(cs).get().unwrap().elapsed().as_micros() as u32
    }

    #[cfg(feature = "esp")]
    pub fn micros_u64() -> u64 {
        let cs = unsafe { critical_section::CriticalSection::new() };
        START.borrow(cs).get().unwrap().elapsed().as_micros()
    }


    #[cfg(feature = "esp")]
    pub fn seconds() -> u64 {
        let cs = unsafe { critical_section::CriticalSection::new() };
        START.borrow(cs).get().unwrap().elapsed().as_secs()
    }

    #[cfg(feature = "esp")]
    pub fn minutes() -> u64 {
        let cs = unsafe { critical_section::CriticalSection::new() };
        START.borrow(cs).get().unwrap().elapsed().as_minutes()
    }

    #[cfg(feature = "esp")]
    pub fn hours() -> u64 {
        let cs = unsafe { critical_section::CriticalSection::new() };
        START.borrow(cs).get().unwrap().elapsed().as_hours()
    }

    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    pub fn delay(milliseconds: u32){
        arduino_hal::delay_ms(milliseconds);
    }

    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    #[allow(non_snake_case)]
    pub fn delayMicroseconds(microseconds: u32){
        arduino_hal::delay_us(microseconds);
    }

    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    pub fn millis() -> u32 {

        let k = with(|cs| {
            MILLIS_COUNTER.borrow(cs).get()
        });
        unsafe { core::arch::asm!("sei"); }
        k
    
    }

    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    pub fn micros() -> u32 {
        let ms = millis();
        let tcnto: u8;
        unsafe {
            let dp = arduino_hal::Peripherals::steal();
            tcnto = dp.TC0.tcnt0.read().bits();
        }
        let micros_from_tcnto = (tcnto as u32) * 4;
        ms * 1000 + micros_from_tcnto
    }

    #[cfg(feature = "mega")]
    pub fn init_i2c(twi: arduino_hal::pac::TWI, sda:Pin<Input<PullUp>, PD1>, scl:Pin<Input<PullUp>, PD0> , speed: Option<u32>) -> i2c::I2c {
        let speed = speed.unwrap_or(100000);
        i2c::I2c::new(twi, sda, scl, speed)
    }

    #[cfg(any(feature = "uno", feature = "nano"))]
    pub fn init_i2c(twi: arduino_hal::pac::TWI, sda:Pin<Input<PullUp>, PC4>, scl:Pin<Input<PullUp>, PC5> , speed: Option<u32>) -> i2c::I2c {
        let speed = speed.unwrap_or(100000);
        i2c::I2c::new(twi, sda, scl, speed)
    }

    #[cfg(feature = "esp")]
    pub fn init_i2c<'a>(i2c_peripheral: esp_hal::peripherals::I2C0<'a>, sda: impl OutputPin + 'a, scl: impl OutputPin + 'a, speed: Option<u32>) -> esp_hal::i2c::master::I2c<'a, esp_hal::Blocking>{
        let speed = speed.unwrap_or(100_000);
        I2c::new(i2c_peripheral, Config::default().with_frequency(Rate::from_hz(speed))).unwrap()
        .with_sda(sda)
        .with_scl(scl)
    }


    #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
    pub fn lcd_init(i2c_instance:  i2c::I2c, address: Option<u8>) -> HD44780<I2CBus<i2c::I2c>>{
        let mut delay = arduino_hal::Delay::new();
        let address = address.unwrap_or(0x27);
         HD44780::new_i2c(i2c_instance, address, &mut delay).unwrap()
    }


    pub fn map(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
        (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
    }

    


}


pub mod gpio {
    pub const INPUT: u8 = 0;
    pub const OUTPUT: u8 = 1;
    pub const PWM: u8 = 2;
    pub const _INPUT: u8 = 3;
    pub const INPUT_PULLUP: u8 = 4;
    pub const INPUT_PULLDOWN: u8 = 5;  // esp specific feature

    pub const LOW: u8 = 0;
    pub const HIGH: u8 = 1;

    pub const _LOW: bool = false;
    pub const _HIGH: bool = true;

}

#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
pub mod time {
   
    use crate::MILLIS_COUNTER;
    static mut SERVO_PULSE_TICKS: u16 = 1500;
    static mut SERVO_STATE: bool = false;
    // use avr_hal_generic::avr_device::atmega2560;
    // use avr_hal_generic::avr_device::interrupt;

    #[cfg(feature = "mega")]
    #[avr_device_macros::interrupt(atmega2560)]
    fn TIMER0_COMPA() {
        let counter = MILLIS_COUNTER.borrow(unsafe {critical_section::CriticalSection::new()});
        counter.set(counter.get().wrapping_add(1));

    }

    #[cfg(any(feature = "uno", feature = "nano"))]
    #[avr_device_macros::interrupt(atmega328p)]
    fn TIMER0_COMPA() {
        let counter = MILLIS_COUNTER.borrow(unsafe {critical_section::CriticalSection::new()});
        counter.set(counter.get().wrapping_add(1));

    }

    #[cfg(feature = "leonardo")]
    #[avr_device_macros::interrupt(atmega32u4)]
    fn TIMER0_COMPA() {
        let counter = MILLIS_COUNTER.borrow(unsafe {critical_section::CriticalSection::new()});
        counter.set(counter.get().wrapping_add(1));

    }


    #[avr_device_macros::interrupt(atmega2560)]
    fn TIMER1_COMPA() {
        unsafe {
            let dp = arduino_hal::Peripherals::steal();
            //let pins = arduino_hal::pins!(dp);
            let portb = dp.PORTB; // direct port access is lighter
            let mask = 1 << 5;

           if SERVO_STATE == false {
            // Start pulse
            //pin_11.set_high();
            portb.portb.write(|w| w.bits(mask)); // set HIGH

            // Schedule end of pulse
            dp.TC1.ocr1a.write(|w| w.bits(us_to_ticks(SERVO_PULSE_TICKS)));
            SERVO_STATE = true;
        } else {
            // End pulse
            //pin_11.set_low();
            portb.portb.write(|w| w.bits(0)); // set LOW
            // Schedule refresh interval (20 ms)
            dp.TC1.ocr1a.write(|w| w.bits(us_to_ticks(20_000)));
            SERVO_STATE = false;
        }

        }
    }

    pub fn us_to_ticks(us: u16) -> u16 {
        // 16 MHz / 8 prescaler = 2 MHz tick → 0.5 µs per tick
        (us as u32 * 2) as u16
    }

    pub fn servo_write(angle: u16) {
        let min_us = 544;
        let max_us = 2400;
        let pulse_us = min_us + ((max_us - min_us) * angle as u32 / 180);
        unsafe { SERVO_PULSE_TICKS = pulse_us as u16; }
    }

}

struct kariAtomicU32 {
    value: u32,
}


impl kariAtomicU32 {
    pub fn new(init: u32) -> Self{
        Self { value: init }
    }

    pub fn load(&self) -> u32 {
        90
    }
}