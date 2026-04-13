#![cfg(feature = "esp")]

use esp_hal::gpio::OutputPin;
use esp_hal::analog::adc::Adc;
use esp_hal::ledc::{Ledc, channel, LowSpeed, HighSpeed,
    channel::ChannelIFace,
    timer::{TimerSpeed, TimerIFace, Timer}
};


#[allow(non_camel_case_types)]
pub enum kariChannel<'a> {
    LS(channel::Channel<'a, LowSpeed>),
    HS(channel::Channel<'a, HighSpeed>),
}

#[allow(non_camel_case_types)]
pub enum kariAnyADC<'a, DM>
where 
    DM: esp_hal::DriverMode
{
    ADC1(Adc<'a, esp_hal::peripherals::ADC1<'a>, DM>),
    ADC2(Adc<'a, esp_hal::peripherals::ADC2<'a>, DM>),
}

#[allow(non_camel_case_types)]
pub enum kariADCType<ADC1PIN, ADC2PIN> {
    ADC1(ADC1PIN),
    ADC2(ADC2PIN),
}

pub fn configure_pwm<'a, S: TimerSpeed>(ledc: &mut Ledc<'a>, 
channel_num: channel::Number, 
pin: impl Into<esp_hal::gpio::AnyPin<'a>> + OutputPin + 'a,
timer: &'a impl TimerIFace<S>
) 
-> channel::Channel<'a, S>
{
    let mut channel = ledc.channel(channel_num, pin);
    channel
    .configure(channel::config::Config {
        timer,
        duty_pct: 10,
        drive_mode: esp_hal::gpio::DriveMode::PushPull,
    })
    .unwrap();

    channel
}


pub fn map_channel(idx: usize) -> (channel::Number, &'static str) {
    match idx {
        0 => (channel::Number::Channel0, "channel0"),
        1 => (channel::Number::Channel1, "channel1"),
        2 => (channel::Number::Channel2, "channel2"),
        3 => (channel::Number::Channel3, "channel3"),
        4 => (channel::Number::Channel4, "channel4"),
        5 => (channel::Number::Channel5, "channel5"),
        6 => (channel::Number::Channel6, "channel6"),
        7 => (channel::Number::Channel7, "channel7"),
        _ => unreachable!("Only 8 channels are available in each channel group"),
    }
}