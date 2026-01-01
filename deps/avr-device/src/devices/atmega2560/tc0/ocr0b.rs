#[doc = "Register `OCR0B` reader"]
pub struct R(crate::R<OCR0B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCR0B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCR0B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCR0B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCR0B` writer"]
pub struct W(crate::W<OCR0B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCR0B_SPEC>;
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
impl From<crate::W<OCR0B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCR0B_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Timer/Counter0 Output Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocr0b](index.html) module"]
pub struct OCR0B_SPEC;
impl crate::RegisterSpec for OCR0B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ocr0b::R](R) reader structure"]
impl crate::Readable for OCR0B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocr0b::W](W) writer structure"]
impl crate::Writable for OCR0B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR0B to value 0"]
impl crate::Resettable for OCR0B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
