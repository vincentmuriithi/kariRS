#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Mux Control A"]
    pub muxctrl: MUXCTRL,
    _reserved2: [u8; 0x02],
    #[doc = "0x05 - DAC Voltage Reference"]
    pub dacref: DACREF,
    #[doc = "0x06 - Interrupt Control"]
    pub intctrl: INTCTRL,
    #[doc = "0x07 - Status"]
    pub status: STATUS,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "DACREF (rw) register accessor: an alias for `Reg<DACREF_SPEC>`"]
pub type DACREF = crate::Reg<dacref::DACREF_SPEC>;
#[doc = "DAC Voltage Reference"]
pub mod dacref;
#[doc = "INTCTRL (rw) register accessor: an alias for `Reg<INTCTRL_SPEC>`"]
pub type INTCTRL = crate::Reg<intctrl::INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod intctrl;
#[doc = "MUXCTRL (rw) register accessor: an alias for `Reg<MUXCTRL_SPEC>`"]
pub type MUXCTRL = crate::Reg<muxctrl::MUXCTRL_SPEC>;
#[doc = "Mux Control A"]
pub mod muxctrl;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
