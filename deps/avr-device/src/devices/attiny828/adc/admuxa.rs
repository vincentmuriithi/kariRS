#[doc = "Register `ADMUXA` reader"]
pub struct R(crate::R<ADMUXA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADMUXA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADMUXA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADMUXA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADMUXA` writer"]
pub struct W(crate::W<ADMUXA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADMUXA_SPEC>;
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
impl From<crate::W<ADMUXA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADMUXA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX` reader - Analog Channel Selection Bits 4:0"]
pub type MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUX` writer - Analog Channel Selection Bits 4:0"]
pub type MUX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ADMUXA_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Analog Channel Selection Bits 4:0"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog Channel Selection Bits 4:0"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<0> {
        MUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The ADC multiplexer Selection Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [admuxa](index.html) module"]
pub struct ADMUXA_SPEC;
impl crate::RegisterSpec for ADMUXA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [admuxa::R](R) reader structure"]
impl crate::Readable for ADMUXA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [admuxa::W](W) writer structure"]
impl crate::Writable for ADMUXA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADMUXA to value 0"]
impl crate::Resettable for ADMUXA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
