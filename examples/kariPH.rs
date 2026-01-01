#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){

    kariBegin!([1, 2], _INPUT);
    Serial!(9600);


    kprintln!("Hello from kariRS!\r");
    let mut x_str = kariString::<64>::new("");
    let mut y_str = kariString::<64>::new("");

    let mut ph_sensor = kariPH!(1);

    kprintln!("Entering loop...\r");
        

#[run]
fn run(){
    kariAsync!(||{

        ph_sensor
        .measure()
        .onMeasure(|value:f32|{
        kprintln!("{}\r", x_str.format_float(value, 3));
        });

            
    }, 1000, task4);
}
}