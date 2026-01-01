#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Pins, Port B"]
    pub pinb: PINB,
    #[doc = "0x01 - Data Direction Register, Port B"]
    pub ddrb: DDRB,
    #[doc = "0x02 - Data Register, Port B"]
    pub portb: PORTB,
}
#[doc = "DDRB (rw) register accessor: an alias for `Reg<DDRB_SPEC>`"]
pub type DDRB = crate::Reg<ddrb::DDRB_SPEC>;
#[doc = "Data Direction Register, Port B"]
pub mod ddrb;
#[doc = "PINB (rw) register accessor: an alias for `Reg<PINB_SPEC>`"]
pub type PINB = crate::Reg<pinb::PINB_SPEC>;
#[doc = "Input Pins, Port B"]
pub mod pinb;
#[doc = "PORTB (rw) register accessor: an alias for `Reg<PORTB_SPEC>`"]
pub type PORTB = crate::Reg<portb::PORTB_SPEC>;
#[doc = "Data Register, Port B"]
pub mod portb;
