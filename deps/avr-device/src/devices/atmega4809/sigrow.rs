#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device ID Byte 0"]
    pub deviceid0: DEVICEID0,
    #[doc = "0x01 - Device ID Byte 1"]
    pub deviceid1: DEVICEID1,
    #[doc = "0x02 - Device ID Byte 2"]
    pub deviceid2: DEVICEID2,
    #[doc = "0x03 - Serial Number Byte 0"]
    pub sernum0: SERNUM0,
    #[doc = "0x04 - Serial Number Byte 1"]
    pub sernum1: SERNUM1,
    #[doc = "0x05 - Serial Number Byte 2"]
    pub sernum2: SERNUM2,
    #[doc = "0x06 - Serial Number Byte 3"]
    pub sernum3: SERNUM3,
    #[doc = "0x07 - Serial Number Byte 4"]
    pub sernum4: SERNUM4,
    #[doc = "0x08 - Serial Number Byte 5"]
    pub sernum5: SERNUM5,
    #[doc = "0x09 - Serial Number Byte 6"]
    pub sernum6: SERNUM6,
    #[doc = "0x0a - Serial Number Byte 7"]
    pub sernum7: SERNUM7,
    #[doc = "0x0b - Serial Number Byte 8"]
    pub sernum8: SERNUM8,
    #[doc = "0x0c - Serial Number Byte 9"]
    pub sernum9: SERNUM9,
    _reserved13: [u8; 0x07],
    #[doc = "0x14 - Oscillator Calibration for 32kHz ULP"]
    pub osccal32k: OSCCAL32K,
    _reserved14: [u8; 0x03],
    #[doc = "0x18 - Oscillator Calibration 16 MHz Byte 0"]
    pub osccal16m0: OSCCAL16M0,
    #[doc = "0x19 - Oscillator Calibration 16 MHz Byte 1"]
    pub osccal16m1: OSCCAL16M1,
    #[doc = "0x1a - Oscillator Calibration 20 MHz Byte 0"]
    pub osccal20m0: OSCCAL20M0,
    #[doc = "0x1b - Oscillator Calibration 20 MHz Byte 1"]
    pub osccal20m1: OSCCAL20M1,
    _reserved18: [u8; 0x04],
    #[doc = "0x20 - Temperature Sensor Calibration Byte 0"]
    pub tempsense0: TEMPSENSE0,
    #[doc = "0x21 - Temperature Sensor Calibration Byte 1"]
    pub tempsense1: TEMPSENSE1,
    #[doc = "0x22 - OSC16 error at 3V"]
    pub osc16err3v: OSC16ERR3V,
    #[doc = "0x23 - OSC16 error at 5V"]
    pub osc16err5v: OSC16ERR5V,
    #[doc = "0x24 - OSC20 error at 3V"]
    pub osc20err3v: OSC20ERR3V,
    #[doc = "0x25 - OSC20 error at 5V"]
    pub osc20err5v: OSC20ERR5V,
    _reserved24: [u8; 0x09],
    #[doc = "0x2f - CRC Checksum Byte 1"]
    pub checksum1: CHECKSUM1,
}
#[doc = "CHECKSUM1 (r) register accessor: an alias for `Reg<CHECKSUM1_SPEC>`"]
pub type CHECKSUM1 = crate::Reg<checksum1::CHECKSUM1_SPEC>;
#[doc = "CRC Checksum Byte 1"]
pub mod checksum1;
#[doc = "DEVICEID0 (r) register accessor: an alias for `Reg<DEVICEID0_SPEC>`"]
pub type DEVICEID0 = crate::Reg<deviceid0::DEVICEID0_SPEC>;
#[doc = "Device ID Byte 0"]
pub mod deviceid0;
#[doc = "DEVICEID1 (r) register accessor: an alias for `Reg<DEVICEID1_SPEC>`"]
pub type DEVICEID1 = crate::Reg<deviceid1::DEVICEID1_SPEC>;
#[doc = "Device ID Byte 1"]
pub mod deviceid1;
#[doc = "DEVICEID2 (r) register accessor: an alias for `Reg<DEVICEID2_SPEC>`"]
pub type DEVICEID2 = crate::Reg<deviceid2::DEVICEID2_SPEC>;
#[doc = "Device ID Byte 2"]
pub mod deviceid2;
#[doc = "OSC16ERR3V (r) register accessor: an alias for `Reg<OSC16ERR3V_SPEC>`"]
pub type OSC16ERR3V = crate::Reg<osc16err3v::OSC16ERR3V_SPEC>;
#[doc = "OSC16 error at 3V"]
pub mod osc16err3v;
#[doc = "OSC16ERR5V (r) register accessor: an alias for `Reg<OSC16ERR5V_SPEC>`"]
pub type OSC16ERR5V = crate::Reg<osc16err5v::OSC16ERR5V_SPEC>;
#[doc = "OSC16 error at 5V"]
pub mod osc16err5v;
#[doc = "OSC20ERR3V (r) register accessor: an alias for `Reg<OSC20ERR3V_SPEC>`"]
pub type OSC20ERR3V = crate::Reg<osc20err3v::OSC20ERR3V_SPEC>;
#[doc = "OSC20 error at 3V"]
pub mod osc20err3v;
#[doc = "OSC20ERR5V (r) register accessor: an alias for `Reg<OSC20ERR5V_SPEC>`"]
pub type OSC20ERR5V = crate::Reg<osc20err5v::OSC20ERR5V_SPEC>;
#[doc = "OSC20 error at 5V"]
pub mod osc20err5v;
#[doc = "OSCCAL16M0 (r) register accessor: an alias for `Reg<OSCCAL16M0_SPEC>`"]
pub type OSCCAL16M0 = crate::Reg<osccal16m0::OSCCAL16M0_SPEC>;
#[doc = "Oscillator Calibration 16 MHz Byte 0"]
pub mod osccal16m0;
#[doc = "OSCCAL16M1 (r) register accessor: an alias for `Reg<OSCCAL16M1_SPEC>`"]
pub type OSCCAL16M1 = crate::Reg<osccal16m1::OSCCAL16M1_SPEC>;
#[doc = "Oscillator Calibration 16 MHz Byte 1"]
pub mod osccal16m1;
#[doc = "OSCCAL20M0 (r) register accessor: an alias for `Reg<OSCCAL20M0_SPEC>`"]
pub type OSCCAL20M0 = crate::Reg<osccal20m0::OSCCAL20M0_SPEC>;
#[doc = "Oscillator Calibration 20 MHz Byte 0"]
pub mod osccal20m0;
#[doc = "OSCCAL20M1 (r) register accessor: an alias for `Reg<OSCCAL20M1_SPEC>`"]
pub type OSCCAL20M1 = crate::Reg<osccal20m1::OSCCAL20M1_SPEC>;
#[doc = "Oscillator Calibration 20 MHz Byte 1"]
pub mod osccal20m1;
#[doc = "OSCCAL32K (r) register accessor: an alias for `Reg<OSCCAL32K_SPEC>`"]
pub type OSCCAL32K = crate::Reg<osccal32k::OSCCAL32K_SPEC>;
#[doc = "Oscillator Calibration for 32kHz ULP"]
pub mod osccal32k;
#[doc = "SERNUM0 (r) register accessor: an alias for `Reg<SERNUM0_SPEC>`"]
pub type SERNUM0 = crate::Reg<sernum0::SERNUM0_SPEC>;
#[doc = "Serial Number Byte 0"]
pub mod sernum0;
#[doc = "SERNUM1 (r) register accessor: an alias for `Reg<SERNUM1_SPEC>`"]
pub type SERNUM1 = crate::Reg<sernum1::SERNUM1_SPEC>;
#[doc = "Serial Number Byte 1"]
pub mod sernum1;
#[doc = "SERNUM2 (r) register accessor: an alias for `Reg<SERNUM2_SPEC>`"]
pub type SERNUM2 = crate::Reg<sernum2::SERNUM2_SPEC>;
#[doc = "Serial Number Byte 2"]
pub mod sernum2;
#[doc = "SERNUM3 (r) register accessor: an alias for `Reg<SERNUM3_SPEC>`"]
pub type SERNUM3 = crate::Reg<sernum3::SERNUM3_SPEC>;
#[doc = "Serial Number Byte 3"]
pub mod sernum3;
#[doc = "SERNUM4 (r) register accessor: an alias for `Reg<SERNUM4_SPEC>`"]
pub type SERNUM4 = crate::Reg<sernum4::SERNUM4_SPEC>;
#[doc = "Serial Number Byte 4"]
pub mod sernum4;
#[doc = "SERNUM5 (r) register accessor: an alias for `Reg<SERNUM5_SPEC>`"]
pub type SERNUM5 = crate::Reg<sernum5::SERNUM5_SPEC>;
#[doc = "Serial Number Byte 5"]
pub mod sernum5;
#[doc = "SERNUM6 (r) register accessor: an alias for `Reg<SERNUM6_SPEC>`"]
pub type SERNUM6 = crate::Reg<sernum6::SERNUM6_SPEC>;
#[doc = "Serial Number Byte 6"]
pub mod sernum6;
#[doc = "SERNUM7 (r) register accessor: an alias for `Reg<SERNUM7_SPEC>`"]
pub type SERNUM7 = crate::Reg<sernum7::SERNUM7_SPEC>;
#[doc = "Serial Number Byte 7"]
pub mod sernum7;
#[doc = "SERNUM8 (r) register accessor: an alias for `Reg<SERNUM8_SPEC>`"]
pub type SERNUM8 = crate::Reg<sernum8::SERNUM8_SPEC>;
#[doc = "Serial Number Byte 8"]
pub mod sernum8;
#[doc = "SERNUM9 (r) register accessor: an alias for `Reg<SERNUM9_SPEC>`"]
pub type SERNUM9 = crate::Reg<sernum9::SERNUM9_SPEC>;
#[doc = "Serial Number Byte 9"]
pub mod sernum9;
#[doc = "TEMPSENSE0 (r) register accessor: an alias for `Reg<TEMPSENSE0_SPEC>`"]
pub type TEMPSENSE0 = crate::Reg<tempsense0::TEMPSENSE0_SPEC>;
#[doc = "Temperature Sensor Calibration Byte 0"]
pub mod tempsense0;
#[doc = "TEMPSENSE1 (r) register accessor: an alias for `Reg<TEMPSENSE1_SPEC>`"]
pub type TEMPSENSE1 = crate::Reg<tempsense1::TEMPSENSE1_SPEC>;
#[doc = "Temperature Sensor Calibration Byte 1"]
pub mod tempsense1;
