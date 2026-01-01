#[doc = "Register `BODCFG` reader"]
pub struct R(crate::R<BODCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BODCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BODCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BODCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLEEP` reader - BOD Operation in Sleep Mode"]
pub type SLEEP_R = crate::FieldReader<u8, SLEEP_A>;
#[doc = "BOD Operation in Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLEEP_A {
    #[doc = "0: Disabled"]
    DIS = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "2: Sampled"]
    SAMPLED = 2,
}
impl From<SLEEP_A> for u8 {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as _
    }
}
impl SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLEEP_A> {
        match self.bits {
            0 => Some(SLEEP_A::DIS),
            1 => Some(SLEEP_A::ENABLED),
            2 => Some(SLEEP_A::SAMPLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SLEEP_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEP_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `SAMPLED`"]
    #[inline(always)]
    pub fn is_sampled(&self) -> bool {
        *self == SLEEP_A::SAMPLED
    }
}
#[doc = "Field `ACTIVE` reader - BOD Operation in Active Mode"]
pub type ACTIVE_R = crate::FieldReader<u8, ACTIVE_A>;
#[doc = "BOD Operation in Active Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTIVE_A {
    #[doc = "0: Disabled"]
    DIS = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "2: Sampled"]
    SAMPLED = 2,
    #[doc = "3: Enabled with wake-up halted until BOD is ready"]
    ENWAKE = 3,
}
impl From<ACTIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as _
    }
}
impl ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE_A {
        match self.bits {
            0 => ACTIVE_A::DIS,
            1 => ACTIVE_A::ENABLED,
            2 => ACTIVE_A::SAMPLED,
            3 => ACTIVE_A::ENWAKE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ACTIVE_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACTIVE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `SAMPLED`"]
    #[inline(always)]
    pub fn is_sampled(&self) -> bool {
        *self == ACTIVE_A::SAMPLED
    }
    #[doc = "Checks if the value of the field is `ENWAKE`"]
    #[inline(always)]
    pub fn is_enwake(&self) -> bool {
        *self == ACTIVE_A::ENWAKE
    }
}
#[doc = "Field `SAMPFREQ` reader - BOD Sample Frequency"]
pub type SAMPFREQ_R = crate::BitReader<SAMPFREQ_A>;
#[doc = "BOD Sample Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMPFREQ_A {
    #[doc = "0: 1kHz sampling frequency"]
    _1KHZ = 0,
    #[doc = "1: 125Hz sampling frequency"]
    _125HZ = 1,
}
impl From<SAMPFREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPFREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SAMPFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPFREQ_A {
        match self.bits {
            false => SAMPFREQ_A::_1KHZ,
            true => SAMPFREQ_A::_125HZ,
        }
    }
    #[doc = "Checks if the value of the field is `_1KHZ`"]
    #[inline(always)]
    pub fn is_1khz(&self) -> bool {
        *self == SAMPFREQ_A::_1KHZ
    }
    #[doc = "Checks if the value of the field is `_125HZ`"]
    #[inline(always)]
    pub fn is_125hz(&self) -> bool {
        *self == SAMPFREQ_A::_125HZ
    }
}
#[doc = "Field `LVL` reader - BOD Level"]
pub type LVL_R = crate::FieldReader<u8, LVL_A>;
#[doc = "BOD Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVL_A {
    #[doc = "0: 1.8 V"]
    BODLEVEL0 = 0,
    #[doc = "1: 2.1 V"]
    BODLEVEL1 = 1,
    #[doc = "2: 2.6 V"]
    BODLEVEL2 = 2,
    #[doc = "3: 2.9 V"]
    BODLEVEL3 = 3,
    #[doc = "4: 3.3 V"]
    BODLEVEL4 = 4,
    #[doc = "5: 3.7 V"]
    BODLEVEL5 = 5,
    #[doc = "6: 4.0 V"]
    BODLEVEL6 = 6,
    #[doc = "7: 4.2 V"]
    BODLEVEL7 = 7,
}
impl From<LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVL_A) -> Self {
        variant as _
    }
}
impl LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVL_A {
        match self.bits {
            0 => LVL_A::BODLEVEL0,
            1 => LVL_A::BODLEVEL1,
            2 => LVL_A::BODLEVEL2,
            3 => LVL_A::BODLEVEL3,
            4 => LVL_A::BODLEVEL4,
            5 => LVL_A::BODLEVEL5,
            6 => LVL_A::BODLEVEL6,
            7 => LVL_A::BODLEVEL7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BODLEVEL0`"]
    #[inline(always)]
    pub fn is_bodlevel0(&self) -> bool {
        *self == LVL_A::BODLEVEL0
    }
    #[doc = "Checks if the value of the field is `BODLEVEL1`"]
    #[inline(always)]
    pub fn is_bodlevel1(&self) -> bool {
        *self == LVL_A::BODLEVEL1
    }
    #[doc = "Checks if the value of the field is `BODLEVEL2`"]
    #[inline(always)]
    pub fn is_bodlevel2(&self) -> bool {
        *self == LVL_A::BODLEVEL2
    }
    #[doc = "Checks if the value of the field is `BODLEVEL3`"]
    #[inline(always)]
    pub fn is_bodlevel3(&self) -> bool {
        *self == LVL_A::BODLEVEL3
    }
    #[doc = "Checks if the value of the field is `BODLEVEL4`"]
    #[inline(always)]
    pub fn is_bodlevel4(&self) -> bool {
        *self == LVL_A::BODLEVEL4
    }
    #[doc = "Checks if the value of the field is `BODLEVEL5`"]
    #[inline(always)]
    pub fn is_bodlevel5(&self) -> bool {
        *self == LVL_A::BODLEVEL5
    }
    #[doc = "Checks if the value of the field is `BODLEVEL6`"]
    #[inline(always)]
    pub fn is_bodlevel6(&self) -> bool {
        *self == LVL_A::BODLEVEL6
    }
    #[doc = "Checks if the value of the field is `BODLEVEL7`"]
    #[inline(always)]
    pub fn is_bodlevel7(&self) -> bool {
        *self == LVL_A::BODLEVEL7
    }
}
impl R {
    #[doc = "Bits 0:1 - BOD Operation in Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - BOD Operation in Active Mode"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - BOD Sample Frequency"]
    #[inline(always)]
    pub fn sampfreq(&self) -> SAMPFREQ_R {
        SAMPFREQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - BOD Level"]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new((self.bits >> 5) & 7)
    }
}
#[doc = "BOD Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodcfg](index.html) module"]
pub struct BODCFG_SPEC;
impl crate::RegisterSpec for BODCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bodcfg::R](R) reader structure"]
impl crate::Readable for BODCFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BODCFG to value 0"]
impl crate::Resettable for BODCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
