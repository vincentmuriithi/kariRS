#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Configuration"]
    pub wdtcfg: WDTCFG,
    #[doc = "0x01 - BOD Configuration"]
    pub bodcfg: BODCFG,
    #[doc = "0x02 - Oscillator Configuration"]
    pub osccfg: OSCCFG,
    _reserved3: [u8; 0x02],
    #[doc = "0x05 - System Configuration 0"]
    pub syscfg0: SYSCFG0,
    #[doc = "0x06 - System Configuration 1"]
    pub syscfg1: SYSCFG1,
    #[doc = "0x07 - Application Code Section End"]
    pub append: APPEND,
    #[doc = "0x08 - Boot Section End"]
    pub bootend: BOOTEND,
}
#[doc = "APPEND (rw) register accessor: an alias for `Reg<APPEND_SPEC>`"]
pub type APPEND = crate::Reg<append::APPEND_SPEC>;
#[doc = "Application Code Section End"]
pub mod append;
#[doc = "BODCFG (rw) register accessor: an alias for `Reg<BODCFG_SPEC>`"]
pub type BODCFG = crate::Reg<bodcfg::BODCFG_SPEC>;
#[doc = "BOD Configuration"]
pub mod bodcfg;
#[doc = "BOOTEND (rw) register accessor: an alias for `Reg<BOOTEND_SPEC>`"]
pub type BOOTEND = crate::Reg<bootend::BOOTEND_SPEC>;
#[doc = "Boot Section End"]
pub mod bootend;
#[doc = "OSCCFG (rw) register accessor: an alias for `Reg<OSCCFG_SPEC>`"]
pub type OSCCFG = crate::Reg<osccfg::OSCCFG_SPEC>;
#[doc = "Oscillator Configuration"]
pub mod osccfg;
#[doc = "SYSCFG0 (rw) register accessor: an alias for `Reg<SYSCFG0_SPEC>`"]
pub type SYSCFG0 = crate::Reg<syscfg0::SYSCFG0_SPEC>;
#[doc = "System Configuration 0"]
pub mod syscfg0;
#[doc = "SYSCFG1 (rw) register accessor: an alias for `Reg<SYSCFG1_SPEC>`"]
pub type SYSCFG1 = crate::Reg<syscfg1::SYSCFG1_SPEC>;
#[doc = "System Configuration 1"]
pub mod syscfg1;
#[doc = "WDTCFG (rw) register accessor: an alias for `Reg<WDTCFG_SPEC>`"]
pub type WDTCFG = crate::Reg<wdtcfg::WDTCFG_SPEC>;
#[doc = "Watchdog Configuration"]
pub mod wdtcfg;
