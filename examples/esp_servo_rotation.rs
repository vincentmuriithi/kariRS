#![no_std]
#![no_main]

mod kari_entry;
use esp_hal::ledc::timer::config;
use kari_entry::*;



#[init]
fn setup(){
     // Generating pin for use with servo
    gen_pins!(servo_pin: d13);

     // Creating a servo instance
    let mut servo = kariServo::new(&mut _kari_ledc, servo_pin, &_kari_hstimer1, None);

    // Creating another servo instance on a different pin and channel
    //gen_pins!(servo_pin1: d33);    
    //let mut servo2 = Servo::new(&mut _kari_ledc, servo_pin1, &_kari_hstimer1, Some(channelNumber::Channel1));


#[run]
fn run(){


     for deg in (0..=180).chain((0..=179).rev()){ 
          servo.set_angle(deg);
          delay(10);  
     }



}

}

