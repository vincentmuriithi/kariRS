#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reference Voltage Calibration Register"]
    pub bgcr: BGCR,
    _reserved1: [u8; 0x0d],
    #[doc = "0x0e - Flash Extended-Mode Control-Register"]
    pub nemcr: NEMCR,
}
#[doc = "BGCR (rw) register accessor: an alias for `Reg<BGCR_SPEC>`"]
pub type BGCR = crate::Reg<bgcr::BGCR_SPEC>;
#[doc = "Reference Voltage Calibration Register"]
pub mod bgcr;
#[doc = "NEMCR (rw) register accessor: an alias for `Reg<NEMCR_SPEC>`"]
pub type NEMCR = crate::Reg<nemcr::NEMCR_SPEC>;
#[doc = "Flash Extended-Mode Control-Register"]
pub mod nemcr;
