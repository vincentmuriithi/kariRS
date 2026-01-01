#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device ID Byte 0"]
    pub deviceid0: DEVICEID0,
    #[doc = "0x01 - Device ID Byte 1"]
    pub deviceid1: DEVICEID1,
    #[doc = "0x02 - Device ID Byte 2"]
    pub deviceid2: DEVICEID2,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - Temperature Calibration 0"]
    pub tempsense0: TEMPSENSE0,
    #[doc = "0x06 - Temperature Calibration 1"]
    pub tempsense1: TEMPSENSE1,
    _reserved5: [u8; 0x08],
    #[doc = "0x10 - LOTNUM0"]
    pub sernum0: SERNUM0,
    #[doc = "0x11 - LOTNUM1"]
    pub sernum1: SERNUM1,
    #[doc = "0x12 - LOTNUM2"]
    pub sernum2: SERNUM2,
    #[doc = "0x13 - LOTNUM3"]
    pub sernum3: SERNUM3,
    #[doc = "0x14 - LOTNUM4"]
    pub sernum4: SERNUM4,
    #[doc = "0x15 - LOTNUM5"]
    pub sernum5: SERNUM5,
    #[doc = "0x16 - RANDOM"]
    pub sernum6: SERNUM6,
    #[doc = "0x17 - SCRIBE"]
    pub sernum7: SERNUM7,
    #[doc = "0x18 - XPOS0"]
    pub sernum8: SERNUM8,
    #[doc = "0x19 - XPOS1"]
    pub sernum9: SERNUM9,
    #[doc = "0x1a - YPOS0"]
    pub sernum10: SERNUM10,
    #[doc = "0x1b - YPOS1"]
    pub sernum11: SERNUM11,
    #[doc = "0x1c - RES0"]
    pub sernum12: SERNUM12,
    #[doc = "0x1d - RES1"]
    pub sernum13: SERNUM13,
    #[doc = "0x1e - RES2"]
    pub sernum14: SERNUM14,
    #[doc = "0x1f - RES3"]
    pub sernum15: SERNUM15,
}
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
#[doc = "SERNUM0 (r) register accessor: an alias for `Reg<SERNUM0_SPEC>`"]
pub type SERNUM0 = crate::Reg<sernum0::SERNUM0_SPEC>;
#[doc = "LOTNUM0"]
pub mod sernum0;
#[doc = "SERNUM1 (r) register accessor: an alias for `Reg<SERNUM1_SPEC>`"]
pub type SERNUM1 = crate::Reg<sernum1::SERNUM1_SPEC>;
#[doc = "LOTNUM1"]
pub mod sernum1;
#[doc = "SERNUM10 (r) register accessor: an alias for `Reg<SERNUM10_SPEC>`"]
pub type SERNUM10 = crate::Reg<sernum10::SERNUM10_SPEC>;
#[doc = "YPOS0"]
pub mod sernum10;
#[doc = "SERNUM11 (r) register accessor: an alias for `Reg<SERNUM11_SPEC>`"]
pub type SERNUM11 = crate::Reg<sernum11::SERNUM11_SPEC>;
#[doc = "YPOS1"]
pub mod sernum11;
#[doc = "SERNUM12 (r) register accessor: an alias for `Reg<SERNUM12_SPEC>`"]
pub type SERNUM12 = crate::Reg<sernum12::SERNUM12_SPEC>;
#[doc = "RES0"]
pub mod sernum12;
#[doc = "SERNUM13 (r) register accessor: an alias for `Reg<SERNUM13_SPEC>`"]
pub type SERNUM13 = crate::Reg<sernum13::SERNUM13_SPEC>;
#[doc = "RES1"]
pub mod sernum13;
#[doc = "SERNUM14 (r) register accessor: an alias for `Reg<SERNUM14_SPEC>`"]
pub type SERNUM14 = crate::Reg<sernum14::SERNUM14_SPEC>;
#[doc = "RES2"]
pub mod sernum14;
#[doc = "SERNUM15 (r) register accessor: an alias for `Reg<SERNUM15_SPEC>`"]
pub type SERNUM15 = crate::Reg<sernum15::SERNUM15_SPEC>;
#[doc = "RES3"]
pub mod sernum15;
#[doc = "SERNUM2 (r) register accessor: an alias for `Reg<SERNUM2_SPEC>`"]
pub type SERNUM2 = crate::Reg<sernum2::SERNUM2_SPEC>;
#[doc = "LOTNUM2"]
pub mod sernum2;
#[doc = "SERNUM3 (r) register accessor: an alias for `Reg<SERNUM3_SPEC>`"]
pub type SERNUM3 = crate::Reg<sernum3::SERNUM3_SPEC>;
#[doc = "LOTNUM3"]
pub mod sernum3;
#[doc = "SERNUM4 (r) register accessor: an alias for `Reg<SERNUM4_SPEC>`"]
pub type SERNUM4 = crate::Reg<sernum4::SERNUM4_SPEC>;
#[doc = "LOTNUM4"]
pub mod sernum4;
#[doc = "SERNUM5 (r) register accessor: an alias for `Reg<SERNUM5_SPEC>`"]
pub type SERNUM5 = crate::Reg<sernum5::SERNUM5_SPEC>;
#[doc = "LOTNUM5"]
pub mod sernum5;
#[doc = "SERNUM6 (r) register accessor: an alias for `Reg<SERNUM6_SPEC>`"]
pub type SERNUM6 = crate::Reg<sernum6::SERNUM6_SPEC>;
#[doc = "RANDOM"]
pub mod sernum6;
#[doc = "SERNUM7 (r) register accessor: an alias for `Reg<SERNUM7_SPEC>`"]
pub type SERNUM7 = crate::Reg<sernum7::SERNUM7_SPEC>;
#[doc = "SCRIBE"]
pub mod sernum7;
#[doc = "SERNUM8 (r) register accessor: an alias for `Reg<SERNUM8_SPEC>`"]
pub type SERNUM8 = crate::Reg<sernum8::SERNUM8_SPEC>;
#[doc = "XPOS0"]
pub mod sernum8;
#[doc = "SERNUM9 (r) register accessor: an alias for `Reg<SERNUM9_SPEC>`"]
pub type SERNUM9 = crate::Reg<sernum9::SERNUM9_SPEC>;
#[doc = "XPOS1"]
pub mod sernum9;
#[doc = "TEMPSENSE0 (r) register accessor: an alias for `Reg<TEMPSENSE0_SPEC>`"]
pub type TEMPSENSE0 = crate::Reg<tempsense0::TEMPSENSE0_SPEC>;
#[doc = "Temperature Calibration 0"]
pub mod tempsense0;
#[doc = "TEMPSENSE1 (r) register accessor: an alias for `Reg<TEMPSENSE1_SPEC>`"]
pub type TEMPSENSE1 = crate::Reg<tempsense1::TEMPSENSE1_SPEC>;
#[doc = "Temperature Calibration 1"]
pub mod tempsense1;
