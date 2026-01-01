#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Revision ID"]
    pub revid: REVID,
    #[doc = "0x01 - External Break"]
    pub extbrk: EXTBRK,
    _reserved2: [u8; 0x15],
    #[doc = "0x17 - OCD Message Register"]
    pub ocdm: OCDM,
    #[doc = "0x18 - OCD Message Status"]
    pub ocdms: OCDMS,
}
#[doc = "EXTBRK (rw) register accessor: an alias for `Reg<EXTBRK_SPEC>`"]
pub type EXTBRK = crate::Reg<extbrk::EXTBRK_SPEC>;
#[doc = "External Break"]
pub mod extbrk;
#[doc = "OCDM (rw) register accessor: an alias for `Reg<OCDM_SPEC>`"]
pub type OCDM = crate::Reg<ocdm::OCDM_SPEC>;
#[doc = "OCD Message Register"]
pub mod ocdm;
#[doc = "OCDMS (rw) register accessor: an alias for `Reg<OCDMS_SPEC>`"]
pub type OCDMS = crate::Reg<ocdms::OCDMS_SPEC>;
#[doc = "OCD Message Status"]
pub mod ocdms;
#[doc = "REVID (rw) register accessor: an alias for `Reg<REVID_SPEC>`"]
pub type REVID = crate::Reg<revid::REVID_SPEC>;
#[doc = "Revision ID"]
pub mod revid;
