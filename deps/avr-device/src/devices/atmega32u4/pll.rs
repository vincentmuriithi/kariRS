#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PLL Status and Control register"]
    pub pllcsr: PLLCSR,
    _reserved1: [u8; 0x08],
    #[doc = "0x09 - PLL Frequency Control Register"]
    pub pllfrq: PLLFRQ,
}
#[doc = "PLLCSR (rw) register accessor: an alias for `Reg<PLLCSR_SPEC>`"]
pub type PLLCSR = crate::Reg<pllcsr::PLLCSR_SPEC>;
#[doc = "PLL Status and Control register"]
pub mod pllcsr;
#[doc = "PLLFRQ (rw) register accessor: an alias for `Reg<PLLFRQ_SPEC>`"]
pub type PLLFRQ = crate::Reg<pllfrq::PLLFRQ_SPEC>;
#[doc = "PLL Frequency Control Register"]
pub mod pllfrq;
