#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose Register 0"]
    pub gpr0: GPR0,
    #[doc = "0x01 - General Purpose Register 1"]
    pub gpr1: GPR1,
    #[doc = "0x02 - General Purpose Register 2"]
    pub gpr2: GPR2,
    #[doc = "0x03 - General Purpose Register 3"]
    pub gpr3: GPR3,
}
#[doc = "GPR0 (rw) register accessor: an alias for `Reg<GPR0_SPEC>`"]
pub type GPR0 = crate::Reg<gpr0::GPR0_SPEC>;
#[doc = "General Purpose Register 0"]
pub mod gpr0;
#[doc = "GPR1 (rw) register accessor: an alias for `Reg<GPR1_SPEC>`"]
pub type GPR1 = crate::Reg<gpr1::GPR1_SPEC>;
#[doc = "General Purpose Register 1"]
pub mod gpr1;
#[doc = "GPR2 (rw) register accessor: an alias for `Reg<GPR2_SPEC>`"]
pub type GPR2 = crate::Reg<gpr2::GPR2_SPEC>;
#[doc = "General Purpose Register 2"]
pub mod gpr2;
#[doc = "GPR3 (rw) register accessor: an alias for `Reg<GPR3_SPEC>`"]
pub type GPR3 = crate::Reg<gpr3::GPR3_SPEC>;
#[doc = "General Purpose Register 3"]
pub mod gpr3;
