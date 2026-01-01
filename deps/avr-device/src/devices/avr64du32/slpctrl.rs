#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control B"]
    pub vregctrl: VREGCTRL,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "VREGCTRL (rw) register accessor: an alias for `Reg<VREGCTRL_SPEC>`"]
pub type VREGCTRL = crate::Reg<vregctrl::VREGCTRL_SPEC>;
#[doc = "Control B"]
pub mod vregctrl;
