#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port B Input Pins"]
    pub pinb: PINB,
    #[doc = "0x01 - Data Direction Register, Port B"]
    pub ddrb: DDRB,
    #[doc = "0x02 - Port B Data Register"]
    pub portb: PORTB,
    #[doc = "0x03 - Pull-up Enable Control Register"]
    pub pueb: PUEB,
}
#[doc = "DDRB (rw) register accessor: an alias for `Reg<DDRB_SPEC>`"]
pub type DDRB = crate::Reg<ddrb::DDRB_SPEC>;
#[doc = "Data Direction Register, Port B"]
pub mod ddrb;
#[doc = "PINB (rw) register accessor: an alias for `Reg<PINB_SPEC>`"]
pub type PINB = crate::Reg<pinb::PINB_SPEC>;
#[doc = "Port B Input Pins"]
pub mod pinb;
#[doc = "PORTB (rw) register accessor: an alias for `Reg<PORTB_SPEC>`"]
pub type PORTB = crate::Reg<portb::PORTB_SPEC>;
#[doc = "Port B Data Register"]
pub mod portb;
#[doc = "PUEB (rw) register accessor: an alias for `Reg<PUEB_SPEC>`"]
pub type PUEB = crate::Reg<pueb::PUEB_SPEC>;
#[doc = "Pull-up Enable Control Register"]
pub mod pueb;
