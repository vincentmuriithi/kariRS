#[doc = "Register `FIFORP` reader"]
pub struct R(crate::R<FIFORP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFORP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFORP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFORP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFORP` writer"]
pub struct W(crate::W<FIFORP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFORP_SPEC>;
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
impl From<crate::W<FIFORP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFORP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFORP` reader - FIFO Read Pointer"]
pub type FIFORP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFORP` writer - FIFO Read Pointer"]
pub type FIFORP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, FIFORP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - FIFO Read Pointer"]
    #[inline(always)]
    pub fn fiforp(&self) -> FIFORP_R {
        FIFORP_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - FIFO Read Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn fiforp(&mut self) -> FIFORP_W<0> {
        FIFORP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Read Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fiforp](index.html) module"]
pub struct FIFORP_SPEC;
impl crate::RegisterSpec for FIFORP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fiforp::R](R) reader structure"]
impl crate::Readable for FIFORP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fiforp::W](W) writer structure"]
impl crate::Writable for FIFORP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFORP to value 0"]
impl crate::Resettable for FIFORP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
