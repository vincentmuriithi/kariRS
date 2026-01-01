#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Mux Control A"]
    pub muxctrla: MUXCTRLA,
    _reserved2: [u8; 0x03],
    #[doc = "0x06 - Interrupt Control"]
    pub intctrl: INTCTRL,
    #[doc = "0x07 - Status"]
    pub status: STATUS,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "INTCTRL (rw) register accessor: an alias for `Reg<INTCTRL_SPEC>`"]
pub type INTCTRL = crate::Reg<intctrl::INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod intctrl;
#[doc = "MUXCTRLA (rw) register accessor: an alias for `Reg<MUXCTRLA_SPEC>`"]
pub type MUXCTRLA = crate::Reg<muxctrla::MUXCTRLA_SPEC>;
#[doc = "Mux Control A"]
pub mod muxctrla;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
