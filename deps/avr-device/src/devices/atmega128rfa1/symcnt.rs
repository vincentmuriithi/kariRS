#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Symbol Counter Control Register 0"]
    pub sccr0: SCCR0,
    #[doc = "0x01 - Symbol Counter Control Register 1"]
    pub sccr1: SCCR1,
    #[doc = "0x02 - Symbol Counter Status Register"]
    pub scsr: SCSR,
    #[doc = "0x03 - Symbol Counter Interrupt Mask Register"]
    pub scirqm: SCIRQM,
    #[doc = "0x04 - Symbol Counter Interrupt Status Register"]
    pub scirqs: SCIRQS,
    #[doc = "0x05 - Symbol Counter Register LL-Byte"]
    pub sccntll: SCCNTLL,
    #[doc = "0x06 - Symbol Counter Register LH-Byte"]
    pub sccntlh: SCCNTLH,
    #[doc = "0x07 - Symbol Counter Register HL-Byte"]
    pub sccnthl: SCCNTHL,
    #[doc = "0x08 - Symbol Counter Register HH-Byte"]
    pub sccnthh: SCCNTHH,
    #[doc = "0x09 - Symbol Counter Beacon Timestamp Register LL-Byte"]
    pub scbtsrll: SCBTSRLL,
    #[doc = "0x0a - Symbol Counter Beacon Timestamp Register LH-Byte"]
    pub scbtsrlh: SCBTSRLH,
    #[doc = "0x0b - Symbol Counter Beacon Timestamp Register HL-Byte"]
    pub scbtsrhl: SCBTSRHL,
    #[doc = "0x0c - Symbol Counter Beacon Timestamp Register HH-Byte"]
    pub scbtsrhh: SCBTSRHH,
    #[doc = "0x0d - Symbol Counter Frame Timestamp Register LL-Byte"]
    pub sctsrll: SCTSRLL,
    #[doc = "0x0e - Symbol Counter Frame Timestamp Register LH-Byte"]
    pub sctsrlh: SCTSRLH,
    #[doc = "0x0f - Symbol Counter Frame Timestamp Register HL-Byte"]
    pub sctsrhl: SCTSRHL,
    #[doc = "0x10 - Symbol Counter Frame Timestamp Register HH-Byte"]
    pub sctsrhh: SCTSRHH,
    #[doc = "0x11 - Symbol Counter Output Compare Register 3 LL-Byte"]
    pub scocr3ll: SCOCR3LL,
    #[doc = "0x12 - Symbol Counter Output Compare Register 3 LH-Byte"]
    pub scocr3lh: SCOCR3LH,
    #[doc = "0x13 - Symbol Counter Output Compare Register 3 HL-Byte"]
    pub scocr3hl: SCOCR3HL,
    #[doc = "0x14 - Symbol Counter Output Compare Register 3 HH-Byte"]
    pub scocr3hh: SCOCR3HH,
    #[doc = "0x15 - Symbol Counter Output Compare Register 2 LL-Byte"]
    pub scocr2ll: SCOCR2LL,
    #[doc = "0x16 - Symbol Counter Output Compare Register 2 LH-Byte"]
    pub scocr2lh: SCOCR2LH,
    #[doc = "0x17 - Symbol Counter Output Compare Register 2 HL-Byte"]
    pub scocr2hl: SCOCR2HL,
    #[doc = "0x18 - Symbol Counter Output Compare Register 2 HH-Byte"]
    pub scocr2hh: SCOCR2HH,
    #[doc = "0x19 - Symbol Counter Output Compare Register 1 LL-Byte"]
    pub scocr1ll: SCOCR1LL,
    #[doc = "0x1a - Symbol Counter Output Compare Register 1 LH-Byte"]
    pub scocr1lh: SCOCR1LH,
    #[doc = "0x1b - Symbol Counter Output Compare Register 1 HL-Byte"]
    pub scocr1hl: SCOCR1HL,
    #[doc = "0x1c - Symbol Counter Output Compare Register 1 HH-Byte"]
    pub scocr1hh: SCOCR1HH,
}
#[doc = "SCBTSRHH (rw) register accessor: an alias for `Reg<SCBTSRHH_SPEC>`"]
pub type SCBTSRHH = crate::Reg<scbtsrhh::SCBTSRHH_SPEC>;
#[doc = "Symbol Counter Beacon Timestamp Register HH-Byte"]
pub mod scbtsrhh;
#[doc = "SCBTSRHL (rw) register accessor: an alias for `Reg<SCBTSRHL_SPEC>`"]
pub type SCBTSRHL = crate::Reg<scbtsrhl::SCBTSRHL_SPEC>;
#[doc = "Symbol Counter Beacon Timestamp Register HL-Byte"]
pub mod scbtsrhl;
#[doc = "SCBTSRLH (rw) register accessor: an alias for `Reg<SCBTSRLH_SPEC>`"]
pub type SCBTSRLH = crate::Reg<scbtsrlh::SCBTSRLH_SPEC>;
#[doc = "Symbol Counter Beacon Timestamp Register LH-Byte"]
pub mod scbtsrlh;
#[doc = "SCBTSRLL (rw) register accessor: an alias for `Reg<SCBTSRLL_SPEC>`"]
pub type SCBTSRLL = crate::Reg<scbtsrll::SCBTSRLL_SPEC>;
#[doc = "Symbol Counter Beacon Timestamp Register LL-Byte"]
pub mod scbtsrll;
#[doc = "SCCNTHH (rw) register accessor: an alias for `Reg<SCCNTHH_SPEC>`"]
pub type SCCNTHH = crate::Reg<sccnthh::SCCNTHH_SPEC>;
#[doc = "Symbol Counter Register HH-Byte"]
pub mod sccnthh;
#[doc = "SCCNTHL (rw) register accessor: an alias for `Reg<SCCNTHL_SPEC>`"]
pub type SCCNTHL = crate::Reg<sccnthl::SCCNTHL_SPEC>;
#[doc = "Symbol Counter Register HL-Byte"]
pub mod sccnthl;
#[doc = "SCCNTLH (rw) register accessor: an alias for `Reg<SCCNTLH_SPEC>`"]
pub type SCCNTLH = crate::Reg<sccntlh::SCCNTLH_SPEC>;
#[doc = "Symbol Counter Register LH-Byte"]
pub mod sccntlh;
#[doc = "SCCNTLL (rw) register accessor: an alias for `Reg<SCCNTLL_SPEC>`"]
pub type SCCNTLL = crate::Reg<sccntll::SCCNTLL_SPEC>;
#[doc = "Symbol Counter Register LL-Byte"]
pub mod sccntll;
#[doc = "SCCR0 (rw) register accessor: an alias for `Reg<SCCR0_SPEC>`"]
pub type SCCR0 = crate::Reg<sccr0::SCCR0_SPEC>;
#[doc = "Symbol Counter Control Register 0"]
pub mod sccr0;
#[doc = "SCCR1 (rw) register accessor: an alias for `Reg<SCCR1_SPEC>`"]
pub type SCCR1 = crate::Reg<sccr1::SCCR1_SPEC>;
#[doc = "Symbol Counter Control Register 1"]
pub mod sccr1;
#[doc = "SCIRQM (rw) register accessor: an alias for `Reg<SCIRQM_SPEC>`"]
pub type SCIRQM = crate::Reg<scirqm::SCIRQM_SPEC>;
#[doc = "Symbol Counter Interrupt Mask Register"]
pub mod scirqm;
#[doc = "SCIRQS (rw) register accessor: an alias for `Reg<SCIRQS_SPEC>`"]
pub type SCIRQS = crate::Reg<scirqs::SCIRQS_SPEC>;
#[doc = "Symbol Counter Interrupt Status Register"]
pub mod scirqs;
#[doc = "SCOCR1HH (rw) register accessor: an alias for `Reg<SCOCR1HH_SPEC>`"]
pub type SCOCR1HH = crate::Reg<scocr1hh::SCOCR1HH_SPEC>;
#[doc = "Symbol Counter Output Compare Register 1 HH-Byte"]
pub mod scocr1hh;
#[doc = "SCOCR1HL (rw) register accessor: an alias for `Reg<SCOCR1HL_SPEC>`"]
pub type SCOCR1HL = crate::Reg<scocr1hl::SCOCR1HL_SPEC>;
#[doc = "Symbol Counter Output Compare Register 1 HL-Byte"]
pub mod scocr1hl;
#[doc = "SCOCR1LH (rw) register accessor: an alias for `Reg<SCOCR1LH_SPEC>`"]
pub type SCOCR1LH = crate::Reg<scocr1lh::SCOCR1LH_SPEC>;
#[doc = "Symbol Counter Output Compare Register 1 LH-Byte"]
pub mod scocr1lh;
#[doc = "SCOCR1LL (rw) register accessor: an alias for `Reg<SCOCR1LL_SPEC>`"]
pub type SCOCR1LL = crate::Reg<scocr1ll::SCOCR1LL_SPEC>;
#[doc = "Symbol Counter Output Compare Register 1 LL-Byte"]
pub mod scocr1ll;
#[doc = "SCOCR2HH (rw) register accessor: an alias for `Reg<SCOCR2HH_SPEC>`"]
pub type SCOCR2HH = crate::Reg<scocr2hh::SCOCR2HH_SPEC>;
#[doc = "Symbol Counter Output Compare Register 2 HH-Byte"]
pub mod scocr2hh;
#[doc = "SCOCR2HL (rw) register accessor: an alias for `Reg<SCOCR2HL_SPEC>`"]
pub type SCOCR2HL = crate::Reg<scocr2hl::SCOCR2HL_SPEC>;
#[doc = "Symbol Counter Output Compare Register 2 HL-Byte"]
pub mod scocr2hl;
#[doc = "SCOCR2LH (rw) register accessor: an alias for `Reg<SCOCR2LH_SPEC>`"]
pub type SCOCR2LH = crate::Reg<scocr2lh::SCOCR2LH_SPEC>;
#[doc = "Symbol Counter Output Compare Register 2 LH-Byte"]
pub mod scocr2lh;
#[doc = "SCOCR2LL (rw) register accessor: an alias for `Reg<SCOCR2LL_SPEC>`"]
pub type SCOCR2LL = crate::Reg<scocr2ll::SCOCR2LL_SPEC>;
#[doc = "Symbol Counter Output Compare Register 2 LL-Byte"]
pub mod scocr2ll;
#[doc = "SCOCR3HH (rw) register accessor: an alias for `Reg<SCOCR3HH_SPEC>`"]
pub type SCOCR3HH = crate::Reg<scocr3hh::SCOCR3HH_SPEC>;
#[doc = "Symbol Counter Output Compare Register 3 HH-Byte"]
pub mod scocr3hh;
#[doc = "SCOCR3HL (rw) register accessor: an alias for `Reg<SCOCR3HL_SPEC>`"]
pub type SCOCR3HL = crate::Reg<scocr3hl::SCOCR3HL_SPEC>;
#[doc = "Symbol Counter Output Compare Register 3 HL-Byte"]
pub mod scocr3hl;
#[doc = "SCOCR3LH (rw) register accessor: an alias for `Reg<SCOCR3LH_SPEC>`"]
pub type SCOCR3LH = crate::Reg<scocr3lh::SCOCR3LH_SPEC>;
#[doc = "Symbol Counter Output Compare Register 3 LH-Byte"]
pub mod scocr3lh;
#[doc = "SCOCR3LL (rw) register accessor: an alias for `Reg<SCOCR3LL_SPEC>`"]
pub type SCOCR3LL = crate::Reg<scocr3ll::SCOCR3LL_SPEC>;
#[doc = "Symbol Counter Output Compare Register 3 LL-Byte"]
pub mod scocr3ll;
#[doc = "SCSR (rw) register accessor: an alias for `Reg<SCSR_SPEC>`"]
pub type SCSR = crate::Reg<scsr::SCSR_SPEC>;
#[doc = "Symbol Counter Status Register"]
pub mod scsr;
#[doc = "SCTSRHH (rw) register accessor: an alias for `Reg<SCTSRHH_SPEC>`"]
pub type SCTSRHH = crate::Reg<sctsrhh::SCTSRHH_SPEC>;
#[doc = "Symbol Counter Frame Timestamp Register HH-Byte"]
pub mod sctsrhh;
#[doc = "SCTSRHL (rw) register accessor: an alias for `Reg<SCTSRHL_SPEC>`"]
pub type SCTSRHL = crate::Reg<sctsrhl::SCTSRHL_SPEC>;
#[doc = "Symbol Counter Frame Timestamp Register HL-Byte"]
pub mod sctsrhl;
#[doc = "SCTSRLH (rw) register accessor: an alias for `Reg<SCTSRLH_SPEC>`"]
pub type SCTSRLH = crate::Reg<sctsrlh::SCTSRLH_SPEC>;
#[doc = "Symbol Counter Frame Timestamp Register LH-Byte"]
pub mod sctsrlh;
#[doc = "SCTSRLL (rw) register accessor: an alias for `Reg<SCTSRLL_SPEC>`"]
pub type SCTSRLL = crate::Reg<sctsrll::SCTSRLL_SPEC>;
#[doc = "Symbol Counter Frame Timestamp Register LL-Byte"]
pub mod sctsrll;
