#[doc = "Register `ADC` reader"]
pub struct R(crate::R<ADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC` writer"]
pub struct W(crate::W<ADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_SPEC>;
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
impl From<crate::W<ADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC` reader - ADC Data bits"]
pub type ADC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC` writer - ADC Data bits"]
pub type ADC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, ADC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - ADC Data bits"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC Data bits"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<0> {
        ADC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "ADC Data Register Bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc](index.html) module"]
pub struct ADC_SPEC;
impl crate::RegisterSpec for ADC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc::R](R) reader structure"]
impl crate::Readable for ADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc::W](W) writer structure"]
impl crate::Writable for ADC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC to value 0"]
impl crate::Resettable for ADC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
