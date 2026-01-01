#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device IO Bytes"]
    pub deviceid: [DEVICEID; 3],
    #[doc = "0x03..0x0d - Serial Number Bytes"]
    pub sernum: [SERNUM; 10],
    _reserved2: [u8; 0x13],
    #[doc = "0x20 - Temperature Sensor Calibration: Gain/Slope"]
    pub tempsense0: TEMPSENSE0,
    #[doc = "0x21 - Temperature Sensor Calibration: Offset"]
    pub tempsense1: TEMPSENSE1,
    #[doc = "0x22 - OSC16 error at 3V"]
    pub osc16err3v: OSC16ERR3V,
    #[doc = "0x23 - OSC16 error at 5V"]
    pub osc16err5v: OSC16ERR5V,
    #[doc = "0x24 - OSC20 error at 3V"]
    pub osc20err3v: OSC20ERR3V,
    #[doc = "0x25 - OSC20 error at 5V"]
    pub osc20err5v: OSC20ERR5V,
}
#[doc = "DEVICEID (r) register accessor: an alias for `Reg<DEVICEID_SPEC>`"]
pub type DEVICEID = crate::Reg<deviceid::DEVICEID_SPEC>;
#[doc = "Device IO Bytes"]
pub mod deviceid;
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
#[doc = "SERNUM (r) register accessor: an alias for `Reg<SERNUM_SPEC>`"]
pub type SERNUM = crate::Reg<sernum::SERNUM_SPEC>;
#[doc = "Serial Number Bytes"]
pub mod sernum;
#[doc = "TEMPSENSE0 (r) register accessor: an alias for `Reg<TEMPSENSE0_SPEC>`"]
pub type TEMPSENSE0 = crate::Reg<tempsense0::TEMPSENSE0_SPEC>;
#[doc = "Temperature Sensor Calibration: Gain/Slope"]
pub mod tempsense0;
#[doc = "TEMPSENSE1 (r) register accessor: an alias for `Reg<TEMPSENSE1_SPEC>`"]
pub type TEMPSENSE1 = crate::Reg<tempsense1::TEMPSENSE1_SPEC>;
#[doc = "Temperature Sensor Calibration: Offset"]
pub mod tempsense1;
