#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PORT J Input Pins"]
    pub pinj: PINJ,
    #[doc = "0x01 - PORT J Data Direction Register"]
    pub ddrj: DDRJ,
    #[doc = "0x02 - PORT J Data Register"]
    pub portj: PORTJ,
}
#[doc = "DDRJ (rw) register accessor: an alias for `Reg<DDRJ_SPEC>`"]
pub type DDRJ = crate::Reg<ddrj::DDRJ_SPEC>;
#[doc = "PORT J Data Direction Register"]
pub mod ddrj;
#[doc = "PINJ (rw) register accessor: an alias for `Reg<PINJ_SPEC>`"]
pub type PINJ = crate::Reg<pinj::PINJ_SPEC>;
#[doc = "PORT J Input Pins"]
pub mod pinj;
#[doc = "PORTJ (rw) register accessor: an alias for `Reg<PORTJ_SPEC>`"]
pub type PORTJ = crate::Reg<portj::PORTJ_SPEC>;
#[doc = "PORT J Data Register"]
pub mod portj;
