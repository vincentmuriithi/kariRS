#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Revision ID"]
    pub revid: REVID,
    _reserved1: [u8; 0x04],
    #[doc = "0x05 - USB Voltage System Control"]
    pub vusbctrl: VUSBCTRL,
}
#[doc = "REVID (rw) register accessor: an alias for `Reg<REVID_SPEC>`"]
pub type REVID = crate::Reg<revid::REVID_SPEC>;
#[doc = "Revision ID"]
pub mod revid;
#[doc = "VUSBCTRL (rw) register accessor: an alias for `Reg<VUSBCTRL_SPEC>`"]
pub type VUSBCTRL = crate::Reg<vusbctrl::VUSBCTRL_SPEC>;
#[doc = "USB Voltage System Control"]
pub mod vusbctrl;
