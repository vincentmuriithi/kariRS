#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Hardware Configuration Register"]
    pub uhwcon: UHWCON,
    #[doc = "0x01 - USB General Control Register"]
    pub usbcon: USBCON,
    #[doc = "0x02 - No Description."]
    pub usbsta: USBSTA,
    #[doc = "0x03 - No Description."]
    pub usbint: USBINT,
}
#[doc = "UHWCON (rw) register accessor: an alias for `Reg<UHWCON_SPEC>`"]
pub type UHWCON = crate::Reg<uhwcon::UHWCON_SPEC>;
#[doc = "USB Hardware Configuration Register"]
pub mod uhwcon;
#[doc = "USBCON (rw) register accessor: an alias for `Reg<USBCON_SPEC>`"]
pub type USBCON = crate::Reg<usbcon::USBCON_SPEC>;
#[doc = "USB General Control Register"]
pub mod usbcon;
#[doc = "USBINT (rw) register accessor: an alias for `Reg<USBINT_SPEC>`"]
pub type USBINT = crate::Reg<usbint::USBINT_SPEC>;
#[doc = "No Description."]
pub mod usbint;
#[doc = "USBSTA (rw) register accessor: an alias for `Reg<USBSTA_SPEC>`"]
pub type USBSTA = crate::Reg<usbsta::USBSTA_SPEC>;
#[doc = "No Description."]
pub mod usbsta;
