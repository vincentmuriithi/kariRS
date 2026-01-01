#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PORT K Input Pins"]
    pub pink: PINK,
    #[doc = "0x01 - PORT K Data Direction Register"]
    pub ddrk: DDRK,
    #[doc = "0x02 - PORT K Data Register"]
    pub portk: PORTK,
}
#[doc = "DDRK (rw) register accessor: an alias for `Reg<DDRK_SPEC>`"]
pub type DDRK = crate::Reg<ddrk::DDRK_SPEC>;
#[doc = "PORT K Data Direction Register"]
pub mod ddrk;
#[doc = "PINK (rw) register accessor: an alias for `Reg<PINK_SPEC>`"]
pub type PINK = crate::Reg<pink::PINK_SPEC>;
#[doc = "PORT K Input Pins"]
pub mod pink;
#[doc = "PORTK (rw) register accessor: an alias for `Reg<PORTK_SPEC>`"]
pub type PORTK = crate::Reg<portk::PORTK_SPEC>;
#[doc = "PORT K Data Register"]
pub mod portk;
