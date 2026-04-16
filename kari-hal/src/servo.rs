#[cfg(feature = "esp")]
use esp_hal::gpio::OutputPin;
#[cfg(feature = "esp")]
use esp_hal::ledc::{Ledc, channel,
timer::{TimerSpeed, TimerIFace, Timer}
};
#[cfg(feature = "esp")]
use embedded_hal_compat::eh1_0::pwm::SetDutyCycle;


#[cfg(feature = "esp")]
use crate::{configure_pwm};


#[cfg(feature = "esp")]
#[allow(non_camel_case_types)]
pub struct kariServo<'a, S: TimerSpeed> {
        channel: channel::Channel<'a, S>,
        min_duty: u32,
        duty_gap: u32,
}

#[cfg(feature = "esp")]
impl<'a, S: TimerSpeed> kariServo<'a, S> {

    pub fn new(ledc: &mut Ledc<'a>, 
    pin: impl Into<esp_hal::gpio::AnyPin<'a>> + OutputPin + 'a, 
    timer: &'a impl TimerIFace<S>,
    channel_num: Option<channel::Number>) -> kariServo<'a, S> {
        let channel_num = channel_num.unwrap_or(channel::Number::Channel0);
        let channel = configure_pwm(ledc, channel_num, pin, timer);
        let max_duty_cycle = channel.max_duty_cycle() as u32;
        let min_duty = (25 * max_duty_cycle) / 1000;
        let max_duty = (125 * max_duty_cycle) / 1000;
        let duty_gap = max_duty - min_duty;

        Self { channel, min_duty, duty_gap }
        
    }

    // #[cfg(any(feature = "esp32s2", feature = "esp32s3", feature = "esp32c3", feature = "esp32c6"))]
    // pub fn new(ledc: &mut Ledc<'a>, 
    // pin: impl Into<esp_hal::gpio::AnyPin<'a>> + OutputPin + 'a, 
    // timer: &'a impl TimerIFace<LowSpeed>,
    // channel_num: Option<channel::Number>) -> kariServo<'a> {
    //     let channel_num = channel_num.unwrap_or(channel::Number::Channel0);
    //     let channel = configure_pwm(ledc, channel_num, pin, timer);
    //     let max_duty_cycle = channel.max_duty_cycle() as u32;
    //     let min_duty = (25 * max_duty_cycle) / 1000;
    //     let max_duty = (125 * max_duty_cycle) / 1000;
    //     let duty_gap = max_duty - min_duty;

    //     Self { channel: kariChannel::LS(channel), min_duty, duty_gap }
        
    // }

    pub fn set_angle(&mut self, angle: u32) {
        let duty = self.duty_from_angle(angle, self.min_duty, self.duty_gap);
        self.channel.set_duty_cycle(duty).unwrap();
    }

    fn duty_from_angle(&mut self, angle: u32, min_duty: u32, duty_gap: u32) -> u16 {
          let duty = min_duty + ((angle * duty_gap) / 180);
          duty as u16
     }
}



#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
#[allow(non_camel_case_types)]
pub struct kariServo {
    pin: u8,
}


#[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
#[allow(unused_unsafe)]
impl kariServo {
    pub fn new(pin: u8) -> Self {
        unsafe {
            let dp = arduino_hal::Peripherals::steal();
            let tc1 = dp.TC1;
            #[cfg(feature = "mega")]
            let tc3 = dp.TC3;
            #[cfg(feature = "mega")]
            let tc4 = dp.TC4;
            #[cfg(feature = "mega")]
            let tc5 = dp.TC5;

            match pin {
                #[cfg(feature = "mega")]
                2 =>{
                    tc3.tccr3a.modify(|_, w| w.wgm3().bits(0b10).com3b().match_clear());
                    tc3.tccr3b.write(|w| w.wgm3().bits(0b11).cs3().prescale_8());
                    tc3.icr3.write(|w| unsafe { w.bits(39_999) });
                    tc3.ocr3b.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(feature = "mega")]
                3 =>{
                    tc3.tccr3a.modify(|_, w| w.wgm3().bits(0b10).com3c().match_clear());
                    tc3.tccr3b.write(|w| w.wgm3().bits(0b11).cs3().prescale_8());
                    tc3.icr3.write(|w| unsafe { w.bits(39_999) });
                    tc3.ocr3c.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(feature = "mega")]
                5 =>{
                    tc3.tccr3a.modify(|_, w| w.wgm3().bits(0b10).com3a().match_clear());
                    tc3.tccr3b.write(|w| w.wgm3().bits(0b11).cs3().prescale_8());
                    tc3.icr3.write(|w| unsafe { w.bits(39_999) });
                    tc3.ocr3a.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(feature = "mega")]
                6 =>{
                    tc4.tccr4a.modify(|_, w| w.wgm4().bits(0b10).com4a().match_clear());
                    tc4.tccr4b.write(|w| w.wgm4().bits(0b11).cs4().prescale_8());
                    tc4.icr4.write(|w| unsafe { w.bits(39_999) });
                    tc4.ocr4a.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(feature = "mega")]
                7 =>{
                    tc4.tccr4a.modify(|_, w| w.wgm4().bits(0b10).com4b().match_clear());
                    tc4.tccr4b.write(|w| w.wgm4().bits(0b11).cs4().prescale_8());
                    tc4.icr4.write(|w| unsafe { w.bits(39_999) });
                    tc4.ocr4b.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(feature = "mega")]
                8 =>{
                    tc4.tccr4a.modify(|_, w| w.wgm4().bits(0b10).com4c().match_clear());
                    tc4.tccr4b.write(|w| w.wgm4().bits(0b11).cs4().prescale_8());
                    tc4.icr4.write(|w| unsafe { w.bits(39_999) });
                    tc4.ocr4c.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(any(feature = "uno", feature = "nano"))]
                9 =>{
                    tc1.tccr1a.modify(|_, w| w.wgm1().bits(0b10).com1a().match_clear());
                    tc1.tccr1b.write(|w| w.wgm1().bits(0b11).cs1().prescale_8());
                    tc1.icr1.write(|w| unsafe { w.bits(39_999) });
                    tc1.ocr1a.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(any(feature = "uno", feature = "nano"))]
                10 =>{
                    tc1.tccr1a.modify(|_, w| w.wgm1().bits(0b10).com1b().match_clear());
                    tc1.tccr1b.write(|w| w.wgm1().bits(0b11).cs1().prescale_8());
                    tc1.icr1.write(|w| unsafe { w.bits(39_999) });
                    tc1.ocr1b.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(feature = "mega")]
                11 =>{
                    //tc1.tccr1a.write(|w| w.wgm1().bits(0b10).com1a().match_clear());
                    tc1.tccr1a.modify(|_, w| w.wgm1().bits(0b10).com1a().match_clear());
                    tc1.tccr1b.write(|w| w.wgm1().bits(0b11).cs1().prescale_8());
                    tc1.icr1.write(|w| unsafe { w.bits(39_999) });
                    tc1.ocr1a.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(feature = "mega")]
                12 =>{
                    //tc1.tccr1a.write(|w| w.wgm1().bits(0b10).com1b().match_clear());
                    tc1.tccr1a.modify(|_, w| w.wgm1().bits(0b10).com1b().match_clear());
                    tc1.tccr1b.write(|w| w.wgm1().bits(0b11).cs1().prescale_8());
                    tc1.icr1.write(|w| unsafe { w.bits(39_999) });
                    tc1.ocr1b.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(feature = "mega")]
                13 =>{
                    //tc1.tccr1a.write(|w| w.wgm1().bits(0b10).com1c().match_clear());
                    tc1.tccr1a.modify(|_, w| w.wgm1().bits(0b10).com1c().match_clear());
                    tc1.tccr1b.write(|w| w.wgm1().bits(0b11).cs1().prescale_8());
                    tc1.icr1.write(|w| unsafe { w.bits(39_999) });
                    tc1.ocr1c.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(feature = "mega")]
                44 =>{
                    tc5.tccr5a.modify(|_, w| w.wgm5().bits(0b10).com5c().match_clear());
                    tc5.tccr5b.write(|w| w.wgm5().bits(0b11).cs5().prescale_8());
                    tc5.icr5.write(|w| unsafe { w.bits(39_999) });
                    tc5.ocr5c.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(feature = "mega")]
                45 =>{
                    tc5.tccr5a.modify(|_, w| w.wgm5().bits(0b10).com5b().match_clear());
                    tc5.tccr5b.write(|w| w.wgm5().bits(0b11).cs5().prescale_8());
                    tc5.icr5.write(|w| unsafe { w.bits(39_999) });
                    tc5.ocr5b.write(|w| unsafe { w.bits(2000) });
                }
                #[cfg(feature = "mega")]
                46 =>{
                    //tc5.tccr5a.write(|w| w.wgm1().bits(0b10).com1c().match_clear());
                    tc5.tccr5a.modify(|_, w| w.wgm5().bits(0b10).com5a().match_clear());
                    tc5.tccr5b.write(|w| w.wgm5().bits(0b11).cs5().prescale_8());
                    tc5.icr5.write(|w| unsafe { w.bits(39_999) });
                    tc5.ocr5a.write(|w| unsafe { w.bits(2000) });
                }
                _ => panic!("Invalid pin for servo. Use 2, 3, 5, 6, 7, 8, 11, 12, or 13."),
            }
            Self { pin }
        }
    }

    pub fn set_angle(&mut self, angle: u8) {
        let ticks = (999 + ((angle as u32 * 4000) / 180)) as u16;
        let dp = unsafe { arduino_hal::Peripherals::steal() };
        let tc1 = dp.TC1;
        #[cfg(feature = "mega")]
        let tc3 = dp.TC3;
        #[cfg(feature = "mega")]
        let tc4 = dp.TC4;
        #[cfg(feature = "mega")]
        let tc5 = dp.TC5;
       match self.pin {
            #[cfg(feature = "mega")]
            2 => tc3.ocr3b.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(feature = "mega")]
            3 => tc3.ocr3c.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(feature = "mega")]
            5 => tc3.ocr3a.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(feature = "mega")]
            6 => tc4.ocr4a.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(feature = "mega")]
            7 => tc4.ocr4b.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(feature = "mega")]
            8 => tc4.ocr4c.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(any(feature = "uno", feature = "nano"))]
            9 => tc1.ocr1a.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(any(feature = "uno", feature = "nano"))]
            10 => tc1.ocr1b.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(feature = "mega")]
            11 => tc1.ocr1a.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(feature = "mega")]
            12 => tc1.ocr1b.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(feature = "mega")]
            13 => tc1.ocr1c.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(feature = "mega")]
            44 => tc5.ocr5c.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(feature = "mega")]
            45 => tc5.ocr5b.write(|w| unsafe { w.bits(ticks) }),
            #[cfg(feature = "mega")]
            46 => tc5.ocr5a.write(|w| unsafe { w.bits(ticks) }),
            _ => unreachable!()
        }
    }
}