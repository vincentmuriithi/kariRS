#[doc = "Register `OCR5C` reader"]
pub struct R(crate::R<OCR5C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCR5C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCR5C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCR5C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCR5C` writer"]
pub struct W(crate::W<OCR5C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCR5C_SPEC>;
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
impl From<crate::W<OCR5C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCR5C_SPEC>) -> Self {
        W(writer)
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
#[doc = "Timer/Counter5 Output Compare Register B Bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocr5c](index.html) module"]
pub struct OCR5C_SPEC;
impl crate::RegisterSpec for OCR5C_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ocr5c::R](R) reader structure"]
impl crate::Readable for OCR5C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocr5c::W](W) writer structure"]
impl crate::Writable for OCR5C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR5C to value 0"]
impl crate::Resettable for OCR5C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
