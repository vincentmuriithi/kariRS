#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART I/O Data Register"]
    pub udr1: UDR1,
    #[doc = "0x01 - USART Baud Rate Register Bytes"]
    pub ubrr1: UBRR1,
    #[doc = "0x03 - USART Control and Status Register D"]
    pub ucsr1d: UCSR1D,
    #[doc = "0x04 - USART Control and Status Register C"]
    pub ucsr1c: UCSR1C,
    #[doc = "0x05 - USART Control and Status Register B"]
    pub ucsr1b: UCSR1B,
    #[doc = "0x06 - USART Control and Status Register A"]
    pub ucsr1a: UCSR1A,
}
#[doc = "UBRR1 (rw) register accessor: an alias for `Reg<UBRR1_SPEC>`"]
pub type UBRR1 = crate::Reg<ubrr1::UBRR1_SPEC>;
#[doc = "USART Baud Rate Register Bytes"]
pub mod ubrr1;
#[doc = "UCSR1A (rw) register accessor: an alias for `Reg<UCSR1A_SPEC>`"]
pub type UCSR1A = crate::Reg<ucsr1a::UCSR1A_SPEC>;
#[doc = "USART Control and Status Register A"]
pub mod ucsr1a;
#[doc = "UCSR1B (rw) register accessor: an alias for `Reg<UCSR1B_SPEC>`"]
pub type UCSR1B = crate::Reg<ucsr1b::UCSR1B_SPEC>;
#[doc = "USART Control and Status Register B"]
pub mod ucsr1b;
#[doc = "UCSR1C (rw) register accessor: an alias for `Reg<UCSR1C_SPEC>`"]
pub type UCSR1C = crate::Reg<ucsr1c::UCSR1C_SPEC>;
#[doc = "USART Control and Status Register C"]
pub mod ucsr1c;
#[doc = "UCSR1D (rw) register accessor: an alias for `Reg<UCSR1D_SPEC>`"]
pub type UCSR1D = crate::Reg<ucsr1d::UCSR1D_SPEC>;
#[doc = "USART Control and Status Register D"]
pub mod ucsr1d;
#[doc = "UDR1 (rw) register accessor: an alias for `Reg<UDR1_SPEC>`"]
pub type UDR1 = crate::Reg<udr1::UDR1_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr1;
