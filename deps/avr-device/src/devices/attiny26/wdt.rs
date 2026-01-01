#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control Register"]
    pub wdtcr: WDTCR,
}
#[doc = "WDTCR (rw) register accessor: an alias for `Reg<WDTCR_SPEC>`"]
pub type WDTCR = crate::Reg<wdtcr::WDTCR_SPEC>;
#[doc = "Watchdog Timer Control Register"]
pub mod wdtcr;
