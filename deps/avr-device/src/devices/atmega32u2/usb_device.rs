#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Regulator Control Register"]
    pub regcr: REGCR,
    _reserved1: [u8; 0x74],
    #[doc = "0x75 - USB General Control Register"]
    pub usbcon: USBCON,
    _reserved2: [u8; 0x07],
    #[doc = "0x7d - USB Device Control Registers"]
    pub udcon: UDCON,
    #[doc = "0x7e - USB Device Interrupt Register"]
    pub udint: UDINT,
    #[doc = "0x7f - USB Device Interrupt Enable Register"]
    pub udien: UDIEN,
    #[doc = "0x80 - USB Device Address Register"]
    pub udaddr: UDADDR,
    #[doc = "0x81 - USB Device Frame Number High Register"]
    pub udfnum: UDFNUM,
    #[doc = "0x83 - USB Device Micro Frame Number"]
    pub udmfn: UDMFN,
    _reserved8: [u8; 0x01],
    #[doc = "0x85 - USB Endpoint Interrupt Register"]
    pub ueintx: UEINTX,
    #[doc = "0x86 - USB Endpoint Number"]
    pub uenum: UENUM,
    #[doc = "0x87 - USB Endpoint Reset Register"]
    pub uerst: UERST,
    #[doc = "0x88 - USB Endpoint Control Register"]
    pub ueconx: UECONX,
    #[doc = "0x89 - USB Endpoint Configuration 0 Register"]
    pub uecfg0x: UECFG0X,
    #[doc = "0x8a - USB Endpoint Configuration 1 Register"]
    pub uecfg1x: UECFG1X,
    #[doc = "0x8b - USB Endpoint Status 0 Register"]
    pub uesta0x: UESTA0X,
    #[doc = "0x8c - USB Endpoint Status 1 Register"]
    pub uesta1x: UESTA1X,
    #[doc = "0x8d - USB Endpoint Interrupt Enable Register"]
    pub ueienx: UEIENX,
    #[doc = "0x8e - USB Data Endpoint"]
    pub uedatx: UEDATX,
    #[doc = "0x8f - USB Endpoint Byte Count Register"]
    pub uebclx: UEBCLX,
    _reserved19: [u8; 0x01],
    #[doc = "0x91 - USB Endpoint Number Interrupt Register"]
    pub ueint: UEINT,
    _reserved20: [u8; 0x06],
    #[doc = "0x98 - USB Software Output Enable register"]
    pub upoe: UPOE,
}
#[doc = "REGCR (rw) register accessor: an alias for `Reg<REGCR_SPEC>`"]
pub type REGCR = crate::Reg<regcr::REGCR_SPEC>;
#[doc = "Regulator Control Register"]
pub mod regcr;
#[doc = "UDADDR (rw) register accessor: an alias for `Reg<UDADDR_SPEC>`"]
pub type UDADDR = crate::Reg<udaddr::UDADDR_SPEC>;
#[doc = "USB Device Address Register"]
pub mod udaddr;
#[doc = "UDCON (rw) register accessor: an alias for `Reg<UDCON_SPEC>`"]
pub type UDCON = crate::Reg<udcon::UDCON_SPEC>;
#[doc = "USB Device Control Registers"]
pub mod udcon;
#[doc = "UDFNUM (rw) register accessor: an alias for `Reg<UDFNUM_SPEC>`"]
pub type UDFNUM = crate::Reg<udfnum::UDFNUM_SPEC>;
#[doc = "USB Device Frame Number High Register"]
pub mod udfnum;
#[doc = "UDIEN (rw) register accessor: an alias for `Reg<UDIEN_SPEC>`"]
pub type UDIEN = crate::Reg<udien::UDIEN_SPEC>;
#[doc = "USB Device Interrupt Enable Register"]
pub mod udien;
#[doc = "UDINT (rw) register accessor: an alias for `Reg<UDINT_SPEC>`"]
pub type UDINT = crate::Reg<udint::UDINT_SPEC>;
#[doc = "USB Device Interrupt Register"]
pub mod udint;
#[doc = "UDMFN (rw) register accessor: an alias for `Reg<UDMFN_SPEC>`"]
pub type UDMFN = crate::Reg<udmfn::UDMFN_SPEC>;
#[doc = "USB Device Micro Frame Number"]
pub mod udmfn;
#[doc = "UEBCLX (rw) register accessor: an alias for `Reg<UEBCLX_SPEC>`"]
pub type UEBCLX = crate::Reg<uebclx::UEBCLX_SPEC>;
#[doc = "USB Endpoint Byte Count Register"]
pub mod uebclx;
#[doc = "UECFG0X (rw) register accessor: an alias for `Reg<UECFG0X_SPEC>`"]
pub type UECFG0X = crate::Reg<uecfg0x::UECFG0X_SPEC>;
#[doc = "USB Endpoint Configuration 0 Register"]
pub mod uecfg0x;
#[doc = "UECFG1X (rw) register accessor: an alias for `Reg<UECFG1X_SPEC>`"]
pub type UECFG1X = crate::Reg<uecfg1x::UECFG1X_SPEC>;
#[doc = "USB Endpoint Configuration 1 Register"]
pub mod uecfg1x;
#[doc = "UECONX (rw) register accessor: an alias for `Reg<UECONX_SPEC>`"]
pub type UECONX = crate::Reg<ueconx::UECONX_SPEC>;
#[doc = "USB Endpoint Control Register"]
pub mod ueconx;
#[doc = "UEDATX (rw) register accessor: an alias for `Reg<UEDATX_SPEC>`"]
pub type UEDATX = crate::Reg<uedatx::UEDATX_SPEC>;
#[doc = "USB Data Endpoint"]
pub mod uedatx;
#[doc = "UEIENX (rw) register accessor: an alias for `Reg<UEIENX_SPEC>`"]
pub type UEIENX = crate::Reg<ueienx::UEIENX_SPEC>;
#[doc = "USB Endpoint Interrupt Enable Register"]
pub mod ueienx;
#[doc = "UEINT (rw) register accessor: an alias for `Reg<UEINT_SPEC>`"]
pub type UEINT = crate::Reg<ueint::UEINT_SPEC>;
#[doc = "USB Endpoint Number Interrupt Register"]
pub mod ueint;
#[doc = "UEINTX (rw) register accessor: an alias for `Reg<UEINTX_SPEC>`"]
pub type UEINTX = crate::Reg<ueintx::UEINTX_SPEC>;
#[doc = "USB Endpoint Interrupt Register"]
pub mod ueintx;
#[doc = "UENUM (rw) register accessor: an alias for `Reg<UENUM_SPEC>`"]
pub type UENUM = crate::Reg<uenum::UENUM_SPEC>;
#[doc = "USB Endpoint Number"]
pub mod uenum;
#[doc = "UERST (rw) register accessor: an alias for `Reg<UERST_SPEC>`"]
pub type UERST = crate::Reg<uerst::UERST_SPEC>;
#[doc = "USB Endpoint Reset Register"]
pub mod uerst;
#[doc = "UESTA0X (rw) register accessor: an alias for `Reg<UESTA0X_SPEC>`"]
pub type UESTA0X = crate::Reg<uesta0x::UESTA0X_SPEC>;
#[doc = "USB Endpoint Status 0 Register"]
pub mod uesta0x;
#[doc = "UESTA1X (rw) register accessor: an alias for `Reg<UESTA1X_SPEC>`"]
pub type UESTA1X = crate::Reg<uesta1x::UESTA1X_SPEC>;
#[doc = "USB Endpoint Status 1 Register"]
pub mod uesta1x;
#[doc = "UPOE (rw) register accessor: an alias for `Reg<UPOE_SPEC>`"]
pub type UPOE = crate::Reg<upoe::UPOE_SPEC>;
#[doc = "USB Software Output Enable register"]
pub mod upoe;
#[doc = "USBCON (rw) register accessor: an alias for `Reg<USBCON_SPEC>`"]
pub type USBCON = crate::Reg<usbcon::USBCON_SPEC>;
#[doc = "USB General Control Register"]
pub mod usbcon;
