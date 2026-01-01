#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port D Input Pins"]
    pub pind: PIND,
    #[doc = "0x01 - Port D Data Direction Register"]
    pub ddrd: DDRD,
    #[doc = "0x02 - Port D Data Register"]
    pub portd: PORTD,
}
#[doc = "DDRD (rw) register accessor: an alias for `Reg<DDRD_SPEC>`"]
pub type DDRD = crate::Reg<ddrd::DDRD_SPEC>;
#[doc = "Port D Data Direction Register"]
pub mod ddrd;
#[doc = "PIND (rw) register accessor: an alias for `Reg<PIND_SPEC>`"]
pub type PIND = crate::Reg<pind::PIND_SPEC>;
#[doc = "Port D Input Pins"]
pub mod pind;
#[doc = "PORTD (rw) register accessor: an alias for `Reg<PORTD_SPEC>`"]
pub type PORTD = crate::Reg<portd::PORTD_SPEC>;
#[doc = "Port D Data Register"]
pub mod portd;
