#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Comparator Control And Status Register-B"]
    pub acsrb: ACSRB,
    _reserved_1_acsr: [u8; 0x01],
    _reserved2: [u8; 0x2e],
    #[doc = "0x30 - Digital Input Disable Register 1"]
    pub didr1: DIDR1,
}
impl RegisterBlock {
    #[doc = "0x01 - Analog Comparator Control And Status Register-A"]
    #[inline(always)]
    pub const fn acsra(&self) -> &ACSRA {
        unsafe { &*(self as *const Self).cast::<u8>().add(1usize).cast() }
    }
    #[doc = "0x01 - Analog Comparator Control And Status Register"]
    #[inline(always)]
    pub const fn acsr(&self) -> &ACSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(1usize).cast() }
    }
}
#[doc = "ACSR (rw) register accessor: an alias for `Reg<ACSR_SPEC>`"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Analog Comparator Control And Status Register"]
pub mod acsr;
#[doc = "ACSRA (rw) register accessor: an alias for `Reg<ACSRA_SPEC>`"]
pub type ACSRA = crate::Reg<acsra::ACSRA_SPEC>;
#[doc = "Analog Comparator Control And Status Register-A"]
pub mod acsra;
#[doc = "ACSRB (rw) register accessor: an alias for `Reg<ACSRB_SPEC>`"]
pub type ACSRB = crate::Reg<acsrb::ACSRB_SPEC>;
#[doc = "Analog Comparator Control And Status Register-B"]
pub mod acsrb;
#[doc = "DIDR1 (rw) register accessor: an alias for `Reg<DIDR1_SPEC>`"]
pub type DIDR1 = crate::Reg<didr1::DIDR1_SPEC>;
#[doc = "Digital Input Disable Register 1"]
pub mod didr1;
