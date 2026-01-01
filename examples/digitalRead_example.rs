#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;

#[init]
fn setup() {
    pinMode!(12, INPUT);
    Serial!(9600);

    #[run]
    fn run() {
        if digitalRead!(12) == HIGH {
            kprintln!("Button pressed");
        }
    }
}