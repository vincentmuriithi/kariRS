#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Pins, Port D"]
    pub pind: PIND,
    #[doc = "0x01 - Data Direction Register, Port D"]
    pub ddrd: DDRD,
    #[doc = "0x02 - Data Register, Port D"]
    pub portd: PORTD,
}
#[doc = "DDRD (rw) register accessor: an alias for `Reg<DDRD_SPEC>`"]
pub type DDRD = crate::Reg<ddrd::DDRD_SPEC>;
#[doc = "Data Direction Register, Port D"]
pub mod ddrd;
#[doc = "PIND (rw) register accessor: an alias for `Reg<PIND_SPEC>`"]
pub type PIND = crate::Reg<pind::PIND_SPEC>;
#[doc = "Input Pins, Port D"]
pub mod pind;
#[doc = "PORTD (rw) register accessor: an alias for `Reg<PORTD_SPEC>`"]
pub type PORTD = crate::Reg<portd::PORTD_SPEC>;
#[doc = "Data Register, Port D"]
pub mod portd;
