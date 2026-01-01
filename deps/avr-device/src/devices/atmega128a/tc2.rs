#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Output Compare Register"]
    pub ocr2: OCR2,
    #[doc = "0x01 - Timer/Counter Register"]
    pub tcnt2: TCNT2,
    #[doc = "0x02 - Timer/Counter Control Register"]
    pub tccr2: TCCR2,
    _reserved3: [u8; 0x10],
    #[doc = "0x13 - Timer/Counter Interrupt Flag Register"]
    pub tifr: TIFR,
    #[doc = "0x14 - No Description."]
    pub timsk: TIMSK,
}
#[doc = "OCR2 (rw) register accessor: an alias for `Reg<OCR2_SPEC>`"]
pub type OCR2 = crate::Reg<ocr2::OCR2_SPEC>;
#[doc = "Output Compare Register"]
pub mod ocr2;
#[doc = "TCCR2 (rw) register accessor: an alias for `Reg<TCCR2_SPEC>`"]
pub type TCCR2 = crate::Reg<tccr2::TCCR2_SPEC>;
#[doc = "Timer/Counter Control Register"]
pub mod tccr2;
#[doc = "TCNT2 (rw) register accessor: an alias for `Reg<TCNT2_SPEC>`"]
pub type TCNT2 = crate::Reg<tcnt2::TCNT2_SPEC>;
#[doc = "Timer/Counter Register"]
pub mod tcnt2;
#[doc = "TIFR (rw) register accessor: an alias for `Reg<TIFR_SPEC>`"]
pub type TIFR = crate::Reg<tifr::TIFR_SPEC>;
#[doc = "Timer/Counter Interrupt Flag Register"]
pub mod tifr;
#[doc = "TIMSK (rw) register accessor: an alias for `Reg<TIMSK_SPEC>`"]
pub type TIMSK = crate::Reg<timsk::TIMSK_SPEC>;
#[doc = "No Description."]
pub mod timsk;
