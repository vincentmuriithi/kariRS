#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]

use core::cell::UnsafeCell;
pub use core::sync::atomic::{AtomicU8, Ordering};
pub use core::cell::Cell;
pub use core::mem::size_of;
use critical_section::Mutex;
pub use hd44780_driver::HD44780;
use heapless::Vec;
pub use postcard::{to_slice, from_bytes};
pub use core::fmt::Write;
pub use heapless::String;
pub mod hal;
pub  use hal::*;
pub mod cores;
pub  use cores::*;


pub static MILLIS_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
pub static mut INIT_REGISTERED: bool = false;
pub static mut LOOP_REGISTERED: bool = false;


#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
pub fn hardware_init() {
    unsafe { core::arch::asm!("sei"); }

}

#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
pub mod kari {
    use crate::MILLIS_COUNTER;
    use arduino_hal::hal::port;
    use arduino_hal::hal::port::*;
    use arduino_hal::hal::wdt::Timeout;
    use arduino_hal::port::mode::*;
    use arduino_hal::port::Pin;
    use critical_section::with;
    use hd44780_driver::HD44780;
    pub use hd44780_driver::*;
    use arduino_hal::i2c;
    use arduino_hal::I2c;
    use hd44780_driver::bus::I2CBus;
    // use core::sync::atomic::Ordering;

    pub fn delay(milliseconds: u32){
        arduino_hal::delay_ms(milliseconds);
    }

    #[allow(non_snake_case)]
    pub fn delayMicroseconds(microseconds: u32){
        arduino_hal::delay_us(microseconds);
    }

    pub fn millis() -> u32 {

        let k = with(|cs| {
            MILLIS_COUNTER.borrow(cs).get()
        });
        unsafe { core::arch::asm!("sei"); }
        k
    
    }

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

    #[cfg(feature = "uno")]
    pub fn init_i2c(twi: arduino_hal::pac::TWI, sda:Pin<Input<PullUp>, PC4>, scl:Pin<Input<PullUp>, PC5> , speed: Option<u32>) -> i2c::I2c {
        let speed = speed.unwrap_or(100000);
        i2c::I2c::new(twi, sda, scl, speed)
    }

    pub fn lcd_init(i2c_instance:  i2c::I2c, address: Option<u8>) -> HD44780<I2CBus<i2c::I2c>>{
        let mut delay = arduino_hal::Delay::new();
        let address = address.unwrap_or(0x27);
         HD44780::new_i2c(i2c_instance, address, &mut delay).unwrap()
    }


    pub fn map(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
        (value - in_min) * (in_max - in_min) / (out_max - out_min) + out_min
    }

    


}

#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
pub mod gpio {
    pub const INPUT: u8 = 0;
    pub const OUTPUT: u8 = 1;
    pub const PWM: u8 = 2;
    pub const _INPUT: u8 = 3;

    pub const LOW: u8 = 0;
    pub const HIGH: u8 = 1;

    pub const _LOW: bool = false;
    pub const _HIGH: bool = true;

}

#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
pub mod time {
   
    use crate::MILLIS_COUNTER;

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