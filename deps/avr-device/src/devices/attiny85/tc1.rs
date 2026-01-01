#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Dead time prescaler register"]
    pub dtps: DTPS,
    #[doc = "0x01 - Dead Time Value Register B"]
    pub dt1b: DT1B,
    #[doc = "0x02 - Dead Time Value Register A"]
    pub dt1a: DT1A,
    _reserved3: [u8; 0x05],
    #[doc = "0x08 - Output Compare Register B"]
    pub ocr1b: OCR1B,
    #[doc = "0x09 - Timer counter control register"]
    pub gtccr: GTCCR,
    #[doc = "0x0a - Output Compare Register C"]
    pub ocr1c: OCR1C,
    #[doc = "0x0b - Output Compare Register A"]
    pub ocr1a: OCR1A,
    #[doc = "0x0c - Timer/Counter Register"]
    pub tcnt1: TCNT1,
    #[doc = "0x0d - Timer/Counter Control Register"]
    pub tccr1: TCCR1,
    _reserved9: [u8; 0x07],
    #[doc = "0x15 - Timer/Counter Interrupt Flag Register"]
    pub tifr: TIFR,
    #[doc = "0x16 - Timer/Counter Interrupt Mask Register"]
    pub timsk: TIMSK,
}
#[doc = "DT1A (rw) register accessor: an alias for `Reg<DT1A_SPEC>`"]
pub type DT1A = crate::Reg<dt1a::DT1A_SPEC>;
#[doc = "Dead Time Value Register A"]
pub mod dt1a;
#[doc = "DT1B (rw) register accessor: an alias for `Reg<DT1B_SPEC>`"]
pub type DT1B = crate::Reg<dt1b::DT1B_SPEC>;
#[doc = "Dead Time Value Register B"]
pub mod dt1b;
#[doc = "DTPS (rw) register accessor: an alias for `Reg<DTPS_SPEC>`"]
pub type DTPS = crate::Reg<dtps::DTPS_SPEC>;
#[doc = "Dead time prescaler register"]
pub mod dtps;
#[doc = "GTCCR (rw) register accessor: an alias for `Reg<GTCCR_SPEC>`"]
pub type GTCCR = crate::Reg<gtccr::GTCCR_SPEC>;
#[doc = "Timer counter control register"]
pub mod gtccr;
#[doc = "OCR1A (rw) register accessor: an alias for `Reg<OCR1A_SPEC>`"]
pub type OCR1A = crate::Reg<ocr1a::OCR1A_SPEC>;
#[doc = "Output Compare Register A"]
pub mod ocr1a;
#[doc = "OCR1B (rw) register accessor: an alias for `Reg<OCR1B_SPEC>`"]
pub type OCR1B = crate::Reg<ocr1b::OCR1B_SPEC>;
#[doc = "Output Compare Register B"]
pub mod ocr1b;
#[doc = "OCR1C (rw) register accessor: an alias for `Reg<OCR1C_SPEC>`"]
pub type OCR1C = crate::Reg<ocr1c::OCR1C_SPEC>;
#[doc = "Output Compare Register C"]
pub mod ocr1c;
#[doc = "TCCR1 (rw) register accessor: an alias for `Reg<TCCR1_SPEC>`"]
pub type TCCR1 = crate::Reg<tccr1::TCCR1_SPEC>;
#[doc = "Timer/Counter Control Register"]
pub mod tccr1;
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
