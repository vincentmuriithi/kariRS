#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - User Row Byte 0"]
    pub userrow0: USERROW0,
    #[doc = "0x01 - User Row Byte 1"]
    pub userrow1: USERROW1,
    #[doc = "0x02 - User Row Byte 2"]
    pub userrow2: USERROW2,
    #[doc = "0x03 - User Row Byte 3"]
    pub userrow3: USERROW3,
    #[doc = "0x04 - User Row Byte 4"]
    pub userrow4: USERROW4,
    #[doc = "0x05 - User Row Byte 5"]
    pub userrow5: USERROW5,
    #[doc = "0x06 - User Row Byte 6"]
    pub userrow6: USERROW6,
    #[doc = "0x07 - User Row Byte 7"]
    pub userrow7: USERROW7,
    #[doc = "0x08 - User Row Byte 8"]
    pub userrow8: USERROW8,
    #[doc = "0x09 - User Row Byte 9"]
    pub userrow9: USERROW9,
    #[doc = "0x0a - User Row Byte 10"]
    pub userrow10: USERROW10,
    #[doc = "0x0b - User Row Byte 11"]
    pub userrow11: USERROW11,
    #[doc = "0x0c - User Row Byte 12"]
    pub userrow12: USERROW12,
    #[doc = "0x0d - User Row Byte 13"]
    pub userrow13: USERROW13,
    #[doc = "0x0e - User Row Byte 14"]
    pub userrow14: USERROW14,
    #[doc = "0x0f - User Row Byte 15"]
    pub userrow15: USERROW15,
    #[doc = "0x10 - User Row Byte 16"]
    pub userrow16: USERROW16,
    #[doc = "0x11 - User Row Byte 17"]
    pub userrow17: USERROW17,
    #[doc = "0x12 - User Row Byte 18"]
    pub userrow18: USERROW18,
    #[doc = "0x13 - User Row Byte 19"]
    pub userrow19: USERROW19,
    #[doc = "0x14 - User Row Byte 20"]
    pub userrow20: USERROW20,
    #[doc = "0x15 - User Row Byte 21"]
    pub userrow21: USERROW21,
    #[doc = "0x16 - User Row Byte 22"]
    pub userrow22: USERROW22,
    #[doc = "0x17 - User Row Byte 23"]
    pub userrow23: USERROW23,
    #[doc = "0x18 - User Row Byte 24"]
    pub userrow24: USERROW24,
    #[doc = "0x19 - User Row Byte 25"]
    pub userrow25: USERROW25,
    #[doc = "0x1a - User Row Byte 26"]
    pub userrow26: USERROW26,
    #[doc = "0x1b - User Row Byte 27"]
    pub userrow27: USERROW27,
    #[doc = "0x1c - User Row Byte 28"]
    pub userrow28: USERROW28,
    #[doc = "0x1d - User Row Byte 29"]
    pub userrow29: USERROW29,
    #[doc = "0x1e - User Row Byte 30"]
    pub userrow30: USERROW30,
    #[doc = "0x1f - User Row Byte 31"]
    pub userrow31: USERROW31,
    #[doc = "0x20 - User Row Byte 32"]
    pub userrow32: USERROW32,
    #[doc = "0x21 - User Row Byte 33"]
    pub userrow33: USERROW33,
    #[doc = "0x22 - User Row Byte 34"]
    pub userrow34: USERROW34,
    #[doc = "0x23 - User Row Byte 35"]
    pub userrow35: USERROW35,
    #[doc = "0x24 - User Row Byte 36"]
    pub userrow36: USERROW36,
    #[doc = "0x25 - User Row Byte 37"]
    pub userrow37: USERROW37,
    #[doc = "0x26 - User Row Byte 38"]
    pub userrow38: USERROW38,
    #[doc = "0x27 - User Row Byte 39"]
    pub userrow39: USERROW39,
    #[doc = "0x28 - User Row Byte 40"]
    pub userrow40: USERROW40,
    #[doc = "0x29 - User Row Byte 41"]
    pub userrow41: USERROW41,
    #[doc = "0x2a - User Row Byte 42"]
    pub userrow42: USERROW42,
    #[doc = "0x2b - User Row Byte 43"]
    pub userrow43: USERROW43,
    #[doc = "0x2c - User Row Byte 44"]
    pub userrow44: USERROW44,
    #[doc = "0x2d - User Row Byte 45"]
    pub userrow45: USERROW45,
    #[doc = "0x2e - User Row Byte 46"]
    pub userrow46: USERROW46,
    #[doc = "0x2f - User Row Byte 47"]
    pub userrow47: USERROW47,
    #[doc = "0x30 - User Row Byte 48"]
    pub userrow48: USERROW48,
    #[doc = "0x31 - User Row Byte 49"]
    pub userrow49: USERROW49,
    #[doc = "0x32 - User Row Byte 50"]
    pub userrow50: USERROW50,
    #[doc = "0x33 - User Row Byte 51"]
    pub userrow51: USERROW51,
    #[doc = "0x34 - User Row Byte 52"]
    pub userrow52: USERROW52,
    #[doc = "0x35 - User Row Byte 53"]
    pub userrow53: USERROW53,
    #[doc = "0x36 - User Row Byte 54"]
    pub userrow54: USERROW54,
    #[doc = "0x37 - User Row Byte 55"]
    pub userrow55: USERROW55,
    #[doc = "0x38 - User Row Byte 56"]
    pub userrow56: USERROW56,
    #[doc = "0x39 - User Row Byte 57"]
    pub userrow57: USERROW57,
    #[doc = "0x3a - User Row Byte 58"]
    pub userrow58: USERROW58,
    #[doc = "0x3b - User Row Byte 59"]
    pub userrow59: USERROW59,
    #[doc = "0x3c - User Row Byte 60"]
    pub userrow60: USERROW60,
    #[doc = "0x3d - User Row Byte 61"]
    pub userrow61: USERROW61,
    #[doc = "0x3e - User Row Byte 62"]
    pub userrow62: USERROW62,
    #[doc = "0x3f - User Row Byte 63"]
    pub userrow63: USERROW63,
}
#[doc = "USERROW0 (rw) register accessor: an alias for `Reg<USERROW0_SPEC>`"]
pub type USERROW0 = crate::Reg<userrow0::USERROW0_SPEC>;
#[doc = "User Row Byte 0"]
pub mod userrow0;
#[doc = "USERROW1 (rw) register accessor: an alias for `Reg<USERROW1_SPEC>`"]
pub type USERROW1 = crate::Reg<userrow1::USERROW1_SPEC>;
#[doc = "User Row Byte 1"]
pub mod userrow1;
#[doc = "USERROW10 (rw) register accessor: an alias for `Reg<USERROW10_SPEC>`"]
pub type USERROW10 = crate::Reg<userrow10::USERROW10_SPEC>;
#[doc = "User Row Byte 10"]
pub mod userrow10;
#[doc = "USERROW11 (rw) register accessor: an alias for `Reg<USERROW11_SPEC>`"]
pub type USERROW11 = crate::Reg<userrow11::USERROW11_SPEC>;
#[doc = "User Row Byte 11"]
pub mod userrow11;
#[doc = "USERROW12 (rw) register accessor: an alias for `Reg<USERROW12_SPEC>`"]
pub type USERROW12 = crate::Reg<userrow12::USERROW12_SPEC>;
#[doc = "User Row Byte 12"]
pub mod userrow12;
#[doc = "USERROW13 (rw) register accessor: an alias for `Reg<USERROW13_SPEC>`"]
pub type USERROW13 = crate::Reg<userrow13::USERROW13_SPEC>;
#[doc = "User Row Byte 13"]
pub mod userrow13;
#[doc = "USERROW14 (rw) register accessor: an alias for `Reg<USERROW14_SPEC>`"]
pub type USERROW14 = crate::Reg<userrow14::USERROW14_SPEC>;
#[doc = "User Row Byte 14"]
pub mod userrow14;
#[doc = "USERROW15 (rw) register accessor: an alias for `Reg<USERROW15_SPEC>`"]
pub type USERROW15 = crate::Reg<userrow15::USERROW15_SPEC>;
#[doc = "User Row Byte 15"]
pub mod userrow15;
#[doc = "USERROW16 (rw) register accessor: an alias for `Reg<USERROW16_SPEC>`"]
pub type USERROW16 = crate::Reg<userrow16::USERROW16_SPEC>;
#[doc = "User Row Byte 16"]
pub mod userrow16;
#[doc = "USERROW17 (rw) register accessor: an alias for `Reg<USERROW17_SPEC>`"]
pub type USERROW17 = crate::Reg<userrow17::USERROW17_SPEC>;
#[doc = "User Row Byte 17"]
pub mod userrow17;
#[doc = "USERROW18 (rw) register accessor: an alias for `Reg<USERROW18_SPEC>`"]
pub type USERROW18 = crate::Reg<userrow18::USERROW18_SPEC>;
#[doc = "User Row Byte 18"]
pub mod userrow18;
#[doc = "USERROW19 (rw) register accessor: an alias for `Reg<USERROW19_SPEC>`"]
pub type USERROW19 = crate::Reg<userrow19::USERROW19_SPEC>;
#[doc = "User Row Byte 19"]
pub mod userrow19;
#[doc = "USERROW2 (rw) register accessor: an alias for `Reg<USERROW2_SPEC>`"]
pub type USERROW2 = crate::Reg<userrow2::USERROW2_SPEC>;
#[doc = "User Row Byte 2"]
pub mod userrow2;
#[doc = "USERROW20 (rw) register accessor: an alias for `Reg<USERROW20_SPEC>`"]
pub type USERROW20 = crate::Reg<userrow20::USERROW20_SPEC>;
#[doc = "User Row Byte 20"]
pub mod userrow20;
#[doc = "USERROW21 (rw) register accessor: an alias for `Reg<USERROW21_SPEC>`"]
pub type USERROW21 = crate::Reg<userrow21::USERROW21_SPEC>;
#[doc = "User Row Byte 21"]
pub mod userrow21;
#[doc = "USERROW22 (rw) register accessor: an alias for `Reg<USERROW22_SPEC>`"]
pub type USERROW22 = crate::Reg<userrow22::USERROW22_SPEC>;
#[doc = "User Row Byte 22"]
pub mod userrow22;
#[doc = "USERROW23 (rw) register accessor: an alias for `Reg<USERROW23_SPEC>`"]
pub type USERROW23 = crate::Reg<userrow23::USERROW23_SPEC>;
#[doc = "User Row Byte 23"]
pub mod userrow23;
#[doc = "USERROW24 (rw) register accessor: an alias for `Reg<USERROW24_SPEC>`"]
pub type USERROW24 = crate::Reg<userrow24::USERROW24_SPEC>;
#[doc = "User Row Byte 24"]
pub mod userrow24;
#[doc = "USERROW25 (rw) register accessor: an alias for `Reg<USERROW25_SPEC>`"]
pub type USERROW25 = crate::Reg<userrow25::USERROW25_SPEC>;
#[doc = "User Row Byte 25"]
pub mod userrow25;
#[doc = "USERROW26 (rw) register accessor: an alias for `Reg<USERROW26_SPEC>`"]
pub type USERROW26 = crate::Reg<userrow26::USERROW26_SPEC>;
#[doc = "User Row Byte 26"]
pub mod userrow26;
#[doc = "USERROW27 (rw) register accessor: an alias for `Reg<USERROW27_SPEC>`"]
pub type USERROW27 = crate::Reg<userrow27::USERROW27_SPEC>;
#[doc = "User Row Byte 27"]
pub mod userrow27;
#[doc = "USERROW28 (rw) register accessor: an alias for `Reg<USERROW28_SPEC>`"]
pub type USERROW28 = crate::Reg<userrow28::USERROW28_SPEC>;
#[doc = "User Row Byte 28"]
pub mod userrow28;
#[doc = "USERROW29 (rw) register accessor: an alias for `Reg<USERROW29_SPEC>`"]
pub type USERROW29 = crate::Reg<userrow29::USERROW29_SPEC>;
#[doc = "User Row Byte 29"]
pub mod userrow29;
#[doc = "USERROW3 (rw) register accessor: an alias for `Reg<USERROW3_SPEC>`"]
pub type USERROW3 = crate::Reg<userrow3::USERROW3_SPEC>;
#[doc = "User Row Byte 3"]
pub mod userrow3;
#[doc = "USERROW30 (rw) register accessor: an alias for `Reg<USERROW30_SPEC>`"]
pub type USERROW30 = crate::Reg<userrow30::USERROW30_SPEC>;
#[doc = "User Row Byte 30"]
pub mod userrow30;
#[doc = "USERROW31 (rw) register accessor: an alias for `Reg<USERROW31_SPEC>`"]
pub type USERROW31 = crate::Reg<userrow31::USERROW31_SPEC>;
#[doc = "User Row Byte 31"]
pub mod userrow31;
#[doc = "USERROW32 (rw) register accessor: an alias for `Reg<USERROW32_SPEC>`"]
pub type USERROW32 = crate::Reg<userrow32::USERROW32_SPEC>;
#[doc = "User Row Byte 32"]
pub mod userrow32;
#[doc = "USERROW33 (rw) register accessor: an alias for `Reg<USERROW33_SPEC>`"]
pub type USERROW33 = crate::Reg<userrow33::USERROW33_SPEC>;
#[doc = "User Row Byte 33"]
pub mod userrow33;
#[doc = "USERROW34 (rw) register accessor: an alias for `Reg<USERROW34_SPEC>`"]
pub type USERROW34 = crate::Reg<userrow34::USERROW34_SPEC>;
#[doc = "User Row Byte 34"]
pub mod userrow34;
#[doc = "USERROW35 (rw) register accessor: an alias for `Reg<USERROW35_SPEC>`"]
pub type USERROW35 = crate::Reg<userrow35::USERROW35_SPEC>;
#[doc = "User Row Byte 35"]
pub mod userrow35;
#[doc = "USERROW36 (rw) register accessor: an alias for `Reg<USERROW36_SPEC>`"]
pub type USERROW36 = crate::Reg<userrow36::USERROW36_SPEC>;
#[doc = "User Row Byte 36"]
pub mod userrow36;
#[doc = "USERROW37 (rw) register accessor: an alias for `Reg<USERROW37_SPEC>`"]
pub type USERROW37 = crate::Reg<userrow37::USERROW37_SPEC>;
#[doc = "User Row Byte 37"]
pub mod userrow37;
#[doc = "USERROW38 (rw) register accessor: an alias for `Reg<USERROW38_SPEC>`"]
pub type USERROW38 = crate::Reg<userrow38::USERROW38_SPEC>;
#[doc = "User Row Byte 38"]
pub mod userrow38;
#[doc = "USERROW39 (rw) register accessor: an alias for `Reg<USERROW39_SPEC>`"]
pub type USERROW39 = crate::Reg<userrow39::USERROW39_SPEC>;
#[doc = "User Row Byte 39"]
pub mod userrow39;
#[doc = "USERROW4 (rw) register accessor: an alias for `Reg<USERROW4_SPEC>`"]
pub type USERROW4 = crate::Reg<userrow4::USERROW4_SPEC>;
#[doc = "User Row Byte 4"]
pub mod userrow4;
#[doc = "USERROW40 (rw) register accessor: an alias for `Reg<USERROW40_SPEC>`"]
pub type USERROW40 = crate::Reg<userrow40::USERROW40_SPEC>;
#[doc = "User Row Byte 40"]
pub mod userrow40;
#[doc = "USERROW41 (rw) register accessor: an alias for `Reg<USERROW41_SPEC>`"]
pub type USERROW41 = crate::Reg<userrow41::USERROW41_SPEC>;
#[doc = "User Row Byte 41"]
pub mod userrow41;
#[doc = "USERROW42 (rw) register accessor: an alias for `Reg<USERROW42_SPEC>`"]
pub type USERROW42 = crate::Reg<userrow42::USERROW42_SPEC>;
#[doc = "User Row Byte 42"]
pub mod userrow42;
#[doc = "USERROW43 (rw) register accessor: an alias for `Reg<USERROW43_SPEC>`"]
pub type USERROW43 = crate::Reg<userrow43::USERROW43_SPEC>;
#[doc = "User Row Byte 43"]
pub mod userrow43;
#[doc = "USERROW44 (rw) register accessor: an alias for `Reg<USERROW44_SPEC>`"]
pub type USERROW44 = crate::Reg<userrow44::USERROW44_SPEC>;
#[doc = "User Row Byte 44"]
pub mod userrow44;
#[doc = "USERROW45 (rw) register accessor: an alias for `Reg<USERROW45_SPEC>`"]
pub type USERROW45 = crate::Reg<userrow45::USERROW45_SPEC>;
#[doc = "User Row Byte 45"]
pub mod userrow45;
#[doc = "USERROW46 (rw) register accessor: an alias for `Reg<USERROW46_SPEC>`"]
pub type USERROW46 = crate::Reg<userrow46::USERROW46_SPEC>;
#[doc = "User Row Byte 46"]
pub mod userrow46;
#[doc = "USERROW47 (rw) register accessor: an alias for `Reg<USERROW47_SPEC>`"]
pub type USERROW47 = crate::Reg<userrow47::USERROW47_SPEC>;
#[doc = "User Row Byte 47"]
pub mod userrow47;
#[doc = "USERROW48 (rw) register accessor: an alias for `Reg<USERROW48_SPEC>`"]
pub type USERROW48 = crate::Reg<userrow48::USERROW48_SPEC>;
#[doc = "User Row Byte 48"]
pub mod userrow48;
#[doc = "USERROW49 (rw) register accessor: an alias for `Reg<USERROW49_SPEC>`"]
pub type USERROW49 = crate::Reg<userrow49::USERROW49_SPEC>;
#[doc = "User Row Byte 49"]
pub mod userrow49;
#[doc = "USERROW5 (rw) register accessor: an alias for `Reg<USERROW5_SPEC>`"]
pub type USERROW5 = crate::Reg<userrow5::USERROW5_SPEC>;
#[doc = "User Row Byte 5"]
pub mod userrow5;
#[doc = "USERROW50 (rw) register accessor: an alias for `Reg<USERROW50_SPEC>`"]
pub type USERROW50 = crate::Reg<userrow50::USERROW50_SPEC>;
#[doc = "User Row Byte 50"]
pub mod userrow50;
#[doc = "USERROW51 (rw) register accessor: an alias for `Reg<USERROW51_SPEC>`"]
pub type USERROW51 = crate::Reg<userrow51::USERROW51_SPEC>;
#[doc = "User Row Byte 51"]
pub mod userrow51;
#[doc = "USERROW52 (rw) register accessor: an alias for `Reg<USERROW52_SPEC>`"]
pub type USERROW52 = crate::Reg<userrow52::USERROW52_SPEC>;
#[doc = "User Row Byte 52"]
pub mod userrow52;
#[doc = "USERROW53 (rw) register accessor: an alias for `Reg<USERROW53_SPEC>`"]
pub type USERROW53 = crate::Reg<userrow53::USERROW53_SPEC>;
#[doc = "User Row Byte 53"]
pub mod userrow53;
#[doc = "USERROW54 (rw) register accessor: an alias for `Reg<USERROW54_SPEC>`"]
pub type USERROW54 = crate::Reg<userrow54::USERROW54_SPEC>;
#[doc = "User Row Byte 54"]
pub mod userrow54;
#[doc = "USERROW55 (rw) register accessor: an alias for `Reg<USERROW55_SPEC>`"]
pub type USERROW55 = crate::Reg<userrow55::USERROW55_SPEC>;
#[doc = "User Row Byte 55"]
pub mod userrow55;
#[doc = "USERROW56 (rw) register accessor: an alias for `Reg<USERROW56_SPEC>`"]
pub type USERROW56 = crate::Reg<userrow56::USERROW56_SPEC>;
#[doc = "User Row Byte 56"]
pub mod userrow56;
#[doc = "USERROW57 (rw) register accessor: an alias for `Reg<USERROW57_SPEC>`"]
pub type USERROW57 = crate::Reg<userrow57::USERROW57_SPEC>;
#[doc = "User Row Byte 57"]
pub mod userrow57;
#[doc = "USERROW58 (rw) register accessor: an alias for `Reg<USERROW58_SPEC>`"]
pub type USERROW58 = crate::Reg<userrow58::USERROW58_SPEC>;
#[doc = "User Row Byte 58"]
pub mod userrow58;
#[doc = "USERROW59 (rw) register accessor: an alias for `Reg<USERROW59_SPEC>`"]
pub type USERROW59 = crate::Reg<userrow59::USERROW59_SPEC>;
#[doc = "User Row Byte 59"]
pub mod userrow59;
#[doc = "USERROW6 (rw) register accessor: an alias for `Reg<USERROW6_SPEC>`"]
pub type USERROW6 = crate::Reg<userrow6::USERROW6_SPEC>;
#[doc = "User Row Byte 6"]
pub mod userrow6;
#[doc = "USERROW60 (rw) register accessor: an alias for `Reg<USERROW60_SPEC>`"]
pub type USERROW60 = crate::Reg<userrow60::USERROW60_SPEC>;
#[doc = "User Row Byte 60"]
pub mod userrow60;
#[doc = "USERROW61 (rw) register accessor: an alias for `Reg<USERROW61_SPEC>`"]
pub type USERROW61 = crate::Reg<userrow61::USERROW61_SPEC>;
#[doc = "User Row Byte 61"]
pub mod userrow61;
#[doc = "USERROW62 (rw) register accessor: an alias for `Reg<USERROW62_SPEC>`"]
pub type USERROW62 = crate::Reg<userrow62::USERROW62_SPEC>;
#[doc = "User Row Byte 62"]
pub mod userrow62;
#[doc = "USERROW63 (rw) register accessor: an alias for `Reg<USERROW63_SPEC>`"]
pub type USERROW63 = crate::Reg<userrow63::USERROW63_SPEC>;
#[doc = "User Row Byte 63"]
pub mod userrow63;
#[doc = "USERROW7 (rw) register accessor: an alias for `Reg<USERROW7_SPEC>`"]
pub type USERROW7 = crate::Reg<userrow7::USERROW7_SPEC>;
#[doc = "User Row Byte 7"]
pub mod userrow7;
#[doc = "USERROW8 (rw) register accessor: an alias for `Reg<USERROW8_SPEC>`"]
pub type USERROW8 = crate::Reg<userrow8::USERROW8_SPEC>;
#[doc = "User Row Byte 8"]
pub mod userrow8;
#[doc = "USERROW9 (rw) register accessor: an alias for `Reg<USERROW9_SPEC>`"]
pub type USERROW9 = crate::Reg<userrow9::USERROW9_SPEC>;
#[doc = "User Row Byte 9"]
pub mod userrow9;
