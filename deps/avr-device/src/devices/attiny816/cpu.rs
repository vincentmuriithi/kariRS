#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Change Protection"]
    pub ccp: CCP,
    _reserved1: [u8; 0x08],
    #[doc = "0x09 - Stack Pointer Low"]
    pub spl: SPL,
    #[doc = "0x0a - Stack Pointer High"]
    pub sph: SPH,
}
#[doc = "CCP (rw) register accessor: an alias for `Reg<CCP_SPEC>`"]
pub type CCP = crate::Reg<ccp::CCP_SPEC>;
#[doc = "Configuration Change Protection"]
pub mod ccp;
#[doc = "SPH (rw) register accessor: an alias for `Reg<SPH_SPEC>`"]
pub type SPH = crate::Reg<sph::SPH_SPEC>;
#[doc = "Stack Pointer High"]
pub mod sph;
#[doc = "SPL (rw) register accessor: an alias for `Reg<SPL_SPEC>`"]
pub type SPL = crate::Reg<spl::SPL_SPEC>;
#[doc = "Stack Pointer Low"]
pub mod spl;
