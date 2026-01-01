#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter Interrupt Flag register"]
    pub tifr1: TIFR1,
    #[doc = "0x01 - Timer/Counter1 Interrupt Mask Register"]
    pub timsk1: TIMSK1,
    _reserved2: [u8; 0x15],
    #[doc = "0x17 - Timer/Counter1 Control Register C"]
    pub tccr1c: TCCR1C,
    _reserved3: [u8; 0x01],
    #[doc = "0x19 - Timer/Counter1 Input Capture Register Bytes"]
    pub icr1: ICR1,
    _reserved4: [u8; 0x02],
    #[doc = "0x1d - Timer/Counter1 Output Compare Register B Bytes"]
    pub ocr1b: OCR1B,
    #[doc = "0x1f - Timer/Counter1 Output Compare Register A Bytes"]
    pub ocr1a: OCR1A,
    #[doc = "0x21 - Timer/Counter1 Bytes"]
    pub tcnt1: TCNT1,
    #[doc = "0x23 - Timer/Counter1 Control Register B"]
    pub tccr1b: TCCR1B,
    #[doc = "0x24 - Timer/Counter1 Control Register A"]
    pub tccr1a: TCCR1A,
}
#[doc = "ICR1 (rw) register accessor: an alias for `Reg<ICR1_SPEC>`"]
pub type ICR1 = crate::Reg<icr1::ICR1_SPEC>;
#[doc = "Timer/Counter1 Input Capture Register Bytes"]
pub mod icr1;
#[doc = "OCR1A (rw) register accessor: an alias for `Reg<OCR1A_SPEC>`"]
pub type OCR1A = crate::Reg<ocr1a::OCR1A_SPEC>;
#[doc = "Timer/Counter1 Output Compare Register A Bytes"]
pub mod ocr1a;
#[doc = "OCR1B (rw) register accessor: an alias for `Reg<OCR1B_SPEC>`"]
pub type OCR1B = crate::Reg<ocr1b::OCR1B_SPEC>;
#[doc = "Timer/Counter1 Output Compare Register B Bytes"]
pub mod ocr1b;
#[doc = "TCCR1A (rw) register accessor: an alias for `Reg<TCCR1A_SPEC>`"]
pub type TCCR1A = crate::Reg<tccr1a::TCCR1A_SPEC>;
#[doc = "Timer/Counter1 Control Register A"]
pub mod tccr1a;
#[doc = "TCCR1B (rw) register accessor: an alias for `Reg<TCCR1B_SPEC>`"]
pub type TCCR1B = crate::Reg<tccr1b::TCCR1B_SPEC>;
#[doc = "Timer/Counter1 Control Register B"]
pub mod tccr1b;
#[doc = "TCCR1C (rw) register accessor: an alias for `Reg<TCCR1C_SPEC>`"]
pub type TCCR1C = crate::Reg<tccr1c::TCCR1C_SPEC>;
#[doc = "Timer/Counter1 Control Register C"]
pub mod tccr1c;
#[doc = "TCNT1 (rw) register accessor: an alias for `Reg<TCNT1_SPEC>`"]
pub type TCNT1 = crate::Reg<tcnt1::TCNT1_SPEC>;
#[doc = "Timer/Counter1 Bytes"]
pub mod tcnt1;
#[doc = "TIFR1 (rw) register accessor: an alias for `Reg<TIFR1_SPEC>`"]
pub type TIFR1 = crate::Reg<tifr1::TIFR1_SPEC>;
#[doc = "Timer/Counter Interrupt Flag register"]
pub mod tifr1;
#[doc = "TIMSK1 (rw) register accessor: an alias for `Reg<TIMSK1_SPEC>`"]
pub type TIMSK1 = crate::Reg<timsk1::TIMSK1_SPEC>;
#[doc = "Timer/Counter1 Interrupt Mask Register"]
pub mod timsk1;
