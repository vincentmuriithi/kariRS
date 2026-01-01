#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){

    Serial!(9600);

    kprintln!("Hello from kariRS!\r");

    let mut x_str = kariString::<64>::new("");
    let mut y_str = kariString::<64>::new("");

    let joy_stick = kariJoyStick!(1, 2, 500, None);

    kprintln!("Entering loop...\r");
        
 
#[run]
fn run(){
    kariAsync!(||{
            
        joy_stick.onX(Some(|value: f32|{
                kprintln!("X value is : {}\r", x_str.format_float(value, 3));
        }), false)

        .onY(Some(|value: f32|{
                kprintln!("Y value is : {}\r", y_str.format_float(value,3));
        }), false);
            
    }, 1000, task4);  
}

}