#[doc = "Register `HIGH` reader"]
pub struct R(crate::R<HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BODEN` reader - Brown-out detection enabled"]
pub type BODEN_R = crate::BitReader<bool>;
#[doc = "Field `BODLEVEL` reader - Brownout detector trigger level"]
pub type BODLEVEL_R = crate::BitReader<BODLEVEL_A>;
#[doc = "Brownout detector trigger level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BODLEVEL_A {
    #[doc = "0: Brown-out detection at VCC=4.0 V"]
    _4V0 = 0,
    #[doc = "1: Brown-out detection at VCC=2.7 V"]
    _2V7 = 1,
}
impl From<BODLEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: BODLEVEL_A) -> Self {
        variant as u8 != 0
    }
}
impl BODLEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODLEVEL_A {
        match self.bits {
            false => BODLEVEL_A::_4V0,
            true => BODLEVEL_A::_2V7,
        }
    }
    #[doc = "Checks if the value of the field is `_4V0`"]
    #[inline(always)]
    pub fn is_4v0(&self) -> bool {
        *self == BODLEVEL_A::_4V0
    }
    #[doc = "Checks if the value of the field is `_2V7`"]
    #[inline(always)]
    pub fn is_2v7(&self) -> bool {
        *self == BODLEVEL_A::_2V7
    }
}
#[doc = "Field `EESAVE` reader - Preserve EEPROM through the Chip Erase cycle"]
pub type EESAVE_R = crate::BitReader<bool>;
#[doc = "Field `SPIEN` reader - Serial program downloading (SPI) enabled"]
pub type SPIEN_R = crate::BitReader<bool>;
#[doc = "Field `RSTDISBL` reader - Reset Disabled (Enable PB7 as i/o pin)"]
pub type RSTDISBL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Brown-out detection enabled"]
    #[inline(always)]
    pub fn boden(&self) -> BODEN_R {
        BODEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Brownout detector trigger level"]
    #[inline(always)]
    pub fn bodlevel(&self) -> BODLEVEL_R {
        BODLEVEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Preserve EEPROM through the Chip Erase cycle"]
    #[inline(always)]
    pub fn eesave(&self) -> EESAVE_R {
        EESAVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Serial program downloading (SPI) enabled"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset Disabled (Enable PB7 as i/o pin)"]
    #[inline(always)]
    pub fn rstdisbl(&self) -> RSTDISBL_R {
        RSTDISBL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [high](index.html) module"]
pub struct HIGH_SPEC;
impl crate::RegisterSpec for HIGH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [high::R](R) reader structure"]
impl crate::Readable for HIGH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HIGH to value 0"]
impl crate::Resettable for HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
