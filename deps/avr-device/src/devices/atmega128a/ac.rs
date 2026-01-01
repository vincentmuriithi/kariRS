#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Comparator Control And Status Register"]
    pub acsr: ACSR,
    _reserved1: [u8; 0x17],
    #[doc = "0x18 - Special Function IO Register"]
    pub sfior: SFIOR,
}
#[doc = "ACSR (rw) register accessor: an alias for `Reg<ACSR_SPEC>`"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Analog Comparator Control And Status Register"]
pub mod acsr;
#[doc = "SFIOR (rw) register accessor: an alias for `Reg<SFIOR_SPEC>`"]
pub type SFIOR = crate::Reg<sfior::SFIOR_SPEC>;
#[doc = "Special Function IO Register"]
pub mod sfior;
