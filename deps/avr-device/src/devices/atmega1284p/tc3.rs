#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter Interrupt Flag register"]
    pub tifr3: TIFR3,
    _reserved1: [u8; 0x38],
    #[doc = "0x39 - Timer/Counter3 Interrupt Mask Register"]
    pub timsk3: TIMSK3,
    _reserved2: [u8; 0x1e],
    #[doc = "0x58 - Timer/Counter3 Control Register A"]
    pub tccr3a: TCCR3A,
    #[doc = "0x59 - Timer/Counter3 Control Register B"]
    pub tccr3b: TCCR3B,
    #[doc = "0x5a - Timer/Counter3 Control Register C"]
    pub tccr3c: TCCR3C,
    _reserved5: [u8; 0x01],
    #[doc = "0x5c - Timer/Counter3 Bytes"]
    pub tcnt3: TCNT3,
    #[doc = "0x5e - Timer/Counter3 Input Capture Register Bytes"]
    pub icr3: ICR3,
    #[doc = "0x60 - Timer/Counter3 Output Compare Register A Bytes"]
    pub ocr3a: OCR3A,
    #[doc = "0x62 - Timer/Counter3 Output Compare Register B Bytes"]
    pub ocr3b: OCR3B,
}
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
#[doc = "TIFR3 (rw) register accessor: an alias for `Reg<TIFR3_SPEC>`"]
pub type TIFR3 = crate::Reg<tifr3::TIFR3_SPEC>;
#[doc = "Timer/Counter Interrupt Flag register"]
pub mod tifr3;
#[doc = "TIMSK3 (rw) register accessor: an alias for `Reg<TIMSK3_SPEC>`"]
pub type TIMSK3 = crate::Reg<timsk3::TIMSK3_SPEC>;
#[doc = "Timer/Counter3 Interrupt Mask Register"]
pub mod timsk3;
