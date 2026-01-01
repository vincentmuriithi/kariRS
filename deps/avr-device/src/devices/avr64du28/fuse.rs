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
    #[doc = "0x07 - Code Section Size"]
    pub codesize: CODESIZE,
    #[doc = "0x08 - Boot Section Size"]
    pub bootsize: BOOTSIZE,
    _reserved7: [u8; 0x01],
    #[doc = "0x0a - Programming and Debugging Interface Configuration"]
    pub pdicfg: PDICFG,
}
#[doc = "BODCFG (rw) register accessor: an alias for `Reg<BODCFG_SPEC>`"]
pub type BODCFG = crate::Reg<bodcfg::BODCFG_SPEC>;
#[doc = "BOD Configuration"]
pub mod bodcfg;
#[doc = "BOOTSIZE (rw) register accessor: an alias for `Reg<BOOTSIZE_SPEC>`"]
pub type BOOTSIZE = crate::Reg<bootsize::BOOTSIZE_SPEC>;
#[doc = "Boot Section Size"]
pub mod bootsize;
#[doc = "CODESIZE (rw) register accessor: an alias for `Reg<CODESIZE_SPEC>`"]
pub type CODESIZE = crate::Reg<codesize::CODESIZE_SPEC>;
#[doc = "Code Section Size"]
pub mod codesize;
#[doc = "OSCCFG (rw) register accessor: an alias for `Reg<OSCCFG_SPEC>`"]
pub type OSCCFG = crate::Reg<osccfg::OSCCFG_SPEC>;
#[doc = "Oscillator Configuration"]
pub mod osccfg;
#[doc = "PDICFG (rw) register accessor: an alias for `Reg<PDICFG_SPEC>`"]
pub type PDICFG = crate::Reg<pdicfg::PDICFG_SPEC>;
#[doc = "Programming and Debugging Interface Configuration"]
pub mod pdicfg;
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
