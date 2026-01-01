#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){

        gen_pins!(sda: d20, scl: d21);
	// gen_pins!(sda: a4, scl: a5);  for uno, nano i.e atmega328 mcus

        let mut current: u32 = 0;
        let mut previous: u32 = 0;
        let mut current2: u32 = 0;
        let mut previous2: u32 = 0;
        let mut counter = 150;

        
        let i2c = init_i2c(dp.TWI, sda, scl, None);
        //let mut lcd = lcd_init(i2c, None);
        let mut _delay = arduino_hal::Delay::new();

        let mut lcd = kariLcd::new(i2c, None);

        let mut counter_str = kariString::<16>::new(counter);

        lcd.set_cursor_visibility(Cursor::Invisible)
        .set_cursor_blink(CursorBlink::Off)
        .write("kariRS is live!")
        .set_cursor(40).write("powered by kari");

        delay(2000);
        

        Serial!(9600);

        lcd.set_cursor(64).write("Countdown :");
        


#[run]
fn run(){

    current = millis();
    current2 = current;



    if current - previous >= 500 {
            counter -= 1;
            counter_str.update(counter);
            if counter > 0 {
            lcd.set_cursor(75).write("    ").set_cursor(75)
            .write(counter_str.as_str());
            } else{
            counter = 200;
            }
            previous = current;
    }     
}

}