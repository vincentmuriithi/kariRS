#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Receive Data Low Byte"]
    pub rxdatal: RXDATAL,
    #[doc = "0x01 - Receive Data High Byte"]
    pub rxdatah: RXDATAH,
    #[doc = "0x02 - Transmit Data Low Byte"]
    pub txdatal: TXDATAL,
    #[doc = "0x03 - Transmit Data High Byte"]
    pub txdatah: TXDATAH,
    #[doc = "0x04 - Status"]
    pub status: STATUS,
    #[doc = "0x05 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x06 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x07 - Control C"]
    pub ctrlc: CTRLC,
    #[doc = "0x08 - Baud Rate"]
    pub baud: BAUD,
    #[doc = "0x0a - Control D"]
    pub ctrld: CTRLD,
    #[doc = "0x0b - Debug Control"]
    pub dbgctrl: DBGCTRL,
    #[doc = "0x0c - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x0d - IRCOM Transmitter Pulse Length Control"]
    pub txplctrl: TXPLCTRL,
    #[doc = "0x0e - IRCOM Receiver Pulse Length Control"]
    pub rxplctrl: RXPLCTRL,
}
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baud Rate"]
pub mod baud;
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "CTRLD (rw) register accessor: an alias for `Reg<CTRLD_SPEC>`"]
pub type CTRLD = crate::Reg<ctrld::CTRLD_SPEC>;
#[doc = "Control D"]
pub mod ctrld;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "RXDATAH (r) register accessor: an alias for `Reg<RXDATAH_SPEC>`"]
pub type RXDATAH = crate::Reg<rxdatah::RXDATAH_SPEC>;
#[doc = "Receive Data High Byte"]
pub mod rxdatah;
#[doc = "RXDATAL (r) register accessor: an alias for `Reg<RXDATAL_SPEC>`"]
pub type RXDATAL = crate::Reg<rxdatal::RXDATAL_SPEC>;
#[doc = "Receive Data Low Byte"]
pub mod rxdatal;
#[doc = "RXPLCTRL (rw) register accessor: an alias for `Reg<RXPLCTRL_SPEC>`"]
pub type RXPLCTRL = crate::Reg<rxplctrl::RXPLCTRL_SPEC>;
#[doc = "IRCOM Receiver Pulse Length Control"]
pub mod rxplctrl;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "TXDATAH (rw) register accessor: an alias for `Reg<TXDATAH_SPEC>`"]
pub type TXDATAH = crate::Reg<txdatah::TXDATAH_SPEC>;
#[doc = "Transmit Data High Byte"]
pub mod txdatah;
#[doc = "TXDATAL (rw) register accessor: an alias for `Reg<TXDATAL_SPEC>`"]
pub type TXDATAL = crate::Reg<txdatal::TXDATAL_SPEC>;
#[doc = "Transmit Data Low Byte"]
pub mod txdatal;
#[doc = "TXPLCTRL (rw) register accessor: an alias for `Reg<TXPLCTRL_SPEC>`"]
pub type TXPLCTRL = crate::Reg<txplctrl::TXPLCTRL_SPEC>;
#[doc = "IRCOM Transmitter Pulse Length Control"]
pub mod txplctrl;
