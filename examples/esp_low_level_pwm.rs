#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){
     Serial!(9600);

     //pinMode!(2, OUTPUT);
     // pinMode!(27, OUTPUT);
     // pinMode!(4, INPUT);
     // pinMode!(5, INPUT_PULLUP);
     // pinMode!(17, INPUT_PULLDOWN);
     pinMode!(15, PWM);

     
     gen_pins!(sda: d21, scl: d22, led: d2, led2: d14);

     let i2c = init_i2c(_peripherals.I2C0, sda, scl, None);

     let mut lcd = kariLcd::new(i2c, None);

     let _kari_lstimer_channels: Vec<&str, 8>= Vec::new();

     let mut channel1 = configure_pwm(&mut _kari_ledc, channel::Number::Channel1, led2, &_kari_lstimer0);
     let mut channel0 = configure_pwm(&mut _kari_ledc, channel::Number::Channel0, led, &_kari_lstimer0);
     

     // let mut channel0 = _kari_ledc.channel(channel::Number::Channel0, led);
     // channel0
     // .configure(channel::config::Config {
     //      timer: &_kari_lstimer0,
     //      duty_pct: 10,
     //      drive_mode: DriveMode::PushPull,
     // })
     // .unwrap();
     

     lcd
          .set_cursor_blink(CursorBlink::Off)
          .set_cursor_visibility(Cursor::Invisible)
          .clear()
          .set_cursor(0)
          .write("kariRS in ESP32");

     let mut counter: usize = 0;


    kprintln!("Hello world in kariRS ESP32 Board ufmt");
    let max_duty = channel0.max_duty_cycle();
    kprintln!("Max duty cycle: {}", max_duty);
    





#[run]
fn run(){

     kari_serial_listener.listen(|data|{
          kprintln!("Received data is: {}", data);
     });

     // kariAsync!(||{
     //      kariPulse!([2], pulse1);
     //      kprintln!("Time: {}", millis());

     // }, 800, task1);

     for duty in(0..=100).chain((0..=99).rev()){
          let duty = (powf(duty as f32 / 100f32, 2.2) * max_duty as f32) as u16;
          channel1.set_duty_cycle(duty);
          channel0.set_duty_cycle(duty);
          let duty = (duty as u16 * 255) / max_duty as u16;
          analogWrite!(15, duty);
          //channel0.set_duty(duty);
          delay(10);
     }

     kariAsync!(||{
          kprintln!("Max duty cycle: {}", max_duty);
     }, 2000, duty_value);

}

}

