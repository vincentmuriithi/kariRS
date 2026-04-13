pub use embedded_dht_rs::dht11::Dht11;
use embedded_dht_rs::dht22::Dht22;
use embedded_dht_rs::dht20::Dht20;
pub use embedded_dht_rs;
use embedded_hal::digital::InputPin;
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;
use super::kariI2C;


#[allow(non_camel_case_types)]
pub struct kariDhtMeasurements<T> {
    pub temperature: T,
    pub humidity: T
}

#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
#[allow(non_camel_case_types)]
pub struct kariDHT11<Pin, Delay> 
where
    Pin: InputPin + OutputPin,
    Delay: DelayNs,
{
    instance: Dht11<Pin, Delay>
}

#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
impl<Pin, Delay> kariDHT11<Pin, Delay> 
where
    Pin: InputPin + OutputPin,
    Delay: DelayNs,
{
    pub fn new(pin: Pin, delay: Delay) -> Self {
        let instance = Dht11::new(pin, delay);
        Self { instance }
    }

    pub fn read(&mut self) -> Result<kariDhtMeasurements<u8>, embedded_dht_rs::SensorError> {
        let measurement = self.instance.read()?;
        Ok(kariDhtMeasurements {
            temperature: measurement.temperature,
            humidity:measurement.humidity
        })
    }

    
}


#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
#[allow(non_camel_case_types)]
pub struct kariDHT22<Pin, Delay> 
where
    Pin: InputPin + OutputPin,
    Delay: DelayNs,
{
    instance: Dht22<Pin, Delay>
}
#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
impl<Pin, Delay> kariDHT22<Pin, Delay> 
where
    Pin: InputPin + OutputPin,
    Delay: DelayNs,
{
    pub fn new(pin: Pin, delay: Delay) -> Self {
        let instance = Dht22::new(pin, delay);
        Self { instance }
    }

    pub fn read(&mut self) -> Result<kariDhtMeasurements<f32>, embedded_dht_rs::SensorError> {
        let measurement = self.instance.read()?;
        Ok(kariDhtMeasurements {
            temperature: measurement.temperature,
            humidity:measurement.humidity
        })
    }

    
}

#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
#[allow(non_camel_case_types)]
pub struct kariDHT20<I2C, Delay> 
where
    I2C: kariI2C + embedded_hal::i2c::I2c,
    Delay: DelayNs,
{
    instance: Dht20<I2C, Delay>
}

#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
impl<I2C, Delay> kariDHT20<I2C, Delay> 
where
    I2C: kariI2C + embedded_hal::i2c::I2c,
    Delay: DelayNs,
{
    pub fn new(i2c: I2C, delay: Delay) -> Self {
        let instance = Dht20::new(i2c, delay);
        Self { instance }
    }

    pub fn read(&mut self) -> Result<kariDhtMeasurements<f32>, embedded_dht_rs::SensorError> {
        let measurement = self.instance.read()?;
        Ok(kariDhtMeasurements {
            temperature: measurement.temperature,
            humidity:measurement.humidity
        })
    }

    
}