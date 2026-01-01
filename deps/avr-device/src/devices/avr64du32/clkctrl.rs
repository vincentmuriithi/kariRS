#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCLK Control A"]
    pub mclkctrla: MCLKCTRLA,
    #[doc = "0x01 - MCLK Control B"]
    pub mclkctrlb: MCLKCTRLB,
    #[doc = "0x02 - MCLK Control C"]
    pub mclkctrlc: MCLKCTRLC,
    #[doc = "0x03 - MCLK Interrupt Control"]
    pub mclkintctrl: MCLKINTCTRL,
    #[doc = "0x04 - MCLK Interrupt Flags"]
    pub mclkintflags: MCLKINTFLAGS,
    #[doc = "0x05 - MCLK Status"]
    pub mclkstatus: MCLKSTATUS,
    #[doc = "0x06 - Timebase"]
    pub mclktimebase: MCLKTIMEBASE,
    _reserved7: [u8; 0x01],
    #[doc = "0x08 - OSCHF Control A"]
    pub oschfctrla: OSCHFCTRLA,
    #[doc = "0x09 - OSCHF Tune"]
    pub oschftune: OSCHFTUNE,
    #[doc = "0x0a - OSCHF Status"]
    pub oschfstatus: OSCHFSTATUS,
    _reserved10: [u8; 0x0d],
    #[doc = "0x18 - OSC32K Control A"]
    pub osc32kctrla: OSC32KCTRLA,
    _reserved11: [u8; 0x03],
    #[doc = "0x1c - XOSC32K Control A"]
    pub xosc32kctrla: XOSC32KCTRLA,
    _reserved12: [u8; 0x03],
    #[doc = "0x20 - XOSCHF Control A"]
    pub xoschfctrla: XOSCHFCTRLA,
    _reserved13: [u8; 0x04],
    #[doc = "0x25 - PLL Status"]
    pub usbpllstatus: USBPLLSTATUS,
}
#[doc = "MCLKCTRLA (rw) register accessor: an alias for `Reg<MCLKCTRLA_SPEC>`"]
pub type MCLKCTRLA = crate::Reg<mclkctrla::MCLKCTRLA_SPEC>;
#[doc = "MCLK Control A"]
pub mod mclkctrla;
#[doc = "MCLKCTRLB (rw) register accessor: an alias for `Reg<MCLKCTRLB_SPEC>`"]
pub type MCLKCTRLB = crate::Reg<mclkctrlb::MCLKCTRLB_SPEC>;
#[doc = "MCLK Control B"]
pub mod mclkctrlb;
#[doc = "MCLKCTRLC (rw) register accessor: an alias for `Reg<MCLKCTRLC_SPEC>`"]
pub type MCLKCTRLC = crate::Reg<mclkctrlc::MCLKCTRLC_SPEC>;
#[doc = "MCLK Control C"]
pub mod mclkctrlc;
#[doc = "MCLKINTCTRL (rw) register accessor: an alias for `Reg<MCLKINTCTRL_SPEC>`"]
pub type MCLKINTCTRL = crate::Reg<mclkintctrl::MCLKINTCTRL_SPEC>;
#[doc = "MCLK Interrupt Control"]
pub mod mclkintctrl;
#[doc = "MCLKINTFLAGS (rw) register accessor: an alias for `Reg<MCLKINTFLAGS_SPEC>`"]
pub type MCLKINTFLAGS = crate::Reg<mclkintflags::MCLKINTFLAGS_SPEC>;
#[doc = "MCLK Interrupt Flags"]
pub mod mclkintflags;
#[doc = "MCLKSTATUS (r) register accessor: an alias for `Reg<MCLKSTATUS_SPEC>`"]
pub type MCLKSTATUS = crate::Reg<mclkstatus::MCLKSTATUS_SPEC>;
#[doc = "MCLK Status"]
pub mod mclkstatus;
#[doc = "MCLKTIMEBASE (rw) register accessor: an alias for `Reg<MCLKTIMEBASE_SPEC>`"]
pub type MCLKTIMEBASE = crate::Reg<mclktimebase::MCLKTIMEBASE_SPEC>;
#[doc = "Timebase"]
pub mod mclktimebase;
#[doc = "OSC32KCTRLA (rw) register accessor: an alias for `Reg<OSC32KCTRLA_SPEC>`"]
pub type OSC32KCTRLA = crate::Reg<osc32kctrla::OSC32KCTRLA_SPEC>;
#[doc = "OSC32K Control A"]
pub mod osc32kctrla;
#[doc = "OSCHFCTRLA (rw) register accessor: an alias for `Reg<OSCHFCTRLA_SPEC>`"]
pub type OSCHFCTRLA = crate::Reg<oschfctrla::OSCHFCTRLA_SPEC>;
#[doc = "OSCHF Control A"]
pub mod oschfctrla;
#[doc = "OSCHFSTATUS (r) register accessor: an alias for `Reg<OSCHFSTATUS_SPEC>`"]
pub type OSCHFSTATUS = crate::Reg<oschfstatus::OSCHFSTATUS_SPEC>;
#[doc = "OSCHF Status"]
pub mod oschfstatus;
#[doc = "OSCHFTUNE (rw) register accessor: an alias for `Reg<OSCHFTUNE_SPEC>`"]
pub type OSCHFTUNE = crate::Reg<oschftune::OSCHFTUNE_SPEC>;
#[doc = "OSCHF Tune"]
pub mod oschftune;
#[doc = "USBPLLSTATUS (r) register accessor: an alias for `Reg<USBPLLSTATUS_SPEC>`"]
pub type USBPLLSTATUS = crate::Reg<usbpllstatus::USBPLLSTATUS_SPEC>;
#[doc = "PLL Status"]
pub mod usbpllstatus;
#[doc = "XOSC32KCTRLA (rw) register accessor: an alias for `Reg<XOSC32KCTRLA_SPEC>`"]
pub type XOSC32KCTRLA = crate::Reg<xosc32kctrla::XOSC32KCTRLA_SPEC>;
#[doc = "XOSC32K Control A"]
pub mod xosc32kctrla;
#[doc = "XOSCHFCTRLA (rw) register accessor: an alias for `Reg<XOSCHFCTRLA_SPEC>`"]
pub type XOSCHFCTRLA = crate::Reg<xoschfctrla::XOSCHFCTRLA_SPEC>;
#[doc = "XOSCHF Control A"]
pub mod xoschfctrla;
