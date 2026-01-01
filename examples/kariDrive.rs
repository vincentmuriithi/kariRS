#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){
        let delay_time = 900;
        kariBegin!([22, 16, 18, 19, 17], OUTPUT);
        kariBegin!([2, 4], PWM);
        kariBegin!([1, 2], _INPUT);

        Serial!(9600);
      

        kprintln!("Hello from kariRS kprintln macro!");
        kprintln!("Hello from kariRS println macro!");
        kprintln!("Hello from kariRS!\r");
      let mut car = kariDrive!(2, 16, 17, 4, 18, 19);
      
        kprintln!("Entering loop...\r");
        
 

#[run]
fn run(){
              
    car.drive(120, false); // move in forward direction
    delay(delay_time)
    car.drive(120, false); // move in backward  direction
    delay(delay_time)
    car.right(45, false); // move right in forward direction
    delay(delay_time)
    car.right(45, true);  // move right in backward direction
    delay(delay_time)
    car.left(45, false);  // move left in forward direction
    delay(delay_time)
    car.left(45, true);   // move left in backward  direction
    delay(delay_time)
              
}

}