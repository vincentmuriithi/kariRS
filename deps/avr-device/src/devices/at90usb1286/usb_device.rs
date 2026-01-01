#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description."]
    pub udcon: UDCON,
    #[doc = "0x01 - No Description."]
    pub udint: UDINT,
    #[doc = "0x02 - No Description."]
    pub udien: UDIEN,
    #[doc = "0x03 - No Description."]
    pub udaddr: UDADDR,
    #[doc = "0x04 - No Description."]
    pub udfnum: UDFNUM,
    #[doc = "0x06 - No Description."]
    pub udmfn: UDMFN,
    _reserved6: [u8; 0x01],
    #[doc = "0x08 - No Description."]
    pub ueintx: UEINTX,
    #[doc = "0x09 - No Description."]
    pub uenum: UENUM,
    #[doc = "0x0a - No Description."]
    pub uerst: UERST,
    #[doc = "0x0b - No Description."]
    pub ueconx: UECONX,
    #[doc = "0x0c - No Description."]
    pub uecfg0x: UECFG0X,
    #[doc = "0x0d - No Description."]
    pub uecfg1x: UECFG1X,
    #[doc = "0x0e - No Description."]
    pub uesta0x: UESTA0X,
    #[doc = "0x0f - No Description."]
    pub uesta1x: UESTA1X,
    #[doc = "0x10 - No Description."]
    pub ueienx: UEIENX,
    #[doc = "0x11 - No Description."]
    pub uedatx: UEDATX,
    #[doc = "0x12 - No Description."]
    pub uebclx: UEBCLX,
    #[doc = "0x13 - No Description."]
    pub uebchx: UEBCHX,
    #[doc = "0x14 - No Description."]
    pub ueint: UEINT,
}
#[doc = "UDADDR (rw) register accessor: an alias for `Reg<UDADDR_SPEC>`"]
pub type UDADDR = crate::Reg<udaddr::UDADDR_SPEC>;
#[doc = "No Description."]
pub mod udaddr;
#[doc = "UDCON (rw) register accessor: an alias for `Reg<UDCON_SPEC>`"]
pub type UDCON = crate::Reg<udcon::UDCON_SPEC>;
#[doc = "No Description."]
pub mod udcon;
#[doc = "UDFNUM (rw) register accessor: an alias for `Reg<UDFNUM_SPEC>`"]
pub type UDFNUM = crate::Reg<udfnum::UDFNUM_SPEC>;
#[doc = "No Description."]
pub mod udfnum;
#[doc = "UDIEN (rw) register accessor: an alias for `Reg<UDIEN_SPEC>`"]
pub type UDIEN = crate::Reg<udien::UDIEN_SPEC>;
#[doc = "No Description."]
pub mod udien;
#[doc = "UDINT (rw) register accessor: an alias for `Reg<UDINT_SPEC>`"]
pub type UDINT = crate::Reg<udint::UDINT_SPEC>;
#[doc = "No Description."]
pub mod udint;
#[doc = "UDMFN (rw) register accessor: an alias for `Reg<UDMFN_SPEC>`"]
pub type UDMFN = crate::Reg<udmfn::UDMFN_SPEC>;
#[doc = "No Description."]
pub mod udmfn;
#[doc = "UEBCHX (rw) register accessor: an alias for `Reg<UEBCHX_SPEC>`"]
pub type UEBCHX = crate::Reg<uebchx::UEBCHX_SPEC>;
#[doc = "No Description."]
pub mod uebchx;
#[doc = "UEBCLX (rw) register accessor: an alias for `Reg<UEBCLX_SPEC>`"]
pub type UEBCLX = crate::Reg<uebclx::UEBCLX_SPEC>;
#[doc = "No Description."]
pub mod uebclx;
#[doc = "UECFG0X (rw) register accessor: an alias for `Reg<UECFG0X_SPEC>`"]
pub type UECFG0X = crate::Reg<uecfg0x::UECFG0X_SPEC>;
#[doc = "No Description."]
pub mod uecfg0x;
#[doc = "UECFG1X (rw) register accessor: an alias for `Reg<UECFG1X_SPEC>`"]
pub type UECFG1X = crate::Reg<uecfg1x::UECFG1X_SPEC>;
#[doc = "No Description."]
pub mod uecfg1x;
#[doc = "UECONX (rw) register accessor: an alias for `Reg<UECONX_SPEC>`"]
pub type UECONX = crate::Reg<ueconx::UECONX_SPEC>;
#[doc = "No Description."]
pub mod ueconx;
#[doc = "UEDATX (rw) register accessor: an alias for `Reg<UEDATX_SPEC>`"]
pub type UEDATX = crate::Reg<uedatx::UEDATX_SPEC>;
#[doc = "No Description."]
pub mod uedatx;
#[doc = "UEIENX (rw) register accessor: an alias for `Reg<UEIENX_SPEC>`"]
pub type UEIENX = crate::Reg<ueienx::UEIENX_SPEC>;
#[doc = "No Description."]
pub mod ueienx;
#[doc = "UEINT (rw) register accessor: an alias for `Reg<UEINT_SPEC>`"]
pub type UEINT = crate::Reg<ueint::UEINT_SPEC>;
#[doc = "No Description."]
pub mod ueint;
#[doc = "UEINTX (rw) register accessor: an alias for `Reg<UEINTX_SPEC>`"]
pub type UEINTX = crate::Reg<ueintx::UEINTX_SPEC>;
#[doc = "No Description."]
pub mod ueintx;
#[doc = "UENUM (rw) register accessor: an alias for `Reg<UENUM_SPEC>`"]
pub type UENUM = crate::Reg<uenum::UENUM_SPEC>;
#[doc = "No Description."]
pub mod uenum;
#[doc = "UERST (rw) register accessor: an alias for `Reg<UERST_SPEC>`"]
pub type UERST = crate::Reg<uerst::UERST_SPEC>;
#[doc = "No Description."]
pub mod uerst;
#[doc = "UESTA0X (rw) register accessor: an alias for `Reg<UESTA0X_SPEC>`"]
pub type UESTA0X = crate::Reg<uesta0x::UESTA0X_SPEC>;
#[doc = "No Description."]
pub mod uesta0x;
#[doc = "UESTA1X (rw) register accessor: an alias for `Reg<UESTA1X_SPEC>`"]
pub type UESTA1X = crate::Reg<uesta1x::UESTA1X_SPEC>;
#[doc = "No Description."]
pub mod uesta1x;
