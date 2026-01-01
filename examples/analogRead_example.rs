#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;


#[init]
fn setup() {
    Serial!(9600);
    pinMode!(0, _INPUT); // analog pin

    #[run]
    fn run() {
        let value = analogRead!(0);
        kprintln!("Analog value: {}", value);
        delay(300);
    }
}