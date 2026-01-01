#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;

#[init]
fn setup() {
    pinMode!(7, PWM);

    #[run]
    fn run() {
        analogWrite!(7, 128); // 50% duty cycle
        delay(1000);
    }
}