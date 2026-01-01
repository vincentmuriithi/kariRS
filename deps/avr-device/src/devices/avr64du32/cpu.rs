#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Change Protection"]
    pub ccp: CCP,
    _reserved1: [u8; 0x06],
    #[doc = "0x07 - Extended Z-pointer Register"]
    pub rampz: RAMPZ,
}
#[doc = "CCP (rw) register accessor: an alias for `Reg<CCP_SPEC>`"]
pub type CCP = crate::Reg<ccp::CCP_SPEC>;
#[doc = "Configuration Change Protection"]
pub mod ccp;
#[doc = "RAMPZ (rw) register accessor: an alias for `Reg<RAMPZ_SPEC>`"]
pub type RAMPZ = crate::Reg<rampz::RAMPZ_SPEC>;
#[doc = "Extended Z-pointer Register"]
pub mod rampz;
