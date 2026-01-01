#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose IO Registers"]
    pub gpio: [GPIO; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - General Purpose IO Registers"]
    #[inline(always)]
    pub fn gpior0(&self) -> &GPIO {
        &self.gpio[0]
    }
    #[doc = "0x01 - General Purpose IO Registers"]
    #[inline(always)]
    pub fn gpior1(&self) -> &GPIO {
        &self.gpio[1]
    }
    #[doc = "0x02 - General Purpose IO Registers"]
    #[inline(always)]
    pub fn gpior2(&self) -> &GPIO {
        &self.gpio[2]
    }
    #[doc = "0x03 - General Purpose IO Registers"]
    #[inline(always)]
    pub fn gpior3(&self) -> &GPIO {
        &self.gpio[3]
    }
}
#[doc = "GPIO (rw) register accessor: an alias for `Reg<GPIO_SPEC>`"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "General Purpose IO Registers"]
pub mod gpio;
