#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Status"]
    pub status: STATUS,
    #[doc = "0x02 - Interrupt Level 0 Priority"]
    pub lvl0pri: LVL0PRI,
    #[doc = "0x03 - Interrupt Level 1 Priority Vector"]
    pub lvl1vec: LVL1VEC,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "LVL0PRI (rw) register accessor: an alias for `Reg<LVL0PRI_SPEC>`"]
pub type LVL0PRI = crate::Reg<lvl0pri::LVL0PRI_SPEC>;
#[doc = "Interrupt Level 0 Priority"]
pub mod lvl0pri;
#[doc = "LVL1VEC (rw) register accessor: an alias for `Reg<LVL1VEC_SPEC>`"]
pub type LVL1VEC = crate::Reg<lvl1vec::LVL1VEC_SPEC>;
#[doc = "Interrupt Level 1 Priority Vector"]
pub mod lvl1vec;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
