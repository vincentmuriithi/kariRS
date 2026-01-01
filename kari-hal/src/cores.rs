use core::any::Any;

// use arduino_hal::hal::Atmega;
// use arduino_hal::pac::EEPROM;
// use heapless::{String, Vec};
// use core::fmt::Write;
// use arduino_hal::i2c;
// use arduino_hal::prelude::*;
// use arduino_hal::prelude::_embedded_hal_blocking_i2c_Write;
use arduino_hal::{Eeprom, eeprom::OutOfBoundsError};
use arduino_hal::prelude::_embedded_hal_serial_Read as Read;
pub use arduino_hal::spi;
//use arduino_hal::spi::ChipSelectPin;
//use embedded_hal::spi::Operation::Transfer;
use embedded_hal::spi::{SpiBus, SpiDevice, };

#[allow(non_camel_case_types)]
pub struct kariEEPROM {
    eeprom: Eeprom
}

type BoundError = Result<(), arduino_hal::eeprom::OutOfBoundsError>;

impl kariEEPROM {
    pub fn new(eeprom: arduino_hal::pac::EEPROM) -> Self {
        let eeprom = arduino_hal::eeprom::Eeprom::new(eeprom);
        Self {eeprom}
    }

    pub fn write(&mut self, offset: u16, buf: &[u8]) -> BoundError {
       self.eeprom.write(offset, buf)?;
       Ok(())
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.eeprom.read_byte(address)
    }

    pub fn read(&self, offset: u16, buf: &mut [u8]) -> BoundError {
        self.eeprom.read(offset, buf)?;
        Ok(())
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.eeprom.write_byte(address, data);
    }

    pub fn erase_byte(&mut self, address: u16) {
        self.eeprom.erase_byte(address);
    }

    pub fn erase(&mut self, from: u16, to: u16) -> BoundError {
        self.eeprom.erase(from, to)?;
        Ok(())
    }

    pub fn capacity(&self) -> u16 {
        self.eeprom.capacity()
    }

    pub fn update_byte(&mut self, address: u16, data: u8) {
        let current = self.eeprom.read_byte(address);
        if current != data {
            self.eeprom.write_byte(address, data);
        }
    }

    pub fn update(&mut self, address: u16, buffer: &[u8]) -> BoundError {
        if address as usize + buffer.len() > self.capacity().into() {
            return Err(OutOfBoundsError)
        }
        for (i, &byte) in buffer.iter().enumerate() {
            let addr = address + i as u16;
            self.update_byte(addr, byte);
        }
        Ok(())
    }
}


pub struct SerialListener<'a, R> 
where 
    R: Read<u8>,
{
    rx: &'a mut R,
    counter: usize,
    serial_buffer: [u8; 128]
}

impl<'a, R> SerialListener<'a, R> 
where 
    R: Read<u8>
{

    pub fn new(rx: &'a mut R) -> Self {
        let serial_buffer = [0u8; 128];
        let counter: usize = 0;
        Self { rx, counter, serial_buffer }
    }
    pub fn listen<F>(&mut self, mut handler: F) 
        where F : FnMut(&str)
        {
            if let Ok(byte) = self.rx.read() {
                if self.counter < self.serial_buffer.len()  {
                    self.serial_buffer[self.counter] = byte;
                    self.counter += 1;

                    if byte == b'\n' || byte == b'\r' {
                        if self.counter > 0 {
                            let _str = str::from_utf8(&self.serial_buffer[..self.counter]).unwrap_or("<invalid utf8>");
                            let _str = _str.trim_end_matches(&['\r', '\n'][..]);
                            self.counter = 0;
                            (handler)(_str);
                        }
                    }
                } else {
                    self.counter = 0;
                }
            }
            

    }
}

#[allow(non_camel_case_types)]
pub struct kariSPI<S, C> {
    spi: S,
    cs: C
}

impl<S, C> kariSPI<S, C> 
where 
    S: SpiBus<u8>,
    C: embedded_hal::digital::OutputPin
{

    pub fn new(spi: S, cs: C) -> Self {
        Self { spi, cs }
    }

    pub fn transfer(&mut self, read: &mut[u8], write:  &mut[u8]) {
        self.cs.set_low().unwrap();
        self.spi.transfer(read, write).unwrap();
        self.cs.set_high().unwrap();
    }

    pub fn write(&mut self, buffer: &mut[u8]) {
        self.cs.set_low().unwrap();
        self.spi.write(buffer).unwrap();
        self.cs.set_high().unwrap();
    }

    pub fn read(&mut self, buffer: &mut[u8]) {
        self.cs.set_low().unwrap();
        self.spi.read(buffer).unwrap();
        self.cs.set_high().unwrap();
    }
}