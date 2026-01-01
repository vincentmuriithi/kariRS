#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART Control and Status Register A"]
    pub ucsr3a: UCSR3A,
    #[doc = "0x01 - USART Control and Status Register B"]
    pub ucsr3b: UCSR3B,
    #[doc = "0x02 - USART Control and Status Register C"]
    pub ucsr3c: UCSR3C,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - USART Baud Rate Register Bytes"]
    pub ubrr3: UBRR3,
    #[doc = "0x06 - USART I/O Data Register"]
    pub udr3: UDR3,
}
#[doc = "UBRR3 (rw) register accessor: an alias for `Reg<UBRR3_SPEC>`"]
pub type UBRR3 = crate::Reg<ubrr3::UBRR3_SPEC>;
#[doc = "USART Baud Rate Register Bytes"]
pub mod ubrr3;
#[doc = "UCSR3A (rw) register accessor: an alias for `Reg<UCSR3A_SPEC>`"]
pub type UCSR3A = crate::Reg<ucsr3a::UCSR3A_SPEC>;
#[doc = "USART Control and Status Register A"]
pub mod ucsr3a;
#[doc = "UCSR3B (rw) register accessor: an alias for `Reg<UCSR3B_SPEC>`"]
pub type UCSR3B = crate::Reg<ucsr3b::UCSR3B_SPEC>;
#[doc = "USART Control and Status Register B"]
pub mod ucsr3b;
#[doc = "UCSR3C (rw) register accessor: an alias for `Reg<UCSR3C_SPEC>`"]
pub type UCSR3C = crate::Reg<ucsr3c::UCSR3C_SPEC>;
#[doc = "USART Control and Status Register C"]
pub mod ucsr3c;
#[doc = "UDR3 (rw) register accessor: an alias for `Reg<UDR3_SPEC>`"]
pub type UDR3 = crate::Reg<udr3::UDR3_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr3;
