#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - On-Chip Debug Register"]
    pub ocdr: OCDR,
    _reserved1: [u8; 0x02],
    #[doc = "0x03 - MCU Status Register"]
    pub mcusr: MCUSR,
    #[doc = "0x04 - MCU Control Register"]
    pub mcucr: MCUCR,
}
#[doc = "MCUCR (rw) register accessor: an alias for `Reg<MCUCR_SPEC>`"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "MCUSR (rw) register accessor: an alias for `Reg<MCUSR_SPEC>`"]
pub type MCUSR = crate::Reg<mcusr::MCUSR_SPEC>;
#[doc = "MCU Status Register"]
pub mod mcusr;
#[doc = "OCDR (rw) register accessor: an alias for `Reg<OCDR_SPEC>`"]
pub type OCDR = crate::Reg<ocdr::OCDR_SPEC>;
#[doc = "On-Chip Debug Register"]
pub mod ocdr;
