#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `HYSMODE` reader - Hysteresis Mode"]
pub type HYSMODE_R = crate::FieldReader<u8, HYSMODE_A>;
#[doc = "Hysteresis Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYSMODE_A {
    #[doc = "0: No hysteresis"]
    NONE = 0,
    #[doc = "1: Small hysteresis"]
    SMALL = 1,
    #[doc = "2: Medium hysteresis"]
    MEDIUM = 2,
    #[doc = "3: Large hysteresis"]
    LARGE = 3,
}
impl From<HYSMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: HYSMODE_A) -> Self {
        variant as _
    }
}
impl HYSMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYSMODE_A {
        match self.bits {
            0 => HYSMODE_A::NONE,
            1 => HYSMODE_A::SMALL,
            2 => HYSMODE_A::MEDIUM,
            3 => HYSMODE_A::LARGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == HYSMODE_A::NONE
    }
    #[doc = "Checks if the value of the field is `SMALL`"]
    #[inline(always)]
    pub fn is_small(&self) -> bool {
        *self == HYSMODE_A::SMALL
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == HYSMODE_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `LARGE`"]
    #[inline(always)]
    pub fn is_large(&self) -> bool {
        *self == HYSMODE_A::LARGE
    }
}
#[doc = "Field `HYSMODE` writer - Hysteresis Mode"]
pub type HYSMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CTRLA_SPEC, u8, HYSMODE_A, 2, O>;
impl<'a, const O: u8> HYSMODE_W<'a, O> {
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(HYSMODE_A::NONE)
    }
    #[doc = "Small hysteresis"]
    #[inline(always)]
    pub fn small(self) -> &'a mut W {
        self.variant(HYSMODE_A::SMALL)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(HYSMODE_A::MEDIUM)
    }
    #[doc = "Large hysteresis"]
    #[inline(always)]
    pub fn large(self) -> &'a mut W {
        self.variant(HYSMODE_A::LARGE)
    }
}
#[doc = "Field `POWER` reader - Power profile"]
pub type POWER_R = crate::FieldReader<u8, POWER_A>;
#[doc = "Power profile\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POWER_A {
    #[doc = "0: Power profile 0, lowest consumption and highest response time."]
    PROFILE0 = 0,
    #[doc = "1: Power profile 1"]
    PROFILE1 = 1,
    #[doc = "2: Power profile 2"]
    PROFILE2 = 2,
    #[doc = "3: Power profile 3"]
    PROFILE3 = 3,
}
impl From<POWER_A> for u8 {
    #[inline(always)]
    fn from(variant: POWER_A) -> Self {
        variant as _
    }
}
impl POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWER_A {
        match self.bits {
            0 => POWER_A::PROFILE0,
            1 => POWER_A::PROFILE1,
            2 => POWER_A::PROFILE2,
            3 => POWER_A::PROFILE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PROFILE0`"]
    #[inline(always)]
    pub fn is_profile0(&self) -> bool {
        *self == POWER_A::PROFILE0
    }
    #[doc = "Checks if the value of the field is `PROFILE1`"]
    #[inline(always)]
    pub fn is_profile1(&self) -> bool {
        *self == POWER_A::PROFILE1
    }
    #[doc = "Checks if the value of the field is `PROFILE2`"]
    #[inline(always)]
    pub fn is_profile2(&self) -> bool {
        *self == POWER_A::PROFILE2
    }
    #[doc = "Checks if the value of the field is `PROFILE3`"]
    #[inline(always)]
    pub fn is_profile3(&self) -> bool {
        *self == POWER_A::PROFILE3
    }
}
#[doc = "Field `POWER` writer - Power profile"]
pub type POWER_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CTRLA_SPEC, u8, POWER_A, 2, O>;
impl<'a, const O: u8> POWER_W<'a, O> {
    #[doc = "Power profile 0, lowest consumption and highest response time."]
    #[inline(always)]
    pub fn profile0(self) -> &'a mut W {
        self.variant(POWER_A::PROFILE0)
    }
    #[doc = "Power profile 1"]
    #[inline(always)]
    pub fn profile1(self) -> &'a mut W {
        self.variant(POWER_A::PROFILE1)
    }
    #[doc = "Power profile 2"]
    #[inline(always)]
    pub fn profile2(self) -> &'a mut W {
        self.variant(POWER_A::PROFILE2)
    }
    #[doc = "Power profile 3"]
    #[inline(always)]
    pub fn profile3(self) -> &'a mut W {
        self.variant(POWER_A::PROFILE3)
    }
}
#[doc = "Field `OUTEN` reader - Output Pad Enable"]
pub type OUTEN_R = crate::BitReader<bool>;
#[doc = "Field `OUTEN` writer - Output Pad Enable"]
pub type OUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby Mode"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby Mode"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Hysteresis Mode"]
    #[inline(always)]
    pub fn hysmode(&self) -> HYSMODE_R {
        HYSMODE_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 3:4 - Power profile"]
    #[inline(always)]
    pub fn power(&self) -> POWER_R {
        POWER_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 6 - Output Pad Enable"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Run in Standby Mode"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 1:2 - Hysteresis Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hysmode(&mut self) -> HYSMODE_W<1> {
        HYSMODE_W::new(self)
    }
    #[doc = "Bits 3:4 - Power profile"]
    #[inline(always)]
    #[must_use]
    pub fn power(&mut self) -> POWER_W<3> {
        POWER_W::new(self)
    }
    #[doc = "Bit 6 - Output Pad Enable"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<6> {
        OUTEN_W::new(self)
    }
    #[doc = "Bit 7 - Run in Standby Mode"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<7> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
