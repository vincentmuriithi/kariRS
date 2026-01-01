#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control Register"]
    pub wdtcsr: WDTCSR,
}
#[doc = "WDTCSR (rw) register accessor: an alias for `Reg<WDTCSR_SPEC>`"]
pub type WDTCSR = crate::Reg<wdtcsr::WDTCSR_SPEC>;
#[doc = "Watchdog Timer Control Register"]
pub mod wdtcsr;
