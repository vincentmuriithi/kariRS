#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - User Row"]
    pub userrow: USERROW,
}
#[doc = "USERROW (rw) register accessor: an alias for `Reg<USERROW_SPEC>`"]
pub type USERROW = crate::Reg<userrow::USERROW_SPEC>;
#[doc = "User Row"]
pub mod userrow;
