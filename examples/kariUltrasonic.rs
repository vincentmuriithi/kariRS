#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;

// This kariUltrasonic syntax API is only supported in  AVR only 

#[init]
fn setup(){
    Serial!(9600);
    gen_pins!(sda: d20, scl: d21, trig: d32 output, echo: d33 floating_input);

    let mut hcsr = kariUltrasonic::new(trig, echo, _kari_delay);
    let mut distance = hcsr.measure_distance();
    
   
    kprintln!("Distance: {}", distance);

#[run]
fn run() {

  
    kariAsync!(|| {
        distance = hcsr.measure_distance();
        kprintln!("Distance: {}", distance);
    }, 1000, ultrasonic_measurements);

}
}