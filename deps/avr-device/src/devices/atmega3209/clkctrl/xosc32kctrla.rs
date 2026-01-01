#[doc = "Register `XOSC32KCTRLA` reader"]
pub struct R(crate::R<XOSC32KCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSC32KCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSC32KCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSC32KCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSC32KCTRLA` writer"]
pub struct W(crate::W<XOSC32KCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSC32KCTRLA_SPEC>;
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
impl From<crate::W<XOSC32KCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSC32KCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, XOSC32KCTRLA_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, XOSC32KCTRLA_SPEC, bool, O>;
#[doc = "Field `SEL` reader - Select"]
pub type SEL_R = crate::BitReader<bool>;
#[doc = "Field `SEL` writer - Select"]
pub type SEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, XOSC32KCTRLA_SPEC, bool, O>;
#[doc = "Field `CSUT` reader - Crystal startup time"]
pub type CSUT_R = crate::FieldReader<u8, CSUT_A>;
#[doc = "Crystal startup time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSUT_A {
    #[doc = "0: 1k cycles"]
    _1K = 0,
    #[doc = "1: 16k cycles"]
    _16K = 1,
    #[doc = "2: 32k cycles"]
    _32K = 2,
    #[doc = "3: 64k cycles"]
    _64K = 3,
}
impl From<CSUT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSUT_A) -> Self {
        variant as _
    }
}
impl CSUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSUT_A {
        match self.bits {
            0 => CSUT_A::_1K,
            1 => CSUT_A::_16K,
            2 => CSUT_A::_32K,
            3 => CSUT_A::_64K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == CSUT_A::_1K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == CSUT_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == CSUT_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == CSUT_A::_64K
    }
}
#[doc = "Field `CSUT` writer - Crystal startup time"]
pub type CSUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, XOSC32KCTRLA_SPEC, u8, CSUT_A, 2, O>;
impl<'a, const O: u8> CSUT_W<'a, O> {
    #[doc = "1k cycles"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut W {
        self.variant(CSUT_A::_1K)
    }
    #[doc = "16k cycles"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut W {
        self.variant(CSUT_A::_16K)
    }
    #[doc = "32k cycles"]
    #[inline(always)]
    pub fn _32k(self) -> &'a mut W {
        self.variant(CSUT_A::_32K)
    }
    #[doc = "64k cycles"]
    #[inline(always)]
    pub fn _64k(self) -> &'a mut W {
        self.variant(CSUT_A::_64K)
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Run standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Crystal startup time"]
    #[inline(always)]
    pub fn csut(&self) -> CSUT_R {
        CSUT_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Run standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<1> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 2 - Select"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<2> {
        SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - Crystal startup time"]
    #[inline(always)]
    #[must_use]
    pub fn csut(&mut self) -> CSUT_W<4> {
        CSUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XOSC32K Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc32kctrla](index.html) module"]
pub struct XOSC32KCTRLA_SPEC;
impl crate::RegisterSpec for XOSC32KCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [xosc32kctrla::R](R) reader structure"]
impl crate::Readable for XOSC32KCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xosc32kctrla::W](W) writer structure"]
impl crate::Writable for XOSC32KCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XOSC32KCTRLA to value 0"]
impl crate::Resettable for XOSC32KCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
