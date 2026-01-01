#[doc = "Register `OCR1C` reader"]
pub struct R(crate::R<OCR1C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCR1C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCR1C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCR1C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCR1C` writer"]
pub struct W(crate::W<OCR1C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCR1C_SPEC>;
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
impl From<crate::W<OCR1C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCR1C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCR1C` reader - Timer/Counter1 Output Compare C bits"]
pub type OCR1C_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OCR1C` writer - Timer/Counter1 Output Compare C bits"]
pub type OCR1C_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, OCR1C_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timer/Counter1 Output Compare C bits"]
    #[inline(always)]
    pub fn ocr1c(&self) -> OCR1C_R {
        OCR1C_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer/Counter1 Output Compare C bits"]
    #[inline(always)]
    #[must_use]
    pub fn ocr1c(&mut self) -> OCR1C_W<0> {
        OCR1C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Timer/Counter1 Output Compare Register C Bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocr1c](index.html) module"]
pub struct OCR1C_SPEC;
impl crate::RegisterSpec for OCR1C_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ocr1c::R](R) reader structure"]
impl crate::Readable for OCR1C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocr1c::W](W) writer structure"]
impl crate::Writable for OCR1C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR1C to value 0"]
impl crate::Resettable for OCR1C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
