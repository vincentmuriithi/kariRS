#![allow(non_upper_case_globals)]

use proc_macro::{TokenStream};
use quote::{format_ident, quote};
use syn::{Expr, ExprAssign, ExprClosure, Ident, ItemFn, LitInt, Token, parse::Parse, parse_macro_input, punctuated::{self, Punctuated}, token::Token};
mod structs;
use structs::*;
use quote::ToTokens;
mod func;
mod apis;
mod cores;
mod gpio;
mod controls;




#[allow(non_snake_case)]
mod _Names {
    pub const _kariAnalogPin: &str = "_kariAnalogPin";
    pub const _kariPWMPin: &str = "_kariPWMPin";
    pub const _kariOutputPin: &str = "_kariOutputPin";
    pub const _kariInputPin: &str = "_kariInputPin";
}


#[proc_macro_attribute]
pub fn init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let user_fn = parse_macro_input!(item as ItemFn);
    let user_fn_name = &user_fn.sig.ident;
    let _fn_block = &user_fn.block;
    let fn_statements = &user_fn.block.stmts;
    let fn_attributes = &user_fn.attrs;
    let fn_visibility = &user_fn.vis;


    let expanded = quote! {
        pub static __KARI_INIT: fn() -> ! = #user_fn_name;
        #(#fn_attributes)*
        #fn_visibility fn #user_fn_name() -> !{
            let dp = arduino_hal::Peripherals::take().unwrap();
            let tc0 = dp.TC0;
            let _kariEEPROM = dp.EEPROM;

            // CTC mode (Clear Timer on Compare Match)
            tc0.tccr0a.write(|w| w.wgm0().ctc());

            // Prescaler clk/64
            tc0.tccr0b.write(|w| w.cs0().prescale_64());

            // Compare value for ~1ms tick at 16 MHz
            tc0.ocr0a.write(|w| unsafe { w.bits(250) });

            // Enable Output Compare A Match interrupt
            tc0.timsk0.write(|w| w.ocie0a().set_bit());


            // i wrote this configuartion when testing ultarsonc control
            // let tc1 = &dp.TC1;
            // tc1.tccr1b.write(|w| w.cs1().prescale_64());


            // Both the commented and the uncommented kari_adcconfig work well i just used the manual seting one
            // as i was just experimenting but any works fine the deafult or the one i manually set
            let mut kari_adc = arduino_hal::Adc::new(dp.ADC, AdcSettings {
                clock_divider: ClockDivider::Factor128,
                ref_voltage: ReferenceVoltage::AVcc
            });

            // let mut kari_adc = arduino_hal::Adc::new(dp.ADC, Default::default());

            let mut _kari_timer1 = arduino_hal::simple_pwm::Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
            #[cfg(any(feature = "uno"))]
            let mut _kari_timer2 = arduino_hal::simple_pwm::Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
            #[cfg(any(feature = "mega"))]
            let mut _kari_timer3 = arduino_hal::simple_pwm::Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);
            #[cfg(feature = "mega")]
            let mut _kari_timer4 = arduino_hal::simple_pwm::Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);
            #[cfg(feature = "mega")]
            let mut _kari_timer5 = arduino_hal::simple_pwm::Timer5Pwm::new(dp.TC5, Prescaler::Prescale64);
            //#[cfg(feature = "mega")]
            let mut _kari_timer0 = arduino_hal::simple_pwm::Timer0Pwm::new(tc0, Prescaler::Prescale64);

            let kariPins = arduino_hal::pins!(dp);
            type DynOutputPin = dyn OutputPin<Error = Infallible>;
            let mut _kariPinsTable: [Option<NonNull<DynOutputPin>>; 80] = [None; 80];

            #(#fn_statements)*

            '_kari_loop:loop {
                    run();
                }
        }
    };

    expanded.into()
} 


#[proc_macro_attribute]
pub fn run(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let user_fn = parse_macro_input!(item as ItemFn);
    let _user_fn_name = &user_fn.sig.ident;
    let fn_block = &user_fn.block;


    let expanded = quote! {
        let mut run = || #fn_block;
       
    };

    expanded.into()
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn pinMode(input: TokenStream) -> TokenStream {
    gpio::pinMode::expand_pinMode(input)
}


#[allow(non_snake_case)]
#[proc_macro]
pub fn digitalWrite(input: TokenStream) -> TokenStream{
   gpio::digitalWrite::expand_digitalWrite(input)
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn digitalRead(input: TokenStream) -> TokenStream {
   gpio::digitalRead::expand_digitalRead(input)
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn analogWrite(input: TokenStream) -> TokenStream {
    gpio::analogWrite::expand_analogWrite(input)
}



#[allow(non_snake_case)]
#[proc_macro]
pub fn analogRead(input: TokenStream) -> TokenStream {
    gpio::analogRead::expand_analogRead(input)
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn Serial(input: TokenStream) -> TokenStream {
    cores::serial::expand_serial(input)
}


#[proc_macro]
pub fn kprintln(input: TokenStream) -> TokenStream {
    cores::kprintln::expand_kprintln(input)
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn pulseIn(input: TokenStream) -> TokenStream {
    cores::pulseIn::expand_pulseIn(input)
}

#[proc_macro]
pub fn gen_pins(input: TokenStream) -> TokenStream {
    let pins = parse_macro_input!(input with Punctuated<pinDecl, Token![,]>::parse_terminated);

    let pin_codes = pins.iter().map(|decl|{
        let name = &decl.name;
        let pin = &decl.pin;
        quote! {
            let #name = kariPins.#pin.into_pull_up_input();
        }
    });

    TokenStream::from(
        quote! {
            #(#pin_codes)*
        }
    )
}



#[allow(non_snake_case)]
#[proc_macro]
pub fn kariAsync(input: TokenStream) -> TokenStream {
    controls::kariAsync::expand_kariAsync(input)
}


#[allow(non_snake_case)]
#[proc_macro]
pub fn kariPulse(input: TokenStream) -> TokenStream {
   controls::kariPulse::expand_kariPulse(input)
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn kariSequential(input: TokenStream) -> TokenStream {
    controls::kariSequential::expand_kariSequential(input)
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn kariBegin(input: TokenStream) -> TokenStream {
   controls::kariBegin::expand_kariBegin(input)
}


#[allow(non_snake_case)]
#[proc_macro]
pub fn kariJoyStick(input: TokenStream) -> TokenStream {
    apis::kariJoyStick::expand_kariJoyStick(input)
}


#[allow(non_snake_case)]
#[proc_macro]
pub fn kariPH(input: TokenStream) -> TokenStream {
    apis::kariPH::expand_kariPH(input)
}



#[allow(non_snake_case)]
#[proc_macro]
pub fn kariDrive(input: TokenStream) -> TokenStream {
    apis::kariDrive::expand_kariDrive(input)
}


#[allow(non_snake_case)]
#[proc_macro]
pub fn kariPIR(input: TokenStream) -> TokenStream {
    apis::kariPIR::expand_kariPIR(input)
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn kariInfrared(input: TokenStream) -> TokenStream {
    apis::kariInfrared::expand_kariInfrared(input)
}


#[proc_macro_attribute]
pub fn eeprom(attr: TokenStream, item: TokenStream) -> TokenStream {
    cores::eeprom::expand_eeprom(attr, item)
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn kariSPI(input: TokenStream) -> TokenStream {
    cores::kariSPI::expand_kariSPI(input)
}