#[doc = "Register `OSCCFG` reader"]
pub struct R(crate::R<OSCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCCFG` writer"]
pub struct W(crate::W<OSCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCCFG_SPEC>;
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
impl From<crate::W<OSCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Frequency Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: 1-32MHz internal oscillator"]
    OSCHF = 0,
    #[doc = "1: 32.768kHz internal oscillator"]
    OSC32K = 1,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::OSCHF),
            1 => Some(CLKSEL_A::OSC32K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OSCHF`"]
    #[inline(always)]
    pub fn is_oschf(&self) -> bool {
        *self == CLKSEL_A::OSCHF
    }
    #[doc = "Checks if the value of the field is `OSC32K`"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == CLKSEL_A::OSC32K
    }
}
#[doc = "Field `CLKSEL` writer - Frequency Select"]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, OSCCFG_SPEC, u8, CLKSEL_A, 3, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "1-32MHz internal oscillator"]
    #[inline(always)]
    pub fn oschf(self) -> &'a mut W {
        self.variant(CLKSEL_A::OSCHF)
    }
    #[doc = "32.768kHz internal oscillator"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut W {
        self.variant(CLKSEL_A::OSC32K)
    }
}
impl R {
    #[doc = "Bits 0:2 - Frequency Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccfg](index.html) module"]
pub struct OSCCFG_SPEC;
impl crate::RegisterSpec for OSCCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osccfg::R](R) reader structure"]
impl crate::Readable for OSCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osccfg::W](W) writer structure"]
impl crate::Writable for OSCCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCCFG to value 0"]
impl crate::Resettable for OSCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
