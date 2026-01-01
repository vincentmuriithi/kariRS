#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - External Reset, Power-on Reset and Watchdog Reset"]
    RESET = 0,
    #[doc = "1 - External Interrupt 0"]
    INT0 = 1,
    #[doc = "2 - External Interrupt Request 0"]
    IO_PINS = 2,
    #[doc = "3 - Timer/Counter1 Compare Match 1A"]
    TIMER1_CMPA = 3,
    #[doc = "4 - Timer/Counter1 Compare Match 1B"]
    TIMER1_CMPB = 4,
    #[doc = "5 - Timer/Counter1 Overflow"]
    TIMER1_OVF1 = 5,
    #[doc = "6 - Timer/Counter0 Overflow"]
    TIMER0_OVF0 = 6,
    #[doc = "7 - USI Start"]
    USI_STRT = 7,
    #[doc = "8 - USI Overflow"]
    USI_OVF = 8,
    #[doc = "9 - EEPROM Ready"]
    EE_RDY = 9,
    #[doc = "10 - Analog Comparator"]
    ANA_COMP = 10,
    #[doc = "11 - ADC Conversion Complete"]
    ADC = 11,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::RESET),
            1 => Ok(Interrupt::INT0),
            2 => Ok(Interrupt::IO_PINS),
            3 => Ok(Interrupt::TIMER1_CMPA),
            4 => Ok(Interrupt::TIMER1_CMPB),
            5 => Ok(Interrupt::TIMER1_OVF1),
            6 => Ok(Interrupt::TIMER0_OVF0),
            7 => Ok(Interrupt::USI_STRT),
            8 => Ok(Interrupt::USI_OVF),
            9 => Ok(Interrupt::EE_RDY),
            10 => Ok(Interrupt::ANA_COMP),
            11 => Ok(Interrupt::ADC),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
