#[doc = "Register `OCR0A` reader"]
pub struct R(crate::R<OCR0A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCR0A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCR0A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCR0A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCR0A` writer"]
pub struct W(crate::W<OCR0A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCR0A_SPEC>;
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
impl From<crate::W<OCR0A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCR0A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCR0A` reader - Timer/Counter0 Output Compare A bits"]
pub type OCR0A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OCR0A` writer - Timer/Counter0 Output Compare A bits"]
pub type OCR0A_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, OCR0A_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Timer/Counter0 Output Compare A bits"]
    #[inline(always)]
    pub fn ocr0a(&self) -> OCR0A_R {
        OCR0A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer/Counter0 Output Compare A bits"]
    #[inline(always)]
    #[must_use]
    pub fn ocr0a(&mut self) -> OCR0A_W<0> {
        OCR0A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Timer/Counter0 Output Compare Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocr0a](index.html) module"]
pub struct OCR0A_SPEC;
impl crate::RegisterSpec for OCR0A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ocr0a::R](R) reader structure"]
impl crate::Readable for OCR0A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocr0a::W](W) writer structure"]
impl crate::Writable for OCR0A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR0A to value 0"]
impl crate::Resettable for OCR0A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
