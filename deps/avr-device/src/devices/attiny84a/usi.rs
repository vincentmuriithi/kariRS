#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USI Control Register"]
    pub usicr: USICR,
    #[doc = "0x01 - USI Status Register"]
    pub usisr: USISR,
    #[doc = "0x02 - USI Data Register"]
    pub usidr: USIDR,
    #[doc = "0x03 - USI Buffer Register"]
    pub usibr: USIBR,
}
#[doc = "USIBR (rw) register accessor: an alias for `Reg<USIBR_SPEC>`"]
pub type USIBR = crate::Reg<usibr::USIBR_SPEC>;
#[doc = "USI Buffer Register"]
pub mod usibr;
#[doc = "USICR (rw) register accessor: an alias for `Reg<USICR_SPEC>`"]
pub type USICR = crate::Reg<usicr::USICR_SPEC>;
#[doc = "USI Control Register"]
pub mod usicr;
#[doc = "USIDR (rw) register accessor: an alias for `Reg<USIDR_SPEC>`"]
pub type USIDR = crate::Reg<usidr::USIDR_SPEC>;
#[doc = "USI Data Register"]
pub mod usidr;
#[doc = "USISR (rw) register accessor: an alias for `Reg<USISR_SPEC>`"]
pub type USISR = crate::Reg<usisr::USISR_SPEC>;
#[doc = "USI Status Register"]
pub mod usisr;
