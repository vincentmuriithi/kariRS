#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PLL Status and Control register"]
    pub pllcsr: PLLCSR,
}
#[doc = "PLLCSR (rw) register accessor: an alias for `Reg<PLLCSR_SPEC>`"]
pub type PLLCSR = crate::Reg<pllcsr::PLLCSR_SPEC>;
#[doc = "PLL Status and Control register"]
pub mod pllcsr;
