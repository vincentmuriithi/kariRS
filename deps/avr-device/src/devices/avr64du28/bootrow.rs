#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Boot row"]
    pub bootrow: BOOTROW,
}
#[doc = "BOOTROW (rw) register accessor: an alias for `Reg<BOOTROW_SPEC>`"]
pub type BOOTROW = crate::Reg<bootrow::BOOTROW_SPEC>;
#[doc = "Boot row"]
pub mod bootrow;
