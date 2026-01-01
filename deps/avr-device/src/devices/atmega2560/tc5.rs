#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter5 Interrupt Flag register"]
    pub tifr5: TIFR5,
    _reserved1: [u8; 0x38],
    #[doc = "0x39 - Timer/Counter5 Interrupt Mask Register"]
    pub timsk5: TIMSK5,
    _reserved2: [u8; 0xac],
    #[doc = "0xe6 - Timer/Counter5 Control Register A"]
    pub tccr5a: TCCR5A,
    #[doc = "0xe7 - Timer/Counter5 Control Register B"]
    pub tccr5b: TCCR5B,
    #[doc = "0xe8 - Timer/Counter 5 Control Register C"]
    pub tccr5c: TCCR5C,
    _reserved5: [u8; 0x01],
    #[doc = "0xea - Timer/Counter5 Bytes"]
    pub tcnt5: TCNT5,
    #[doc = "0xec - Timer/Counter5 Input Capture Register Bytes"]
    pub icr5: ICR5,
    #[doc = "0xee - Timer/Counter5 Output Compare Register A Bytes"]
    pub ocr5a: OCR5A,
    #[doc = "0xf0 - Timer/Counter5 Output Compare Register B Bytes"]
    pub ocr5b: OCR5B,
    #[doc = "0xf2 - Timer/Counter5 Output Compare Register B Bytes"]
    pub ocr5c: OCR5C,
}
#[doc = "ICR5 (rw) register accessor: an alias for `Reg<ICR5_SPEC>`"]
pub type ICR5 = crate::Reg<icr5::ICR5_SPEC>;
#[doc = "Timer/Counter5 Input Capture Register Bytes"]
pub mod icr5;
#[doc = "OCR5A (rw) register accessor: an alias for `Reg<OCR5A_SPEC>`"]
pub type OCR5A = crate::Reg<ocr5a::OCR5A_SPEC>;
#[doc = "Timer/Counter5 Output Compare Register A Bytes"]
pub mod ocr5a;
#[doc = "OCR5B (rw) register accessor: an alias for `Reg<OCR5B_SPEC>`"]
pub type OCR5B = crate::Reg<ocr5b::OCR5B_SPEC>;
#[doc = "Timer/Counter5 Output Compare Register B Bytes"]
pub mod ocr5b;
#[doc = "OCR5C (rw) register accessor: an alias for `Reg<OCR5C_SPEC>`"]
pub type OCR5C = crate::Reg<ocr5c::OCR5C_SPEC>;
#[doc = "Timer/Counter5 Output Compare Register B Bytes"]
pub mod ocr5c;
#[doc = "TCCR5A (rw) register accessor: an alias for `Reg<TCCR5A_SPEC>`"]
pub type TCCR5A = crate::Reg<tccr5a::TCCR5A_SPEC>;
#[doc = "Timer/Counter5 Control Register A"]
pub mod tccr5a;
#[doc = "TCCR5B (rw) register accessor: an alias for `Reg<TCCR5B_SPEC>`"]
pub type TCCR5B = crate::Reg<tccr5b::TCCR5B_SPEC>;
#[doc = "Timer/Counter5 Control Register B"]
pub mod tccr5b;
#[doc = "TCCR5C (rw) register accessor: an alias for `Reg<TCCR5C_SPEC>`"]
pub type TCCR5C = crate::Reg<tccr5c::TCCR5C_SPEC>;
#[doc = "Timer/Counter 5 Control Register C"]
pub mod tccr5c;
#[doc = "TCNT5 (rw) register accessor: an alias for `Reg<TCNT5_SPEC>`"]
pub type TCNT5 = crate::Reg<tcnt5::TCNT5_SPEC>;
#[doc = "Timer/Counter5 Bytes"]
pub mod tcnt5;
#[doc = "TIFR5 (rw) register accessor: an alias for `Reg<TIFR5_SPEC>`"]
pub type TIFR5 = crate::Reg<tifr5::TIFR5_SPEC>;
#[doc = "Timer/Counter5 Interrupt Flag register"]
pub mod tifr5;
#[doc = "TIMSK5 (rw) register accessor: an alias for `Reg<TIMSK5_SPEC>`"]
pub type TIMSK5 = crate::Reg<timsk5::TIMSK5_SPEC>;
#[doc = "Timer/Counter5 Interrupt Mask Register"]
pub mod timsk5;
