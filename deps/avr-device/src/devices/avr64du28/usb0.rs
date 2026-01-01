#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x02 - Bus State"]
    pub busstate: BUSSTATE,
    #[doc = "0x03 - Address"]
    pub addr: ADDR,
    #[doc = "0x04 - FIFO Write Pointer"]
    pub fifowp: FIFOWP,
    #[doc = "0x05 - FIFO Read Pointer"]
    pub fiforp: FIFORP,
    #[doc = "0x06 - Endpoint Configuration Table Pointer"]
    pub epptr: EPPTR,
    #[doc = "0x08 - Interrupt Control A"]
    pub intctrla: INTCTRLA,
    #[doc = "0x09 - Interrupt Control B"]
    pub intctrlb: INTCTRLB,
    #[doc = "0x0a - Interrupt Flags A"]
    pub intflagsa: INTFLAGSA,
    #[doc = "0x0b - Interrupt Flags B"]
    pub intflagsb: INTFLAGSB,
}
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address"]
pub mod addr;
#[doc = "BUSSTATE (r) register accessor: an alias for `Reg<BUSSTATE_SPEC>`"]
pub type BUSSTATE = crate::Reg<busstate::BUSSTATE_SPEC>;
#[doc = "Bus State"]
pub mod busstate;
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "EPPTR (rw) register accessor: an alias for `Reg<EPPTR_SPEC>`"]
pub type EPPTR = crate::Reg<epptr::EPPTR_SPEC>;
#[doc = "Endpoint Configuration Table Pointer"]
pub mod epptr;
#[doc = "FIFORP (rw) register accessor: an alias for `Reg<FIFORP_SPEC>`"]
pub type FIFORP = crate::Reg<fiforp::FIFORP_SPEC>;
#[doc = "FIFO Read Pointer"]
pub mod fiforp;
#[doc = "FIFOWP (rw) register accessor: an alias for `Reg<FIFOWP_SPEC>`"]
pub type FIFOWP = crate::Reg<fifowp::FIFOWP_SPEC>;
#[doc = "FIFO Write Pointer"]
pub mod fifowp;
#[doc = "INTCTRLA (rw) register accessor: an alias for `Reg<INTCTRLA_SPEC>`"]
pub type INTCTRLA = crate::Reg<intctrla::INTCTRLA_SPEC>;
#[doc = "Interrupt Control A"]
pub mod intctrla;
#[doc = "INTCTRLB (rw) register accessor: an alias for `Reg<INTCTRLB_SPEC>`"]
pub type INTCTRLB = crate::Reg<intctrlb::INTCTRLB_SPEC>;
#[doc = "Interrupt Control B"]
pub mod intctrlb;
#[doc = "INTFLAGSA (rw) register accessor: an alias for `Reg<INTFLAGSA_SPEC>`"]
pub type INTFLAGSA = crate::Reg<intflagsa::INTFLAGSA_SPEC>;
#[doc = "Interrupt Flags A"]
pub mod intflagsa;
#[doc = "INTFLAGSB (rw) register accessor: an alias for `Reg<INTFLAGSB_SPEC>`"]
pub type INTFLAGSB = crate::Reg<intflagsb::INTFLAGSB_SPEC>;
#[doc = "Interrupt Flags B"]
pub mod intflagsb;
