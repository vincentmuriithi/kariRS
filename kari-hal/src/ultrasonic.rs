use embedded_hal::{digital::{InputPin, OutputPin}};
use embedded_hal::delay::DelayNs; 


#[allow(non_camel_case_types)]
pub struct kariUltrasonic<TRIGPIN, ECHOPIN, DELAY> {
    trig: TRIGPIN,
    echo: ECHOPIN,
    delay: DELAY
}

impl<TRIGPIN, ECHOPIN, DELAY> kariUltrasonic<TRIGPIN, ECHOPIN, DELAY> 
where 
    TRIGPIN: OutputPin,
    ECHOPIN: InputPin,
    DELAY: DelayNs,
{
    pub fn new(trig: TRIGPIN, echo: ECHOPIN, delay: DELAY) -> Self {
        Self {trig, echo, delay}
    }

    pub fn measure_distance(&mut self) -> u32 {
        self.trig.set_low().ok();
        self.delay.delay_us(2);
        self.trig.set_high().ok();
        self.delay.delay_us(10);
        self.trig.set_low().ok();

        while self.echo.is_low().unwrap() {}
        let mut elapsed = 0u32;
        while self.echo.is_high().unwrap() {
            self.delay.delay_us(1);
            elapsed += 1;
        }

        (elapsed as f32 * 0.034 / 2.0) as u32


    }
}



