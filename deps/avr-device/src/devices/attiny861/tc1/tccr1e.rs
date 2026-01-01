#[doc = "Register `TCCR1E` reader"]
pub struct R(crate::R<TCCR1E_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR1E_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR1E_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR1E_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR1E` writer"]
pub struct W(crate::W<TCCR1E_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR1E_SPEC>;
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
impl From<crate::W<TCCR1E_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR1E_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OC1OE` reader - Ouput Compare Override Enable Bits"]
pub type OC1OE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC1OE` writer - Ouput Compare Override Enable Bits"]
pub type OC1OE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR1E_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Ouput Compare Override Enable Bits"]
    #[inline(always)]
    pub fn oc1oe(&self) -> OC1OE_R {
        OC1OE_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Ouput Compare Override Enable Bits"]
    #[inline(always)]
    #[must_use]
    pub fn oc1oe(&mut self) -> OC1OE_W<0> {
        OC1OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter1 Control Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr1e](index.html) module"]
pub struct TCCR1E_SPEC;
impl crate::RegisterSpec for TCCR1E_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr1e::R](R) reader structure"]
impl crate::Readable for TCCR1E_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr1e::W](W) writer structure"]
impl crate::Writable for TCCR1E_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1E to value 0"]
impl crate::Resettable for TCCR1E_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
