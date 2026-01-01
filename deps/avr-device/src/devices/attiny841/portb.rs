#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port B Data register"]
    pub pinb: PINB,
    #[doc = "0x01 - Data Direction Register, Port B"]
    pub ddrb: DDRB,
    #[doc = "0x02 - Input Pins, Port B"]
    pub portb: PORTB,
    _reserved3: [u8; 0x29],
    #[doc = "0x2c - Pull-up Enable Control Register"]
    pub pueb: PUEB,
    _reserved4: [u8; 0x01],
    #[doc = "0x2e - Port Control Register"]
    pub portcr: PORTCR,
}
#[doc = "DDRB (rw) register accessor: an alias for `Reg<DDRB_SPEC>`"]
pub type DDRB = crate::Reg<ddrb::DDRB_SPEC>;
#[doc = "Data Direction Register, Port B"]
pub mod ddrb;
#[doc = "PINB (rw) register accessor: an alias for `Reg<PINB_SPEC>`"]
pub type PINB = crate::Reg<pinb::PINB_SPEC>;
#[doc = "Port B Data register"]
pub mod pinb;
#[doc = "PORTB (rw) register accessor: an alias for `Reg<PORTB_SPEC>`"]
pub type PORTB = crate::Reg<portb::PORTB_SPEC>;
#[doc = "Input Pins, Port B"]
pub mod portb;
#[doc = "PORTCR (rw) register accessor: an alias for `Reg<PORTCR_SPEC>`"]
pub type PORTCR = crate::Reg<portcr::PORTCR_SPEC>;
#[doc = "Port Control Register"]
pub mod portcr;
#[doc = "PUEB (rw) register accessor: an alias for `Reg<PUEB_SPEC>`"]
pub type PUEB = crate::Reg<pueb::PUEB_SPEC>;
#[doc = "Pull-up Enable Control Register"]
pub mod pueb;
