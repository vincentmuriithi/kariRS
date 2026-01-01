#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port D Input Pins"]
    pub pind: PIND,
    #[doc = "0x01 - Data Direction Register, Port D"]
    pub ddrd: DDRD,
    #[doc = "0x02 - Port D Data Register"]
    pub portd: PORTD,
    #[doc = "0x03 - Pull-up Enable Control Register"]
    pub pued: PUED,
}
#[doc = "DDRD (rw) register accessor: an alias for `Reg<DDRD_SPEC>`"]
pub type DDRD = crate::Reg<ddrd::DDRD_SPEC>;
#[doc = "Data Direction Register, Port D"]
pub mod ddrd;
#[doc = "PIND (rw) register accessor: an alias for `Reg<PIND_SPEC>`"]
pub type PIND = crate::Reg<pind::PIND_SPEC>;
#[doc = "Port D Input Pins"]
pub mod pind;
#[doc = "PORTD (rw) register accessor: an alias for `Reg<PORTD_SPEC>`"]
pub type PORTD = crate::Reg<portd::PORTD_SPEC>;
#[doc = "Port D Data Register"]
pub mod portd;
#[doc = "PUED (rw) register accessor: an alias for `Reg<PUED_SPEC>`"]
pub type PUED = crate::Reg<pued::PUED_SPEC>;
#[doc = "Pull-up Enable Control Register"]
pub mod pued;
