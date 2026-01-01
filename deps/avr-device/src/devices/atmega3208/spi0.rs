#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x02 - Interrupt Control"]
    pub intctrl: INTCTRL,
    #[doc = "0x03 - Interrupt Flags"]
    pub intflags: INTFLAGS,
    #[doc = "0x04 - Data"]
    pub data: DATA,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data"]
pub mod data;
#[doc = "INTCTRL (rw) register accessor: an alias for `Reg<INTCTRL_SPEC>`"]
pub type INTCTRL = crate::Reg<intctrl::INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod intctrl;
#[doc = "INTFLAGS (rw) register accessor: an alias for `Reg<INTFLAGS_SPEC>`"]
pub type INTFLAGS = crate::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
