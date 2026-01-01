#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter4 Interrupt Flag register"]
    pub tifr4: TIFR4,
    _reserved1: [u8; 0x38],
    #[doc = "0x39 - Timer/Counter4 Interrupt Mask Register"]
    pub timsk4: TIMSK4,
    _reserved2: [u8; 0x2d],
    #[doc = "0x67 - Timer/Counter4 Control Register A"]
    pub tccr4a: TCCR4A,
    #[doc = "0x68 - Timer/Counter4 Control Register B"]
    pub tccr4b: TCCR4B,
    #[doc = "0x69 - Timer/Counter 4 Control Register C"]
    pub tccr4c: TCCR4C,
    _reserved5: [u8; 0x01],
    #[doc = "0x6b - Timer/Counter4 Bytes"]
    pub tcnt4: TCNT4,
    #[doc = "0x6d - Timer/Counter4 Input Capture Register Bytes"]
    pub icr4: ICR4,
    #[doc = "0x6f - Timer/Counter4 Output Compare Register A Bytes"]
    pub ocr4a: OCR4A,
    #[doc = "0x71 - Timer/Counter4 Output Compare Register B Bytes"]
    pub ocr4b: OCR4B,
    #[doc = "0x73 - Timer/Counter4 Output Compare Register B Bytes"]
    pub ocr4c: OCR4C,
}
#[doc = "ICR4 (rw) register accessor: an alias for `Reg<ICR4_SPEC>`"]
pub type ICR4 = crate::Reg<icr4::ICR4_SPEC>;
#[doc = "Timer/Counter4 Input Capture Register Bytes"]
pub mod icr4;
#[doc = "OCR4A (rw) register accessor: an alias for `Reg<OCR4A_SPEC>`"]
pub type OCR4A = crate::Reg<ocr4a::OCR4A_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register A Bytes"]
pub mod ocr4a;
#[doc = "OCR4B (rw) register accessor: an alias for `Reg<OCR4B_SPEC>`"]
pub type OCR4B = crate::Reg<ocr4b::OCR4B_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register B Bytes"]
pub mod ocr4b;
#[doc = "OCR4C (rw) register accessor: an alias for `Reg<OCR4C_SPEC>`"]
pub type OCR4C = crate::Reg<ocr4c::OCR4C_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register B Bytes"]
pub mod ocr4c;
#[doc = "TCCR4A (rw) register accessor: an alias for `Reg<TCCR4A_SPEC>`"]
pub type TCCR4A = crate::Reg<tccr4a::TCCR4A_SPEC>;
#[doc = "Timer/Counter4 Control Register A"]
pub mod tccr4a;
#[doc = "TCCR4B (rw) register accessor: an alias for `Reg<TCCR4B_SPEC>`"]
pub type TCCR4B = crate::Reg<tccr4b::TCCR4B_SPEC>;
#[doc = "Timer/Counter4 Control Register B"]
pub mod tccr4b;
#[doc = "TCCR4C (rw) register accessor: an alias for `Reg<TCCR4C_SPEC>`"]
pub type TCCR4C = crate::Reg<tccr4c::TCCR4C_SPEC>;
#[doc = "Timer/Counter 4 Control Register C"]
pub mod tccr4c;
#[doc = "TCNT4 (rw) register accessor: an alias for `Reg<TCNT4_SPEC>`"]
pub type TCNT4 = crate::Reg<tcnt4::TCNT4_SPEC>;
#[doc = "Timer/Counter4 Bytes"]
pub mod tcnt4;
#[doc = "TIFR4 (rw) register accessor: an alias for `Reg<TIFR4_SPEC>`"]
pub type TIFR4 = crate::Reg<tifr4::TIFR4_SPEC>;
#[doc = "Timer/Counter4 Interrupt Flag register"]
pub mod tifr4;
#[doc = "TIMSK4 (rw) register accessor: an alias for `Reg<TIMSK4_SPEC>`"]
pub type TIMSK4 = crate::Reg<timsk4::TIMSK4_SPEC>;
#[doc = "Timer/Counter4 Interrupt Mask Register"]
pub mod timsk4;
