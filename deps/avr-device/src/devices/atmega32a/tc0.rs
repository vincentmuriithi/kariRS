#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter Register"]
    pub tcnt0: TCNT0,
    #[doc = "0x01 - Timer/Counter Control Register"]
    pub tccr0: TCCR0,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - Timer/Counter Interrupt Flag register"]
    pub tifr: TIFR,
    #[doc = "0x07 - Timer/Counter Interrupt Mask Register"]
    pub timsk: TIMSK,
    _reserved4: [u8; 0x02],
    #[doc = "0x0a - Output Compare Register"]
    pub ocr0: OCR0,
}
#[doc = "OCR0 (rw) register accessor: an alias for `Reg<OCR0_SPEC>`"]
pub type OCR0 = crate::Reg<ocr0::OCR0_SPEC>;
#[doc = "Output Compare Register"]
pub mod ocr0;
#[doc = "TCCR0 (rw) register accessor: an alias for `Reg<TCCR0_SPEC>`"]
pub type TCCR0 = crate::Reg<tccr0::TCCR0_SPEC>;
#[doc = "Timer/Counter Control Register"]
pub mod tccr0;
#[doc = "TCNT0 (rw) register accessor: an alias for `Reg<TCNT0_SPEC>`"]
pub type TCNT0 = crate::Reg<tcnt0::TCNT0_SPEC>;
#[doc = "Timer/Counter Register"]
pub mod tcnt0;
#[doc = "TIFR (rw) register accessor: an alias for `Reg<TIFR_SPEC>`"]
pub type TIFR = crate::Reg<tifr::TIFR_SPEC>;
#[doc = "Timer/Counter Interrupt Flag register"]
pub mod tifr;
#[doc = "TIMSK (rw) register accessor: an alias for `Reg<TIMSK_SPEC>`"]
pub type TIMSK = crate::Reg<timsk::TIMSK_SPEC>;
#[doc = "Timer/Counter Interrupt Mask Register"]
pub mod timsk;
