#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART Control and Status Register A"]
    pub ucsr2a: UCSR2A,
    #[doc = "0x01 - USART Control and Status Register B"]
    pub ucsr2b: UCSR2B,
    #[doc = "0x02 - USART Control and Status Register C"]
    pub ucsr2c: UCSR2C,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - USART Baud Rate Register Bytes"]
    pub ubrr2: UBRR2,
    #[doc = "0x06 - USART I/O Data Register"]
    pub udr2: UDR2,
}
#[doc = "UBRR2 (rw) register accessor: an alias for `Reg<UBRR2_SPEC>`"]
pub type UBRR2 = crate::Reg<ubrr2::UBRR2_SPEC>;
#[doc = "USART Baud Rate Register Bytes"]
pub mod ubrr2;
#[doc = "UCSR2A (rw) register accessor: an alias for `Reg<UCSR2A_SPEC>`"]
pub type UCSR2A = crate::Reg<ucsr2a::UCSR2A_SPEC>;
#[doc = "USART Control and Status Register A"]
pub mod ucsr2a;
#[doc = "UCSR2B (rw) register accessor: an alias for `Reg<UCSR2B_SPEC>`"]
pub type UCSR2B = crate::Reg<ucsr2b::UCSR2B_SPEC>;
#[doc = "USART Control and Status Register B"]
pub mod ucsr2b;
#[doc = "UCSR2C (rw) register accessor: an alias for `Reg<UCSR2C_SPEC>`"]
pub type UCSR2C = crate::Reg<ucsr2c::UCSR2C_SPEC>;
#[doc = "USART Control and Status Register C"]
pub mod ucsr2c;
#[doc = "UDR2 (rw) register accessor: an alias for `Reg<UDR2_SPEC>`"]
pub type UDR2 = crate::Reg<udr2::UDR2_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr2;
