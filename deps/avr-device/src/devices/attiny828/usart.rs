#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART Control and Status Register A"]
    pub ucsra: UCSRA,
    #[doc = "0x01 - USART Control and Status Register B"]
    pub ucsrb: UCSRB,
    #[doc = "0x02 - USART Control and Status Register C"]
    pub ucsrc: UCSRC,
    #[doc = "0x03 - USART Control and Status Register D"]
    pub ucsrd: UCSRD,
    #[doc = "0x04 - USART Baud Rate Register Bytes"]
    pub ubrr: UBRR,
    #[doc = "0x06 - USART I/O Data Register"]
    pub udr: UDR,
}
#[doc = "UBRR (rw) register accessor: an alias for `Reg<UBRR_SPEC>`"]
pub type UBRR = crate::Reg<ubrr::UBRR_SPEC>;
#[doc = "USART Baud Rate Register Bytes"]
pub mod ubrr;
#[doc = "UCSRA (rw) register accessor: an alias for `Reg<UCSRA_SPEC>`"]
pub type UCSRA = crate::Reg<ucsra::UCSRA_SPEC>;
#[doc = "USART Control and Status Register A"]
pub mod ucsra;
#[doc = "UCSRB (rw) register accessor: an alias for `Reg<UCSRB_SPEC>`"]
pub type UCSRB = crate::Reg<ucsrb::UCSRB_SPEC>;
#[doc = "USART Control and Status Register B"]
pub mod ucsrb;
#[doc = "UCSRC (rw) register accessor: an alias for `Reg<UCSRC_SPEC>`"]
pub type UCSRC = crate::Reg<ucsrc::UCSRC_SPEC>;
#[doc = "USART Control and Status Register C"]
pub mod ucsrc;
#[doc = "UCSRD (rw) register accessor: an alias for `Reg<UCSRD_SPEC>`"]
pub type UCSRD = crate::Reg<ucsrd::UCSRD_SPEC>;
#[doc = "USART Control and Status Register D"]
pub mod ucsrd;
#[doc = "UDR (rw) register accessor: an alias for `Reg<UDR_SPEC>`"]
pub type UDR = crate::Reg<udr::UDR_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr;
