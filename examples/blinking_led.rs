#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;


#[init]
fn setup() {
    pinMode!(13, OUTPUT);

    #[run]
    fn run() {
        digitalWrite!(13, HIGH);
        delay(500);
        digitalWrite!(13, LOW);
        delay(500);
    }
}