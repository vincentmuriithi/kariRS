#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control Register"]
    pub wdtcsr: WDTCSR,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Watchdog Timer Clock Divider"]
    pub wdtckd: WDTCKD,
}
#[doc = "WDTCKD (rw) register accessor: an alias for `Reg<WDTCKD_SPEC>`"]
pub type WDTCKD = crate::Reg<wdtckd::WDTCKD_SPEC>;
#[doc = "Watchdog Timer Clock Divider"]
pub mod wdtckd;
#[doc = "WDTCSR (rw) register accessor: an alias for `Reg<WDTCSR_SPEC>`"]
pub type WDTCSR = crate::Reg<wdtcsr::WDTCSR_SPEC>;
#[doc = "Watchdog Timer Control Register"]
pub mod wdtcsr;
