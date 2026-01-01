#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCLK Control A"]
    pub mclkctrla: MCLKCTRLA,
    #[doc = "0x01 - MCLK Control B"]
    pub mclkctrlb: MCLKCTRLB,
    #[doc = "0x02 - MCLK Lock"]
    pub mclklock: MCLKLOCK,
    #[doc = "0x03 - MCLK Status"]
    pub mclkstatus: MCLKSTATUS,
    _reserved4: [u8; 0x0c],
    #[doc = "0x10 - OSC20M Control A"]
    pub osc20mctrla: OSC20MCTRLA,
    #[doc = "0x11 - OSC20M Calibration A"]
    pub osc20mcaliba: OSC20MCALIBA,
    #[doc = "0x12 - OSC20M Calibration B"]
    pub osc20mcalibb: OSC20MCALIBB,
    _reserved7: [u8; 0x05],
    #[doc = "0x18 - OSC32K Control A"]
    pub osc32kctrla: OSC32KCTRLA,
}
#[doc = "MCLKCTRLA (rw) register accessor: an alias for `Reg<MCLKCTRLA_SPEC>`"]
pub type MCLKCTRLA = crate::Reg<mclkctrla::MCLKCTRLA_SPEC>;
#[doc = "MCLK Control A"]
pub mod mclkctrla;
#[doc = "MCLKCTRLB (rw) register accessor: an alias for `Reg<MCLKCTRLB_SPEC>`"]
pub type MCLKCTRLB = crate::Reg<mclkctrlb::MCLKCTRLB_SPEC>;
#[doc = "MCLK Control B"]
pub mod mclkctrlb;
#[doc = "MCLKLOCK (rw) register accessor: an alias for `Reg<MCLKLOCK_SPEC>`"]
pub type MCLKLOCK = crate::Reg<mclklock::MCLKLOCK_SPEC>;
#[doc = "MCLK Lock"]
pub mod mclklock;
#[doc = "MCLKSTATUS (rw) register accessor: an alias for `Reg<MCLKSTATUS_SPEC>`"]
pub type MCLKSTATUS = crate::Reg<mclkstatus::MCLKSTATUS_SPEC>;
#[doc = "MCLK Status"]
pub mod mclkstatus;
#[doc = "OSC20MCALIBA (rw) register accessor: an alias for `Reg<OSC20MCALIBA_SPEC>`"]
pub type OSC20MCALIBA = crate::Reg<osc20mcaliba::OSC20MCALIBA_SPEC>;
#[doc = "OSC20M Calibration A"]
pub mod osc20mcaliba;
#[doc = "OSC20MCALIBB (rw) register accessor: an alias for `Reg<OSC20MCALIBB_SPEC>`"]
pub type OSC20MCALIBB = crate::Reg<osc20mcalibb::OSC20MCALIBB_SPEC>;
#[doc = "OSC20M Calibration B"]
pub mod osc20mcalibb;
#[doc = "OSC20MCTRLA (rw) register accessor: an alias for `Reg<OSC20MCTRLA_SPEC>`"]
pub type OSC20MCTRLA = crate::Reg<osc20mctrla::OSC20MCTRLA_SPEC>;
#[doc = "OSC20M Control A"]
pub mod osc20mctrla;
#[doc = "OSC32KCTRLA (rw) register accessor: an alias for `Reg<OSC32KCTRLA_SPEC>`"]
pub type OSC32KCTRLA = crate::Reg<osc32kctrla::OSC32KCTRLA_SPEC>;
#[doc = "OSC32K Control A"]
pub mod osc32kctrla;
