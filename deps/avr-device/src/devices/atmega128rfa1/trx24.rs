#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES Control Register"]
    pub aes_ctrl: AES_CTRL,
    #[doc = "0x01 - AES Status Register"]
    pub aes_status: AES_STATUS,
    #[doc = "0x02 - AES Plain and Cipher Text Buffer Register"]
    pub aes_state: AES_STATE,
    #[doc = "0x03 - AES Encryption and Decryption Key Buffer Register"]
    pub aes_key: AES_KEY,
    _reserved4: [u8; 0x01],
    #[doc = "0x05 - Transceiver Status Register"]
    pub trx_status: TRX_STATUS,
    #[doc = "0x06 - Transceiver State Control Register"]
    pub trx_state: TRX_STATE,
    #[doc = "0x07 - Reserved"]
    pub trx_ctrl_0: TRX_CTRL_0,
    #[doc = "0x08 - Transceiver Control Register 1"]
    pub trx_ctrl_1: TRX_CTRL_1,
    #[doc = "0x09 - Transceiver Transmit Power Control Register"]
    pub phy_tx_pwr: PHY_TX_PWR,
    #[doc = "0x0a - Receiver Signal Strength Indicator Register"]
    pub phy_rssi: PHY_RSSI,
    #[doc = "0x0b - Transceiver Energy Detection Level Register"]
    pub phy_ed_level: PHY_ED_LEVEL,
    #[doc = "0x0c - Transceiver Clear Channel Assessment (CCA) Control Register"]
    pub phy_cc_cca: PHY_CC_CCA,
    #[doc = "0x0d - Transceiver CCA Threshold Setting Register"]
    pub cca_thres: CCA_THRES,
    #[doc = "0x0e - Transceiver Receive Control Register"]
    pub rx_ctrl: RX_CTRL,
    #[doc = "0x0f - Start of Frame Delimiter Value Register"]
    pub sfd_value: SFD_VALUE,
    #[doc = "0x10 - Transceiver Control Register 2"]
    pub trx_ctrl_2: TRX_CTRL_2,
    #[doc = "0x11 - Antenna Diversity Control Register"]
    pub ant_div: ANT_DIV,
    #[doc = "0x12 - Transceiver Interrupt Enable Register"]
    pub irq_mask: IRQ_MASK,
    #[doc = "0x13 - Transceiver Interrupt Status Register"]
    pub irq_status: IRQ_STATUS,
    #[doc = "0x14 - Voltage Regulator Control and Status Register"]
    pub vreg_ctrl: VREG_CTRL,
    #[doc = "0x15 - Battery Monitor Control and Status Register"]
    pub batmon: BATMON,
    #[doc = "0x16 - Crystal Oscillator Control Register"]
    pub xosc_ctrl: XOSC_CTRL,
    _reserved22: [u8; 0x02],
    #[doc = "0x19 - Transceiver Receiver Sensitivity Control Register"]
    pub rx_syn: RX_SYN,
    _reserved23: [u8; 0x01],
    #[doc = "0x1b - Transceiver Acknowledgment Frame Control Register 1"]
    pub xah_ctrl_1: XAH_CTRL_1,
    #[doc = "0x1c - Transceiver Filter Tuning Control Register"]
    pub ftn_ctrl: FTN_CTRL,
    _reserved25: [u8; 0x01],
    #[doc = "0x1e - Transceiver Center Frequency Calibration Control Register"]
    pub pll_cf: PLL_CF,
    #[doc = "0x1f - Transceiver Delay Cell Calibration Control Register"]
    pub pll_dcu: PLL_DCU,
    #[doc = "0x20 - Device Identification Register (Part Number)"]
    pub part_num: PART_NUM,
    #[doc = "0x21 - Device Identification Register (Version Number)"]
    pub version_num: VERSION_NUM,
    #[doc = "0x22 - Device Identification Register (Manufacture ID Low Byte)"]
    pub man_id_0: MAN_ID_0,
    #[doc = "0x23 - Device Identification Register (Manufacture ID High Byte)"]
    pub man_id_1: MAN_ID_1,
    #[doc = "0x24 - Transceiver MAC Short Address Register (Low Byte)"]
    pub short_addr_0: SHORT_ADDR_0,
    #[doc = "0x25 - Transceiver MAC Short Address Register (High Byte)"]
    pub short_addr_1: SHORT_ADDR_1,
    #[doc = "0x26 - Transceiver Personal Area Network ID Register (Low Byte)"]
    pub pan_id_0: PAN_ID_0,
    #[doc = "0x27 - Transceiver Personal Area Network ID Register (High Byte)"]
    pub pan_id_1: PAN_ID_1,
    #[doc = "0x28 - Transceiver MAC IEEE Address Register 0"]
    pub ieee_addr_0: IEEE_ADDR_0,
    #[doc = "0x29 - Transceiver MAC IEEE Address Register 1"]
    pub ieee_addr_1: IEEE_ADDR_1,
    #[doc = "0x2a - Transceiver MAC IEEE Address Register 2"]
    pub ieee_addr_2: IEEE_ADDR_2,
    #[doc = "0x2b - Transceiver MAC IEEE Address Register 3"]
    pub ieee_addr_3: IEEE_ADDR_3,
    #[doc = "0x2c - Transceiver MAC IEEE Address Register 4"]
    pub ieee_addr_4: IEEE_ADDR_4,
    #[doc = "0x2d - Transceiver MAC IEEE Address Register 5"]
    pub ieee_addr_5: IEEE_ADDR_5,
    #[doc = "0x2e - Transceiver MAC IEEE Address Register 6"]
    pub ieee_addr_6: IEEE_ADDR_6,
    #[doc = "0x2f - Transceiver MAC IEEE Address Register 7"]
    pub ieee_addr_7: IEEE_ADDR_7,
    #[doc = "0x30 - Transceiver Extended Operating Mode Control Register"]
    pub xah_ctrl_0: XAH_CTRL_0,
    #[doc = "0x31 - Transceiver CSMA-CA Random Number Generator Seed Register"]
    pub csma_seed_0: CSMA_SEED_0,
    #[doc = "0x32 - Transceiver Acknowledgment Frame Control Register 2"]
    pub csma_seed_1: CSMA_SEED_1,
    #[doc = "0x33 - Transceiver CSMA-CA Back-off Exponent Control Register"]
    pub csma_be: CSMA_BE,
    _reserved47: [u8; 0x06],
    #[doc = "0x3a - Transceiver Digital Test Control Register"]
    pub tst_ctrl_digi: TST_CTRL_DIGI,
    _reserved48: [u8; 0x04],
    #[doc = "0x3f - Transceiver Received Frame Length Register"]
    pub tst_rx_length: TST_RX_LENGTH,
    _reserved49: [u8; 0x04],
    #[doc = "0x44 - Start of frame buffer"]
    pub trxfbst: TRXFBST,
    _reserved50: [u8; 0x7e],
    #[doc = "0xc3 - End of frame buffer"]
    pub trxfbend: TRXFBEND,
}
#[doc = "AES_CTRL (rw) register accessor: an alias for `Reg<AES_CTRL_SPEC>`"]
pub type AES_CTRL = crate::Reg<aes_ctrl::AES_CTRL_SPEC>;
#[doc = "AES Control Register"]
pub mod aes_ctrl;
#[doc = "AES_KEY (rw) register accessor: an alias for `Reg<AES_KEY_SPEC>`"]
pub type AES_KEY = crate::Reg<aes_key::AES_KEY_SPEC>;
#[doc = "AES Encryption and Decryption Key Buffer Register"]
pub mod aes_key;
#[doc = "AES_STATE (rw) register accessor: an alias for `Reg<AES_STATE_SPEC>`"]
pub type AES_STATE = crate::Reg<aes_state::AES_STATE_SPEC>;
#[doc = "AES Plain and Cipher Text Buffer Register"]
pub mod aes_state;
#[doc = "AES_STATUS (rw) register accessor: an alias for `Reg<AES_STATUS_SPEC>`"]
pub type AES_STATUS = crate::Reg<aes_status::AES_STATUS_SPEC>;
#[doc = "AES Status Register"]
pub mod aes_status;
#[doc = "ANT_DIV (rw) register accessor: an alias for `Reg<ANT_DIV_SPEC>`"]
pub type ANT_DIV = crate::Reg<ant_div::ANT_DIV_SPEC>;
#[doc = "Antenna Diversity Control Register"]
pub mod ant_div;
#[doc = "BATMON (rw) register accessor: an alias for `Reg<BATMON_SPEC>`"]
pub type BATMON = crate::Reg<batmon::BATMON_SPEC>;
#[doc = "Battery Monitor Control and Status Register"]
pub mod batmon;
#[doc = "CCA_THRES (rw) register accessor: an alias for `Reg<CCA_THRES_SPEC>`"]
pub type CCA_THRES = crate::Reg<cca_thres::CCA_THRES_SPEC>;
#[doc = "Transceiver CCA Threshold Setting Register"]
pub mod cca_thres;
#[doc = "CSMA_BE (rw) register accessor: an alias for `Reg<CSMA_BE_SPEC>`"]
pub type CSMA_BE = crate::Reg<csma_be::CSMA_BE_SPEC>;
#[doc = "Transceiver CSMA-CA Back-off Exponent Control Register"]
pub mod csma_be;
#[doc = "CSMA_SEED_0 (rw) register accessor: an alias for `Reg<CSMA_SEED_0_SPEC>`"]
pub type CSMA_SEED_0 = crate::Reg<csma_seed_0::CSMA_SEED_0_SPEC>;
#[doc = "Transceiver CSMA-CA Random Number Generator Seed Register"]
pub mod csma_seed_0;
#[doc = "CSMA_SEED_1 (rw) register accessor: an alias for `Reg<CSMA_SEED_1_SPEC>`"]
pub type CSMA_SEED_1 = crate::Reg<csma_seed_1::CSMA_SEED_1_SPEC>;
#[doc = "Transceiver Acknowledgment Frame Control Register 2"]
pub mod csma_seed_1;
#[doc = "FTN_CTRL (rw) register accessor: an alias for `Reg<FTN_CTRL_SPEC>`"]
pub type FTN_CTRL = crate::Reg<ftn_ctrl::FTN_CTRL_SPEC>;
#[doc = "Transceiver Filter Tuning Control Register"]
pub mod ftn_ctrl;
#[doc = "IEEE_ADDR_0 (rw) register accessor: an alias for `Reg<IEEE_ADDR_0_SPEC>`"]
pub type IEEE_ADDR_0 = crate::Reg<ieee_addr_0::IEEE_ADDR_0_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 0"]
pub mod ieee_addr_0;
#[doc = "IEEE_ADDR_1 (rw) register accessor: an alias for `Reg<IEEE_ADDR_1_SPEC>`"]
pub type IEEE_ADDR_1 = crate::Reg<ieee_addr_1::IEEE_ADDR_1_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 1"]
pub mod ieee_addr_1;
#[doc = "IEEE_ADDR_2 (rw) register accessor: an alias for `Reg<IEEE_ADDR_2_SPEC>`"]
pub type IEEE_ADDR_2 = crate::Reg<ieee_addr_2::IEEE_ADDR_2_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 2"]
pub mod ieee_addr_2;
#[doc = "IEEE_ADDR_3 (rw) register accessor: an alias for `Reg<IEEE_ADDR_3_SPEC>`"]
pub type IEEE_ADDR_3 = crate::Reg<ieee_addr_3::IEEE_ADDR_3_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 3"]
pub mod ieee_addr_3;
#[doc = "IEEE_ADDR_4 (rw) register accessor: an alias for `Reg<IEEE_ADDR_4_SPEC>`"]
pub type IEEE_ADDR_4 = crate::Reg<ieee_addr_4::IEEE_ADDR_4_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 4"]
pub mod ieee_addr_4;
#[doc = "IEEE_ADDR_5 (rw) register accessor: an alias for `Reg<IEEE_ADDR_5_SPEC>`"]
pub type IEEE_ADDR_5 = crate::Reg<ieee_addr_5::IEEE_ADDR_5_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 5"]
pub mod ieee_addr_5;
#[doc = "IEEE_ADDR_6 (rw) register accessor: an alias for `Reg<IEEE_ADDR_6_SPEC>`"]
pub type IEEE_ADDR_6 = crate::Reg<ieee_addr_6::IEEE_ADDR_6_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 6"]
pub mod ieee_addr_6;
#[doc = "IEEE_ADDR_7 (rw) register accessor: an alias for `Reg<IEEE_ADDR_7_SPEC>`"]
pub type IEEE_ADDR_7 = crate::Reg<ieee_addr_7::IEEE_ADDR_7_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 7"]
pub mod ieee_addr_7;
#[doc = "IRQ_MASK (rw) register accessor: an alias for `Reg<IRQ_MASK_SPEC>`"]
pub type IRQ_MASK = crate::Reg<irq_mask::IRQ_MASK_SPEC>;
#[doc = "Transceiver Interrupt Enable Register"]
pub mod irq_mask;
#[doc = "IRQ_STATUS (rw) register accessor: an alias for `Reg<IRQ_STATUS_SPEC>`"]
pub type IRQ_STATUS = crate::Reg<irq_status::IRQ_STATUS_SPEC>;
#[doc = "Transceiver Interrupt Status Register"]
pub mod irq_status;
#[doc = "MAN_ID_0 (rw) register accessor: an alias for `Reg<MAN_ID_0_SPEC>`"]
pub type MAN_ID_0 = crate::Reg<man_id_0::MAN_ID_0_SPEC>;
#[doc = "Device Identification Register (Manufacture ID Low Byte)"]
pub mod man_id_0;
#[doc = "MAN_ID_1 (rw) register accessor: an alias for `Reg<MAN_ID_1_SPEC>`"]
pub type MAN_ID_1 = crate::Reg<man_id_1::MAN_ID_1_SPEC>;
#[doc = "Device Identification Register (Manufacture ID High Byte)"]
pub mod man_id_1;
#[doc = "PAN_ID_0 (rw) register accessor: an alias for `Reg<PAN_ID_0_SPEC>`"]
pub type PAN_ID_0 = crate::Reg<pan_id_0::PAN_ID_0_SPEC>;
#[doc = "Transceiver Personal Area Network ID Register (Low Byte)"]
pub mod pan_id_0;
#[doc = "PAN_ID_1 (rw) register accessor: an alias for `Reg<PAN_ID_1_SPEC>`"]
pub type PAN_ID_1 = crate::Reg<pan_id_1::PAN_ID_1_SPEC>;
#[doc = "Transceiver Personal Area Network ID Register (High Byte)"]
pub mod pan_id_1;
#[doc = "PART_NUM (rw) register accessor: an alias for `Reg<PART_NUM_SPEC>`"]
pub type PART_NUM = crate::Reg<part_num::PART_NUM_SPEC>;
#[doc = "Device Identification Register (Part Number)"]
pub mod part_num;
#[doc = "PHY_CC_CCA (rw) register accessor: an alias for `Reg<PHY_CC_CCA_SPEC>`"]
pub type PHY_CC_CCA = crate::Reg<phy_cc_cca::PHY_CC_CCA_SPEC>;
#[doc = "Transceiver Clear Channel Assessment (CCA) Control Register"]
pub mod phy_cc_cca;
#[doc = "PHY_ED_LEVEL (rw) register accessor: an alias for `Reg<PHY_ED_LEVEL_SPEC>`"]
pub type PHY_ED_LEVEL = crate::Reg<phy_ed_level::PHY_ED_LEVEL_SPEC>;
#[doc = "Transceiver Energy Detection Level Register"]
pub mod phy_ed_level;
#[doc = "PHY_RSSI (rw) register accessor: an alias for `Reg<PHY_RSSI_SPEC>`"]
pub type PHY_RSSI = crate::Reg<phy_rssi::PHY_RSSI_SPEC>;
#[doc = "Receiver Signal Strength Indicator Register"]
pub mod phy_rssi;
#[doc = "PHY_TX_PWR (rw) register accessor: an alias for `Reg<PHY_TX_PWR_SPEC>`"]
pub type PHY_TX_PWR = crate::Reg<phy_tx_pwr::PHY_TX_PWR_SPEC>;
#[doc = "Transceiver Transmit Power Control Register"]
pub mod phy_tx_pwr;
#[doc = "PLL_CF (rw) register accessor: an alias for `Reg<PLL_CF_SPEC>`"]
pub type PLL_CF = crate::Reg<pll_cf::PLL_CF_SPEC>;
#[doc = "Transceiver Center Frequency Calibration Control Register"]
pub mod pll_cf;
#[doc = "PLL_DCU (rw) register accessor: an alias for `Reg<PLL_DCU_SPEC>`"]
pub type PLL_DCU = crate::Reg<pll_dcu::PLL_DCU_SPEC>;
#[doc = "Transceiver Delay Cell Calibration Control Register"]
pub mod pll_dcu;
#[doc = "RX_CTRL (rw) register accessor: an alias for `Reg<RX_CTRL_SPEC>`"]
pub type RX_CTRL = crate::Reg<rx_ctrl::RX_CTRL_SPEC>;
#[doc = "Transceiver Receive Control Register"]
pub mod rx_ctrl;
#[doc = "RX_SYN (rw) register accessor: an alias for `Reg<RX_SYN_SPEC>`"]
pub type RX_SYN = crate::Reg<rx_syn::RX_SYN_SPEC>;
#[doc = "Transceiver Receiver Sensitivity Control Register"]
pub mod rx_syn;
#[doc = "SFD_VALUE (rw) register accessor: an alias for `Reg<SFD_VALUE_SPEC>`"]
pub type SFD_VALUE = crate::Reg<sfd_value::SFD_VALUE_SPEC>;
#[doc = "Start of Frame Delimiter Value Register"]
pub mod sfd_value;
#[doc = "SHORT_ADDR_0 (rw) register accessor: an alias for `Reg<SHORT_ADDR_0_SPEC>`"]
pub type SHORT_ADDR_0 = crate::Reg<short_addr_0::SHORT_ADDR_0_SPEC>;
#[doc = "Transceiver MAC Short Address Register (Low Byte)"]
pub mod short_addr_0;
#[doc = "SHORT_ADDR_1 (rw) register accessor: an alias for `Reg<SHORT_ADDR_1_SPEC>`"]
pub type SHORT_ADDR_1 = crate::Reg<short_addr_1::SHORT_ADDR_1_SPEC>;
#[doc = "Transceiver MAC Short Address Register (High Byte)"]
pub mod short_addr_1;
#[doc = "TRXFBEND (rw) register accessor: an alias for `Reg<TRXFBEND_SPEC>`"]
pub type TRXFBEND = crate::Reg<trxfbend::TRXFBEND_SPEC>;
#[doc = "End of frame buffer"]
pub mod trxfbend;
#[doc = "TRXFBST (rw) register accessor: an alias for `Reg<TRXFBST_SPEC>`"]
pub type TRXFBST = crate::Reg<trxfbst::TRXFBST_SPEC>;
#[doc = "Start of frame buffer"]
pub mod trxfbst;
#[doc = "TRX_CTRL_0 (rw) register accessor: an alias for `Reg<TRX_CTRL_0_SPEC>`"]
pub type TRX_CTRL_0 = crate::Reg<trx_ctrl_0::TRX_CTRL_0_SPEC>;
#[doc = "Reserved"]
pub mod trx_ctrl_0;
#[doc = "TRX_CTRL_1 (rw) register accessor: an alias for `Reg<TRX_CTRL_1_SPEC>`"]
pub type TRX_CTRL_1 = crate::Reg<trx_ctrl_1::TRX_CTRL_1_SPEC>;
#[doc = "Transceiver Control Register 1"]
pub mod trx_ctrl_1;
#[doc = "TRX_CTRL_2 (rw) register accessor: an alias for `Reg<TRX_CTRL_2_SPEC>`"]
pub type TRX_CTRL_2 = crate::Reg<trx_ctrl_2::TRX_CTRL_2_SPEC>;
#[doc = "Transceiver Control Register 2"]
pub mod trx_ctrl_2;
#[doc = "TRX_STATE (rw) register accessor: an alias for `Reg<TRX_STATE_SPEC>`"]
pub type TRX_STATE = crate::Reg<trx_state::TRX_STATE_SPEC>;
#[doc = "Transceiver State Control Register"]
pub mod trx_state;
#[doc = "TRX_STATUS (rw) register accessor: an alias for `Reg<TRX_STATUS_SPEC>`"]
pub type TRX_STATUS = crate::Reg<trx_status::TRX_STATUS_SPEC>;
#[doc = "Transceiver Status Register"]
pub mod trx_status;
#[doc = "TST_CTRL_DIGI (rw) register accessor: an alias for `Reg<TST_CTRL_DIGI_SPEC>`"]
pub type TST_CTRL_DIGI = crate::Reg<tst_ctrl_digi::TST_CTRL_DIGI_SPEC>;
#[doc = "Transceiver Digital Test Control Register"]
pub mod tst_ctrl_digi;
#[doc = "TST_RX_LENGTH (rw) register accessor: an alias for `Reg<TST_RX_LENGTH_SPEC>`"]
pub type TST_RX_LENGTH = crate::Reg<tst_rx_length::TST_RX_LENGTH_SPEC>;
#[doc = "Transceiver Received Frame Length Register"]
pub mod tst_rx_length;
#[doc = "VERSION_NUM (rw) register accessor: an alias for `Reg<VERSION_NUM_SPEC>`"]
pub type VERSION_NUM = crate::Reg<version_num::VERSION_NUM_SPEC>;
#[doc = "Device Identification Register (Version Number)"]
pub mod version_num;
#[doc = "VREG_CTRL (rw) register accessor: an alias for `Reg<VREG_CTRL_SPEC>`"]
pub type VREG_CTRL = crate::Reg<vreg_ctrl::VREG_CTRL_SPEC>;
#[doc = "Voltage Regulator Control and Status Register"]
pub mod vreg_ctrl;
#[doc = "XAH_CTRL_0 (rw) register accessor: an alias for `Reg<XAH_CTRL_0_SPEC>`"]
pub type XAH_CTRL_0 = crate::Reg<xah_ctrl_0::XAH_CTRL_0_SPEC>;
#[doc = "Transceiver Extended Operating Mode Control Register"]
pub mod xah_ctrl_0;
#[doc = "XAH_CTRL_1 (rw) register accessor: an alias for `Reg<XAH_CTRL_1_SPEC>`"]
pub type XAH_CTRL_1 = crate::Reg<xah_ctrl_1::XAH_CTRL_1_SPEC>;
#[doc = "Transceiver Acknowledgment Frame Control Register 1"]
pub mod xah_ctrl_1;
#[doc = "XOSC_CTRL (rw) register accessor: an alias for `Reg<XOSC_CTRL_SPEC>`"]
pub type XOSC_CTRL = crate::Reg<xosc_ctrl::XOSC_CTRL_SPEC>;
#[doc = "Crystal Oscillator Control Register"]
pub mod xosc_ctrl;
