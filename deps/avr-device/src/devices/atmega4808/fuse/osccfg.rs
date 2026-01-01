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
#[doc = "Field `FREQSEL` reader - Frequency Select"]
pub type FREQSEL_R = crate::FieldReader<u8, FREQSEL_A>;
#[doc = "Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FREQSEL_A {
    #[doc = "1: 16 MHz"]
    _16MHZ = 1,
    #[doc = "2: 20 MHz"]
    _20MHZ = 2,
}
impl From<FREQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQSEL_A) -> Self {
        variant as _
    }
}
impl FREQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FREQSEL_A> {
        match self.bits {
            1 => Some(FREQSEL_A::_16MHZ),
            2 => Some(FREQSEL_A::_20MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        *self == FREQSEL_A::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_20MHZ`"]
    #[inline(always)]
    pub fn is_20mhz(&self) -> bool {
        *self == FREQSEL_A::_20MHZ
    }
}
#[doc = "Field `FREQSEL` writer - Frequency Select"]
pub type FREQSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, OSCCFG_SPEC, u8, FREQSEL_A, 2, O>;
impl<'a, const O: u8> FREQSEL_W<'a, O> {
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_16MHZ)
    }
    #[doc = "20 MHz"]
    #[inline(always)]
    pub fn _20mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_20MHZ)
    }
}
#[doc = "Field `OSCLOCK` reader - Oscillator Lock"]
pub type OSCLOCK_R = crate::BitReader<bool>;
#[doc = "Field `OSCLOCK` writer - Oscillator Lock"]
pub type OSCLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u8, OSCCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Frequency Select"]
    #[inline(always)]
    pub fn freqsel(&self) -> FREQSEL_R {
        FREQSEL_R::new(self.bits & 3)
    }
    #[doc = "Bit 7 - Oscillator Lock"]
    #[inline(always)]
    pub fn osclock(&self) -> OSCLOCK_R {
        OSCLOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn freqsel(&mut self) -> FREQSEL_W<0> {
        FREQSEL_W::new(self)
    }
    #[doc = "Bit 7 - Oscillator Lock"]
    #[inline(always)]
    #[must_use]
    pub fn osclock(&mut self) -> OSCLOCK_W<7> {
        OSCLOCK_W::new(self)
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
