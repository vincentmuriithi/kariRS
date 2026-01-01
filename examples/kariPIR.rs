#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){
    pinMode!(12, INPUT);
    Serial!(9600);
    kprintln!("Hello from kariRS!\r");
    let mut pir = kariPIR!(12);

#[run]
fn run(){

    kariAsync!(||{
        pir.on_measure(||{
        kprintln!("Motion detected\r");
        kprintln!("Callback function\r");
        }, None::<fn()>)

        .on_measure(||{},
        Some(||{
        kprintln!("No Motion detected\r");
        kprintln!("Fallback function\r");
        }));  
    }, 10, task4);

      
      
}

}