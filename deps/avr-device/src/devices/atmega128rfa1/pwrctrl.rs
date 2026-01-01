#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Control Register"]
    pub mcucr: MCUCR,
    _reserved1: [u8; 0xd9],
    #[doc = "0xda - Low Leakage Voltage Regulator Control Register"]
    pub llcr: LLCR,
    #[doc = "0xdb - Low Leakage Voltage Regulator Data Register (Low-Byte)"]
    pub lldrl: LLDRL,
    #[doc = "0xdc - Low Leakage Voltage Regulator Data Register (High-Byte)"]
    pub lldrh: LLDRH,
    #[doc = "0xdd - Data Retention Configuration Register of SRAM 3"]
    pub drtram3: DRTRAM3,
    #[doc = "0xde - Data Retention Configuration Register of SRAM 2"]
    pub drtram2: DRTRAM2,
    #[doc = "0xdf - Data Retention Configuration Register of SRAM 1"]
    pub drtram1: DRTRAM1,
    #[doc = "0xe0 - Data Retention Configuration Register of SRAM 0"]
    pub drtram0: DRTRAM0,
    #[doc = "0xe1 - Port Driver Strength Register 0"]
    pub dpds0: DPDS0,
    #[doc = "0xe2 - Port Driver Strength Register 1"]
    pub dpds1: DPDS1,
    _reserved10: [u8; 0x01],
    #[doc = "0xe4 - Transceiver Pin Register"]
    pub trxpr: TRXPR,
}
#[doc = "DPDS0 (rw) register accessor: an alias for `Reg<DPDS0_SPEC>`"]
pub type DPDS0 = crate::Reg<dpds0::DPDS0_SPEC>;
#[doc = "Port Driver Strength Register 0"]
pub mod dpds0;
#[doc = "DPDS1 (rw) register accessor: an alias for `Reg<DPDS1_SPEC>`"]
pub type DPDS1 = crate::Reg<dpds1::DPDS1_SPEC>;
#[doc = "Port Driver Strength Register 1"]
pub mod dpds1;
#[doc = "DRTRAM0 (rw) register accessor: an alias for `Reg<DRTRAM0_SPEC>`"]
pub type DRTRAM0 = crate::Reg<drtram0::DRTRAM0_SPEC>;
#[doc = "Data Retention Configuration Register of SRAM 0"]
pub mod drtram0;
#[doc = "DRTRAM1 (rw) register accessor: an alias for `Reg<DRTRAM1_SPEC>`"]
pub type DRTRAM1 = crate::Reg<drtram1::DRTRAM1_SPEC>;
#[doc = "Data Retention Configuration Register of SRAM 1"]
pub mod drtram1;
#[doc = "DRTRAM2 (rw) register accessor: an alias for `Reg<DRTRAM2_SPEC>`"]
pub type DRTRAM2 = crate::Reg<drtram2::DRTRAM2_SPEC>;
#[doc = "Data Retention Configuration Register of SRAM 2"]
pub mod drtram2;
#[doc = "DRTRAM3 (rw) register accessor: an alias for `Reg<DRTRAM3_SPEC>`"]
pub type DRTRAM3 = crate::Reg<drtram3::DRTRAM3_SPEC>;
#[doc = "Data Retention Configuration Register of SRAM 3"]
pub mod drtram3;
#[doc = "LLCR (rw) register accessor: an alias for `Reg<LLCR_SPEC>`"]
pub type LLCR = crate::Reg<llcr::LLCR_SPEC>;
#[doc = "Low Leakage Voltage Regulator Control Register"]
pub mod llcr;
#[doc = "LLDRH (rw) register accessor: an alias for `Reg<LLDRH_SPEC>`"]
pub type LLDRH = crate::Reg<lldrh::LLDRH_SPEC>;
#[doc = "Low Leakage Voltage Regulator Data Register (High-Byte)"]
pub mod lldrh;
#[doc = "LLDRL (rw) register accessor: an alias for `Reg<LLDRL_SPEC>`"]
pub type LLDRL = crate::Reg<lldrl::LLDRL_SPEC>;
#[doc = "Low Leakage Voltage Regulator Data Register (Low-Byte)"]
pub mod lldrl;
#[doc = "MCUCR (rw) register accessor: an alias for `Reg<MCUCR_SPEC>`"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "TRXPR (rw) register accessor: an alias for `Reg<TRXPR_SPEC>`"]
pub type TRXPR = crate::Reg<trxpr::TRXPR_SPEC>;
#[doc = "Transceiver Pin Register"]
pub mod trxpr;
