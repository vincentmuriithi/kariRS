#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port C Input Pins"]
    pub pinc: PINC,
    #[doc = "0x01 - Port C Data Direction Register"]
    pub ddrc: DDRC,
    #[doc = "0x02 - Port C Data Register"]
    pub portc: PORTC,
}
#[doc = "DDRC (rw) register accessor: an alias for `Reg<DDRC_SPEC>`"]
pub type DDRC = crate::Reg<ddrc::DDRC_SPEC>;
#[doc = "Port C Data Direction Register"]
pub mod ddrc;
#[doc = "PINC (rw) register accessor: an alias for `Reg<PINC_SPEC>`"]
pub type PINC = crate::Reg<pinc::PINC_SPEC>;
#[doc = "Port C Input Pins"]
pub mod pinc;
#[doc = "PORTC (rw) register accessor: an alias for `Reg<PORTC_SPEC>`"]
pub type PORTC = crate::Reg<portc::PORTC_SPEC>;
#[doc = "Port C Data Register"]
pub mod portc;
