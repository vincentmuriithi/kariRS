#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){
   
    Serial!(9600);


    let mut eeprom = kariEEPROM::new(_kariEEPROM);


    kprintln!("Hello from kariRS kprintln macro!");
    kprintln!("Hello from kariRS println macro!");
    kprintln!("Hello from kariRS!\r");
    kprintln!("{}\r", millis());
    kprintln!("{}\r", micros());

    let num = 600i32;
    let bytes = num.to_le_bytes();
    size_of::<u16>();


    eeprom.write(0x00, &[56, 200, 12]);
    eeprom.write(0x03, &bytes);
    eeprom.capacity();
    kprintln!("The data at 0x00 is: {}\r", eeprom.read_byte(0x00));
    kprintln!("The data at 0x01 is: {}\r", eeprom.read_byte(0x01));
    kprintln!("The data at 0x02 is: {}\r", eeprom.read_byte(0x02));
    kprintln!("The data at 0x03 is: {}\r", eeprom.read_byte(0x03));
    kprintln!("The eeprom capacity is {}\r", eeprom.capacity());

    let mut buf = [0u8; 2];
    eeprom.read(0x03, &mut buf);
    let ret = u16::from_le_bytes(buf);
    kprintln!("The retrieved data is: {}\r", ret);

    /// saving and loading structured data

     #[eeprom(0x00)]
        struct Config {
            id: u16,
            flag: u8,
        }
       

        let config = Config { id: 987, flag: 0};
        config.save(&mut eeprom, None);

        let config2 = Config { id: 2026, flag: 220};
        config2.save(&mut eeprom, Config::next_addr(1));
    


        let restored = Config::load(&eeprom, None);
        kprintln!("Struct data: id - {}, flag - {}\r", restored.id, restored.flag);
        kprintln!("Struct size: {}\r", size_of::<Config>());

        let restored2 = Config::load(&eeprom, Config::next_addr(1));
        kprintln!("Struct2 data: id - {}, flag - {}\r", restored2.id, restored2.flag);
        kprintln!("Struct2 size: {}\r", size_of::<Config>());
        

#[run]
fn run(){}

}