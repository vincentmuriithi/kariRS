#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){
     pinMode!(9, PWM);

     // Creating a servo instance
    let mut servo = kariServo::new(9);


#[run]
fn run(){


     for deg in (0..=180).chain((0..=179).rev()){ 
          servo.set_angle(deg);
          delay(10);  
     }



}

}

