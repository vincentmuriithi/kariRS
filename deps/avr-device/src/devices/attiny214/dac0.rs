#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - DATA Register"]
    pub data: DATA,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control Register A"]
pub mod ctrla;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DATA Register"]
pub mod data;
