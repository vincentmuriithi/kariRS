#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){
    Serial!(9600);
    kprintln!("Welcome to kariRS\r");   

#[run]
fn run(){

    kari_serial_listener.listen(|data|{
        kprintln!("{}\r", data);
    });
}

}