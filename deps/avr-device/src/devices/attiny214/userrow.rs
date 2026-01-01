#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - User Row Bytes"]
    pub userrow: [USERROW; 32],
}
#[doc = "USERROW (rw) register accessor: an alias for `Reg<USERROW_SPEC>`"]
pub type USERROW = crate::Reg<userrow::USERROW_SPEC>;
#[doc = "User Row Bytes"]
pub mod userrow;
