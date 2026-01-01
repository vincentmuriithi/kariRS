#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description."]
    pub lockbit: LOCKBIT,
}
#[doc = "LOCKBIT (rw) register accessor: an alias for `Reg<LOCKBIT_SPEC>`"]
pub type LOCKBIT = crate::Reg<lockbit::LOCKBIT_SPEC>;
#[doc = "No Description."]
pub mod lockbit;
