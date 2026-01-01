#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LIN Control Register"]
    pub lincr: LINCR,
    #[doc = "0x01 - LIN Status and Interrupt Register"]
    pub linsir: LINSIR,
    #[doc = "0x02 - LIN Enable Interrupt Register"]
    pub linenir: LINENIR,
    #[doc = "0x03 - LIN Error Register"]
    pub linerr: LINERR,
    #[doc = "0x04 - LIN Bit Timing Register"]
    pub linbtr: LINBTR,
    #[doc = "0x05 - LIN Baud Rate Low Register"]
    pub linbrrl: LINBRRL,
    #[doc = "0x06 - LIN Baud Rate High Register"]
    pub linbrrh: LINBRRH,
    #[doc = "0x07 - LIN Data Length Register"]
    pub lindlr: LINDLR,
    #[doc = "0x08 - LIN Identifier Register"]
    pub linidr: LINIDR,
    #[doc = "0x09 - LIN Data Buffer Selection Register"]
    pub linsel: LINSEL,
    #[doc = "0x0a - LIN Data Register"]
    pub lindat: LINDAT,
}
#[doc = "LINBRRH (rw) register accessor: an alias for `Reg<LINBRRH_SPEC>`"]
pub type LINBRRH = crate::Reg<linbrrh::LINBRRH_SPEC>;
#[doc = "LIN Baud Rate High Register"]
pub mod linbrrh;
#[doc = "LINBRRL (rw) register accessor: an alias for `Reg<LINBRRL_SPEC>`"]
pub type LINBRRL = crate::Reg<linbrrl::LINBRRL_SPEC>;
#[doc = "LIN Baud Rate Low Register"]
pub mod linbrrl;
#[doc = "LINBTR (rw) register accessor: an alias for `Reg<LINBTR_SPEC>`"]
pub type LINBTR = crate::Reg<linbtr::LINBTR_SPEC>;
#[doc = "LIN Bit Timing Register"]
pub mod linbtr;
#[doc = "LINCR (rw) register accessor: an alias for `Reg<LINCR_SPEC>`"]
pub type LINCR = crate::Reg<lincr::LINCR_SPEC>;
#[doc = "LIN Control Register"]
pub mod lincr;
#[doc = "LINDAT (rw) register accessor: an alias for `Reg<LINDAT_SPEC>`"]
pub type LINDAT = crate::Reg<lindat::LINDAT_SPEC>;
#[doc = "LIN Data Register"]
pub mod lindat;
#[doc = "LINDLR (rw) register accessor: an alias for `Reg<LINDLR_SPEC>`"]
pub type LINDLR = crate::Reg<lindlr::LINDLR_SPEC>;
#[doc = "LIN Data Length Register"]
pub mod lindlr;
#[doc = "LINENIR (rw) register accessor: an alias for `Reg<LINENIR_SPEC>`"]
pub type LINENIR = crate::Reg<linenir::LINENIR_SPEC>;
#[doc = "LIN Enable Interrupt Register"]
pub mod linenir;
#[doc = "LINERR (rw) register accessor: an alias for `Reg<LINERR_SPEC>`"]
pub type LINERR = crate::Reg<linerr::LINERR_SPEC>;
#[doc = "LIN Error Register"]
pub mod linerr;
#[doc = "LINIDR (rw) register accessor: an alias for `Reg<LINIDR_SPEC>`"]
pub type LINIDR = crate::Reg<linidr::LINIDR_SPEC>;
#[doc = "LIN Identifier Register"]
pub mod linidr;
#[doc = "LINSEL (rw) register accessor: an alias for `Reg<LINSEL_SPEC>`"]
pub type LINSEL = crate::Reg<linsel::LINSEL_SPEC>;
#[doc = "LIN Data Buffer Selection Register"]
pub mod linsel;
#[doc = "LINSIR (rw) register accessor: an alias for `Reg<LINSIR_SPEC>`"]
pub type LINSIR = crate::Reg<linsir::LINSIR_SPEC>;
#[doc = "LIN Status and Interrupt Register"]
pub mod linsir;
