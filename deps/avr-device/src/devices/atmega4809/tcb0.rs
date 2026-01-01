#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control Register B"]
    pub ctrlb: CTRLB,
    _reserved2: [u8; 0x02],
    #[doc = "0x04 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x05 - Interrupt Control"]
    pub intctrl: INTCTRL,
    #[doc = "0x06 - Interrupt Flags"]
    pub intflags: INTFLAGS,
    #[doc = "0x07 - Status"]
    pub status: STATUS,
    #[doc = "0x08 - Debug Control"]
    pub dbgctrl: DBGCTRL,
    #[doc = "0x09 - Temporary Value"]
    pub temp: TEMP,
    #[doc = "0x0a - Count"]
    pub cnt: CNT,
    #[doc = "0x0c - Compare or Capture"]
    pub ccmp: CCMP,
}
#[doc = "CCMP (rw) register accessor: an alias for `Reg<CCMP_SPEC>`"]
pub type CCMP = crate::Reg<ccmp::CCMP_SPEC>;
#[doc = "Compare or Capture"]
pub mod ccmp;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Count"]
pub mod cnt;
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control Register B"]
pub mod ctrlb;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTCTRL (rw) register accessor: an alias for `Reg<INTCTRL_SPEC>`"]
pub type INTCTRL = crate::Reg<intctrl::INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod intctrl;
#[doc = "INTFLAGS (rw) register accessor: an alias for `Reg<INTFLAGS_SPEC>`"]
pub type INTFLAGS = crate::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "TEMP (rw) register accessor: an alias for `Reg<TEMP_SPEC>`"]
pub type TEMP = crate::Reg<temp::TEMP_SPEC>;
#[doc = "Temporary Value"]
pub mod temp;
