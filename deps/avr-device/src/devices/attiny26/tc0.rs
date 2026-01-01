#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Counter 0"]
    pub tcnt0: TCNT0,
    #[doc = "0x01 - Timer/Counter0 Control Register"]
    pub tccr0: TCCR0,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - Timer/Counter Interrupt Flag register"]
    pub tifr: TIFR,
    #[doc = "0x07 - Timer/Counter Interrupt Mask Register"]
    pub timsk: TIMSK,
}
#[doc = "TCCR0 (rw) register accessor: an alias for `Reg<TCCR0_SPEC>`"]
pub type TCCR0 = crate::Reg<tccr0::TCCR0_SPEC>;
#[doc = "Timer/Counter0 Control Register"]
pub mod tccr0;
#[doc = "TCNT0 (rw) register accessor: an alias for `Reg<TCNT0_SPEC>`"]
pub type TCNT0 = crate::Reg<tcnt0::TCNT0_SPEC>;
#[doc = "Timer Counter 0"]
pub mod tcnt0;
#[doc = "TIFR (rw) register accessor: an alias for `Reg<TIFR_SPEC>`"]
pub type TIFR = crate::Reg<tifr::TIFR_SPEC>;
#[doc = "Timer/Counter Interrupt Flag register"]
pub mod tifr;
#[doc = "TIMSK (rw) register accessor: an alias for `Reg<TIMSK_SPEC>`"]
pub type TIMSK = crate::Reg<timsk::TIMSK_SPEC>;
#[doc = "Timer/Counter Interrupt Mask Register"]
pub mod timsk;
