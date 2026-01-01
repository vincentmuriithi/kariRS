#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PLL Control and Status Register"]
    pub pllcsr: PLLCSR,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Output Compare Register"]
    pub ocr1c: OCR1C,
    #[doc = "0x03 - Output Compare Register"]
    pub ocr1b: OCR1B,
    #[doc = "0x04 - Output Compare Register"]
    pub ocr1a: OCR1A,
    #[doc = "0x05 - Timer/Counter Register"]
    pub tcnt1: TCNT1,
    #[doc = "0x06 - Timer/Counter Control Register B"]
    pub tccr1b: TCCR1B,
    #[doc = "0x07 - Timer/Counter Control Register A"]
    pub tccr1a: TCCR1A,
    _reserved7: [u8; 0x07],
    #[doc = "0x0f - Timer/Counter Interrupt Flag Register"]
    pub tifr: TIFR,
    #[doc = "0x10 - Timer/Counter Interrupt Mask Register"]
    pub timsk: TIMSK,
}
#[doc = "OCR1A (rw) register accessor: an alias for `Reg<OCR1A_SPEC>`"]
pub type OCR1A = crate::Reg<ocr1a::OCR1A_SPEC>;
#[doc = "Output Compare Register"]
pub mod ocr1a;
#[doc = "OCR1B (rw) register accessor: an alias for `Reg<OCR1B_SPEC>`"]
pub type OCR1B = crate::Reg<ocr1b::OCR1B_SPEC>;
#[doc = "Output Compare Register"]
pub mod ocr1b;
#[doc = "OCR1C (rw) register accessor: an alias for `Reg<OCR1C_SPEC>`"]
pub type OCR1C = crate::Reg<ocr1c::OCR1C_SPEC>;
#[doc = "Output Compare Register"]
pub mod ocr1c;
#[doc = "PLLCSR (rw) register accessor: an alias for `Reg<PLLCSR_SPEC>`"]
pub type PLLCSR = crate::Reg<pllcsr::PLLCSR_SPEC>;
#[doc = "PLL Control and Status Register"]
pub mod pllcsr;
#[doc = "TCCR1A (rw) register accessor: an alias for `Reg<TCCR1A_SPEC>`"]
pub type TCCR1A = crate::Reg<tccr1a::TCCR1A_SPEC>;
#[doc = "Timer/Counter Control Register A"]
pub mod tccr1a;
#[doc = "TCCR1B (rw) register accessor: an alias for `Reg<TCCR1B_SPEC>`"]
pub type TCCR1B = crate::Reg<tccr1b::TCCR1B_SPEC>;
#[doc = "Timer/Counter Control Register B"]
pub mod tccr1b;
#[doc = "TCNT1 (rw) register accessor: an alias for `Reg<TCNT1_SPEC>`"]
pub type TCNT1 = crate::Reg<tcnt1::TCNT1_SPEC>;
#[doc = "Timer/Counter Register"]
pub mod tcnt1;
#[doc = "TIFR (rw) register accessor: an alias for `Reg<TIFR_SPEC>`"]
pub type TIFR = crate::Reg<tifr::TIFR_SPEC>;
#[doc = "Timer/Counter Interrupt Flag Register"]
pub mod tifr;
#[doc = "TIMSK (rw) register accessor: an alias for `Reg<TIMSK_SPEC>`"]
pub type TIMSK = crate::Reg<timsk::TIMSK_SPEC>;
#[doc = "Timer/Counter Interrupt Mask Register"]
pub mod timsk;
