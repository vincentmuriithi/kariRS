#[doc = "Register `SAMPLE` reader"]
pub struct R(crate::R<SAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPLE` writer"]
pub struct W(crate::W<SAMPLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLE_SPEC>;
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
impl From<crate::W<SAMPLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPLE` reader - ADC Sample"]
pub type SAMPLE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC Sample"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "ADC Sample\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample](index.html) module"]
pub struct SAMPLE_SPEC;
impl crate::RegisterSpec for SAMPLE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sample::R](R) reader structure"]
impl crate::Readable for SAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sample::W](W) writer structure"]
impl crate::Writable for SAMPLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPLE to value 0"]
impl crate::Resettable for SAMPLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
