#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){

    Serial!(9600);


#[run]
fn run(){


    kariAsync!(||{
            kprintln!("schedule1: {}", millis());
            kprintln!("I come after half a second");
    }, 500, task1);

    kariAsync!(||{
            kprintln!("schedule2: {}", millis());
            kprintln!("I come after second");
            
    }, 1000, task2);

}
}