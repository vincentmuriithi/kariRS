#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Dual Control"]
    pub dualctrl: DUALCTRL,
    #[doc = "0x02 - Debug Control Register"]
    pub dbgctrl: DBGCTRL,
    #[doc = "0x03 - Master Control A"]
    pub mctrla: MCTRLA,
    #[doc = "0x04 - Master Control B"]
    pub mctrlb: MCTRLB,
    #[doc = "0x05 - Master Status"]
    pub mstatus: MSTATUS,
    #[doc = "0x06 - Master Baurd Rate Control"]
    pub mbaud: MBAUD,
    #[doc = "0x07 - Master Address"]
    pub maddr: MADDR,
    #[doc = "0x08 - Master Data"]
    pub mdata: MDATA,
    #[doc = "0x09 - Slave Control A"]
    pub sctrla: SCTRLA,
    #[doc = "0x0a - Slave Control B"]
    pub sctrlb: SCTRLB,
    #[doc = "0x0b - Slave Status"]
    pub sstatus: SSTATUS,
    #[doc = "0x0c - Slave Address"]
    pub saddr: SADDR,
    #[doc = "0x0d - Slave Data"]
    pub sdata: SDATA,
    #[doc = "0x0e - Slave Address Mask"]
    pub saddrmask: SADDRMASK,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control Register"]
pub mod dbgctrl;
#[doc = "DUALCTRL (rw) register accessor: an alias for `Reg<DUALCTRL_SPEC>`"]
pub type DUALCTRL = crate::Reg<dualctrl::DUALCTRL_SPEC>;
#[doc = "Dual Control"]
pub mod dualctrl;
#[doc = "MADDR (rw) register accessor: an alias for `Reg<MADDR_SPEC>`"]
pub type MADDR = crate::Reg<maddr::MADDR_SPEC>;
#[doc = "Master Address"]
pub mod maddr;
#[doc = "MBAUD (rw) register accessor: an alias for `Reg<MBAUD_SPEC>`"]
pub type MBAUD = crate::Reg<mbaud::MBAUD_SPEC>;
#[doc = "Master Baurd Rate Control"]
pub mod mbaud;
#[doc = "MCTRLA (rw) register accessor: an alias for `Reg<MCTRLA_SPEC>`"]
pub type MCTRLA = crate::Reg<mctrla::MCTRLA_SPEC>;
#[doc = "Master Control A"]
pub mod mctrla;
#[doc = "MCTRLB (rw) register accessor: an alias for `Reg<MCTRLB_SPEC>`"]
pub type MCTRLB = crate::Reg<mctrlb::MCTRLB_SPEC>;
#[doc = "Master Control B"]
pub mod mctrlb;
#[doc = "MDATA (rw) register accessor: an alias for `Reg<MDATA_SPEC>`"]
pub type MDATA = crate::Reg<mdata::MDATA_SPEC>;
#[doc = "Master Data"]
pub mod mdata;
#[doc = "MSTATUS (rw) register accessor: an alias for `Reg<MSTATUS_SPEC>`"]
pub type MSTATUS = crate::Reg<mstatus::MSTATUS_SPEC>;
#[doc = "Master Status"]
pub mod mstatus;
#[doc = "SADDR (rw) register accessor: an alias for `Reg<SADDR_SPEC>`"]
pub type SADDR = crate::Reg<saddr::SADDR_SPEC>;
#[doc = "Slave Address"]
pub mod saddr;
#[doc = "SADDRMASK (rw) register accessor: an alias for `Reg<SADDRMASK_SPEC>`"]
pub type SADDRMASK = crate::Reg<saddrmask::SADDRMASK_SPEC>;
#[doc = "Slave Address Mask"]
pub mod saddrmask;
#[doc = "SCTRLA (rw) register accessor: an alias for `Reg<SCTRLA_SPEC>`"]
pub type SCTRLA = crate::Reg<sctrla::SCTRLA_SPEC>;
#[doc = "Slave Control A"]
pub mod sctrla;
#[doc = "SCTRLB (rw) register accessor: an alias for `Reg<SCTRLB_SPEC>`"]
pub type SCTRLB = crate::Reg<sctrlb::SCTRLB_SPEC>;
#[doc = "Slave Control B"]
pub mod sctrlb;
#[doc = "SDATA (rw) register accessor: an alias for `Reg<SDATA_SPEC>`"]
pub type SDATA = crate::Reg<sdata::SDATA_SPEC>;
#[doc = "Slave Data"]
pub mod sdata;
#[doc = "SSTATUS (rw) register accessor: an alias for `Reg<SSTATUS_SPEC>`"]
pub type SSTATUS = crate::Reg<sstatus::SSTATUS_SPEC>;
#[doc = "Slave Status"]
pub mod sstatus;
