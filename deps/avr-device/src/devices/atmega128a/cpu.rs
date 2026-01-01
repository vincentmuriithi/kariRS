#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Control And Status Register"]
    pub mcucsr: MCUCSR,
    #[doc = "0x01 - MCU Control Register"]
    pub mcucr: MCUCR,
    _reserved2: [u8; 0x05],
    #[doc = "0x07 - RAM Page Z Select Register"]
    pub rampz: RAMPZ,
    #[doc = "0x08 - XTAL Divide Control Register"]
    pub xdiv: XDIV,
    _reserved4: [u8; 0x0f],
    #[doc = "0x18 - External Memory Control Register B"]
    pub xmcrb: XMCRB,
    #[doc = "0x19 - External Memory Control Register A"]
    pub xmcra: XMCRA,
    _reserved6: [u8; 0x01],
    #[doc = "0x1b - Oscillator Calibration Value"]
    pub osccal: OSCCAL,
}
#[doc = "MCUCR (rw) register accessor: an alias for `Reg<MCUCR_SPEC>`"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "MCUCSR (rw) register accessor: an alias for `Reg<MCUCSR_SPEC>`"]
pub type MCUCSR = crate::Reg<mcucsr::MCUCSR_SPEC>;
#[doc = "MCU Control And Status Register"]
pub mod mcucsr;
#[doc = "OSCCAL (rw) register accessor: an alias for `Reg<OSCCAL_SPEC>`"]
pub type OSCCAL = crate::Reg<osccal::OSCCAL_SPEC>;
#[doc = "Oscillator Calibration Value"]
pub mod osccal;
#[doc = "RAMPZ (rw) register accessor: an alias for `Reg<RAMPZ_SPEC>`"]
pub type RAMPZ = crate::Reg<rampz::RAMPZ_SPEC>;
#[doc = "RAM Page Z Select Register"]
pub mod rampz;
#[doc = "XDIV (rw) register accessor: an alias for `Reg<XDIV_SPEC>`"]
pub type XDIV = crate::Reg<xdiv::XDIV_SPEC>;
#[doc = "XTAL Divide Control Register"]
pub mod xdiv;
#[doc = "XMCRA (rw) register accessor: an alias for `Reg<XMCRA_SPEC>`"]
pub type XMCRA = crate::Reg<xmcra::XMCRA_SPEC>;
#[doc = "External Memory Control Register A"]
pub mod xmcra;
#[doc = "XMCRB (rw) register accessor: an alias for `Reg<XMCRB_SPEC>`"]
pub type XMCRB = crate::Reg<xmcrb::XMCRB_SPEC>;
#[doc = "External Memory Control Register B"]
pub mod xmcrb;
