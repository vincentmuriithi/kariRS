#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Comparator 0 Control And Status Register A"]
    pub acsr0a: ACSR0A,
    #[doc = "0x01 - Analog Comparator 0 Control And Status Register B"]
    pub acsr0b: ACSR0B,
    #[doc = "0x02 - Analog Comparator 1 Control And Status Register A"]
    pub acsr1a: ACSR1A,
    #[doc = "0x03 - Analog Comparator 1 Control And Status Register B"]
    pub acsr1b: ACSR1B,
}
#[doc = "ACSR0A (rw) register accessor: an alias for `Reg<ACSR0A_SPEC>`"]
pub type ACSR0A = crate::Reg<acsr0a::ACSR0A_SPEC>;
#[doc = "Analog Comparator 0 Control And Status Register A"]
pub mod acsr0a;
#[doc = "ACSR0B (rw) register accessor: an alias for `Reg<ACSR0B_SPEC>`"]
pub type ACSR0B = crate::Reg<acsr0b::ACSR0B_SPEC>;
#[doc = "Analog Comparator 0 Control And Status Register B"]
pub mod acsr0b;
#[doc = "ACSR1A (rw) register accessor: an alias for `Reg<ACSR1A_SPEC>`"]
pub type ACSR1A = crate::Reg<acsr1a::ACSR1A_SPEC>;
#[doc = "Analog Comparator 1 Control And Status Register A"]
pub mod acsr1a;
#[doc = "ACSR1B (rw) register accessor: an alias for `Reg<ACSR1B_SPEC>`"]
pub type ACSR1B = crate::Reg<acsr1b::ACSR1B_SPEC>;
#[doc = "Analog Comparator 1 Control And Status Register B"]
pub mod acsr1b;
