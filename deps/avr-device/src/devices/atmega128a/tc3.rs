#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Special Function IO Register"]
    pub sfior: SFIOR,
    _reserved1: [u8; 0x3b],
    #[doc = "0x3c - Extended Timer/Counter Interrupt Flag register"]
    pub etifr: ETIFR,
    #[doc = "0x3d - Extended Timer/Counter Interrupt Mask Register"]
    pub etimsk: ETIMSK,
    _reserved3: [u8; 0x02],
    #[doc = "0x40 - Timer/Counter3 Input Capture Register Bytes"]
    pub icr3: ICR3,
    #[doc = "0x42 - Timer/Counter3 Output compare Register C Bytes"]
    pub ocr3c: OCR3C,
    #[doc = "0x44 - Timer/Counter3 Output Compare Register B Bytes"]
    pub ocr3b: OCR3B,
    #[doc = "0x46 - Timer/Counter3 Output Compare Register A Bytes"]
    pub ocr3a: OCR3A,
    #[doc = "0x48 - Timer/Counter3 Bytes"]
    pub tcnt3: TCNT3,
    #[doc = "0x4a - Timer/Counter3 Control Register B"]
    pub tccr3b: TCCR3B,
    #[doc = "0x4b - Timer/Counter3 Control Register A"]
    pub tccr3a: TCCR3A,
    #[doc = "0x4c - Timer/Counter3 Control Register C"]
    pub tccr3c: TCCR3C,
}
#[doc = "ETIFR (rw) register accessor: an alias for `Reg<ETIFR_SPEC>`"]
pub type ETIFR = crate::Reg<etifr::ETIFR_SPEC>;
#[doc = "Extended Timer/Counter Interrupt Flag register"]
pub mod etifr;
#[doc = "ETIMSK (rw) register accessor: an alias for `Reg<ETIMSK_SPEC>`"]
pub type ETIMSK = crate::Reg<etimsk::ETIMSK_SPEC>;
#[doc = "Extended Timer/Counter Interrupt Mask Register"]
pub mod etimsk;
#[doc = "ICR3 (rw) register accessor: an alias for `Reg<ICR3_SPEC>`"]
pub type ICR3 = crate::Reg<icr3::ICR3_SPEC>;
#[doc = "Timer/Counter3 Input Capture Register Bytes"]
pub mod icr3;
#[doc = "OCR3A (rw) register accessor: an alias for `Reg<OCR3A_SPEC>`"]
pub type OCR3A = crate::Reg<ocr3a::OCR3A_SPEC>;
#[doc = "Timer/Counter3 Output Compare Register A Bytes"]
pub mod ocr3a;
#[doc = "OCR3B (rw) register accessor: an alias for `Reg<OCR3B_SPEC>`"]
pub type OCR3B = crate::Reg<ocr3b::OCR3B_SPEC>;
#[doc = "Timer/Counter3 Output Compare Register B Bytes"]
pub mod ocr3b;
#[doc = "OCR3C (rw) register accessor: an alias for `Reg<OCR3C_SPEC>`"]
pub type OCR3C = crate::Reg<ocr3c::OCR3C_SPEC>;
#[doc = "Timer/Counter3 Output compare Register C Bytes"]
pub mod ocr3c;
#[doc = "SFIOR (rw) register accessor: an alias for `Reg<SFIOR_SPEC>`"]
pub type SFIOR = crate::Reg<sfior::SFIOR_SPEC>;
#[doc = "Special Function IO Register"]
pub mod sfior;
#[doc = "TCCR3A (rw) register accessor: an alias for `Reg<TCCR3A_SPEC>`"]
pub type TCCR3A = crate::Reg<tccr3a::TCCR3A_SPEC>;
#[doc = "Timer/Counter3 Control Register A"]
pub mod tccr3a;
#[doc = "TCCR3B (rw) register accessor: an alias for `Reg<TCCR3B_SPEC>`"]
pub type TCCR3B = crate::Reg<tccr3b::TCCR3B_SPEC>;
#[doc = "Timer/Counter3 Control Register B"]
pub mod tccr3b;
#[doc = "TCCR3C (rw) register accessor: an alias for `Reg<TCCR3C_SPEC>`"]
pub type TCCR3C = crate::Reg<tccr3c::TCCR3C_SPEC>;
#[doc = "Timer/Counter3 Control Register C"]
pub mod tccr3c;
#[doc = "TCNT3 (rw) register accessor: an alias for `Reg<TCNT3_SPEC>`"]
pub type TCNT3 = crate::Reg<tcnt3::TCNT3_SPEC>;
#[doc = "Timer/Counter3 Bytes"]
pub mod tcnt3;
