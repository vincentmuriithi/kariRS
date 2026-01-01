#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Debug Control Register"]
    pub dbgctrl: DBGCTRL,
    #[doc = "0x03 - Host Control A"]
    pub mctrla: MCTRLA,
    #[doc = "0x04 - Host Control B"]
    pub mctrlb: MCTRLB,
    #[doc = "0x05 - Host Status"]
    pub mstatus: MSTATUS,
    #[doc = "0x06 - Host Baud Rate Control"]
    pub mbaud: MBAUD,
    #[doc = "0x07 - Host Address"]
    pub maddr: MADDR,
    #[doc = "0x08 - Host Data"]
    pub mdata: MDATA,
    #[doc = "0x09 - Client Control A"]
    pub sctrla: SCTRLA,
    #[doc = "0x0a - Client Control B"]
    pub sctrlb: SCTRLB,
    #[doc = "0x0b - Client Status"]
    pub sstatus: SSTATUS,
    #[doc = "0x0c - Client Address"]
    pub saddr: SADDR,
    #[doc = "0x0d - Client Data"]
    pub sdata: SDATA,
    #[doc = "0x0e - Client Address Mask"]
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
#[doc = "MADDR (rw) register accessor: an alias for `Reg<MADDR_SPEC>`"]
pub type MADDR = crate::Reg<maddr::MADDR_SPEC>;
#[doc = "Host Address"]
pub mod maddr;
#[doc = "MBAUD (rw) register accessor: an alias for `Reg<MBAUD_SPEC>`"]
pub type MBAUD = crate::Reg<mbaud::MBAUD_SPEC>;
#[doc = "Host Baud Rate Control"]
pub mod mbaud;
#[doc = "MCTRLA (rw) register accessor: an alias for `Reg<MCTRLA_SPEC>`"]
pub type MCTRLA = crate::Reg<mctrla::MCTRLA_SPEC>;
#[doc = "Host Control A"]
pub mod mctrla;
#[doc = "MCTRLB (rw) register accessor: an alias for `Reg<MCTRLB_SPEC>`"]
pub type MCTRLB = crate::Reg<mctrlb::MCTRLB_SPEC>;
#[doc = "Host Control B"]
pub mod mctrlb;
#[doc = "MDATA (rw) register accessor: an alias for `Reg<MDATA_SPEC>`"]
pub type MDATA = crate::Reg<mdata::MDATA_SPEC>;
#[doc = "Host Data"]
pub mod mdata;
#[doc = "MSTATUS (rw) register accessor: an alias for `Reg<MSTATUS_SPEC>`"]
pub type MSTATUS = crate::Reg<mstatus::MSTATUS_SPEC>;
#[doc = "Host Status"]
pub mod mstatus;
#[doc = "SADDR (rw) register accessor: an alias for `Reg<SADDR_SPEC>`"]
pub type SADDR = crate::Reg<saddr::SADDR_SPEC>;
#[doc = "Client Address"]
pub mod saddr;
#[doc = "SADDRMASK (rw) register accessor: an alias for `Reg<SADDRMASK_SPEC>`"]
pub type SADDRMASK = crate::Reg<saddrmask::SADDRMASK_SPEC>;
#[doc = "Client Address Mask"]
pub mod saddrmask;
#[doc = "SCTRLA (rw) register accessor: an alias for `Reg<SCTRLA_SPEC>`"]
pub type SCTRLA = crate::Reg<sctrla::SCTRLA_SPEC>;
#[doc = "Client Control A"]
pub mod sctrla;
#[doc = "SCTRLB (rw) register accessor: an alias for `Reg<SCTRLB_SPEC>`"]
pub type SCTRLB = crate::Reg<sctrlb::SCTRLB_SPEC>;
#[doc = "Client Control B"]
pub mod sctrlb;
#[doc = "SDATA (rw) register accessor: an alias for `Reg<SDATA_SPEC>`"]
pub type SDATA = crate::Reg<sdata::SDATA_SPEC>;
#[doc = "Client Data"]
pub mod sdata;
#[doc = "SSTATUS (rw) register accessor: an alias for `Reg<SSTATUS_SPEC>`"]
pub type SSTATUS = crate::Reg<sstatus::SSTATUS_SPEC>;
#[doc = "Client Status"]
pub mod sstatus;
