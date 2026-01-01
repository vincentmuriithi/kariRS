#[doc = "Register `OSCCAL` reader"]
pub struct R(crate::R<OSCCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCCAL` writer"]
pub struct W(crate::W<OSCCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCCAL_SPEC>;
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
impl From<crate::W<OSCCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSCCAL` reader - Oscillator Calibration"]
pub type OSCCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSCCAL` writer - Oscillator Calibration"]
pub type OSCCAL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, OSCCAL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Oscillator Calibration"]
    #[inline(always)]
    pub fn osccal(&self) -> OSCCAL_R {
        OSCCAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn osccal(&mut self) -> OSCCAL_W<0> {
        OSCCAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccal](index.html) module"]
pub struct OSCCAL_SPEC;
impl crate::RegisterSpec for OSCCAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osccal::R](R) reader structure"]
impl crate::Readable for OSCCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osccal::W](W) writer structure"]
impl crate::Writable for OSCCAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCCAL to value 0"]
impl crate::Resettable for OSCCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
