#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){

    Serial!(9600);

    kprintln!("Hello from kariRS!\r");


    let mut feedback = 50.0;
    let mut step = 0;
    let mut correction: f32 = 0.0;

    let mut correction_str = kariString::<64>::new("");
    let mut feedback_str = kariString::<64>::new("");
    let mut step_str = kariString::<64>::new(step);

    let mut pid = kariPID::new(100.0, 0.5, 0.0001, 80.0); // set point is 100 then kp, ki, kd

        
 

#[run]
fn run(){

    kariAsync!(||{

        correction = pid.evaluate(feedback);
        feedback += correction * 0.5;
        step_str.update(step);
        kprintln!("Step: {} - correction: {} - feedback: {}\r",
            step_str.as_str(), correction_str.format_float(correction, 3),
            feedback_str.format_float(feedback, 3)
        );

        step += 1;
    }, 1000, task3);


      
      
}

}