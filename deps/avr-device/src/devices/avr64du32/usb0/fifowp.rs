#[doc = "Register `FIFOWP` reader"]
pub struct R(crate::R<FIFOWP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOWP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOWP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOWP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOWP` writer"]
pub struct W(crate::W<FIFOWP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOWP_SPEC>;
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
impl From<crate::W<FIFOWP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOWP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOWP` reader - FIFO Write Pointer"]
pub type FIFOWP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOWP` writer - FIFO Write Pointer"]
pub type FIFOWP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, FIFOWP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - FIFO Write Pointer"]
    #[inline(always)]
    pub fn fifowp(&self) -> FIFOWP_R {
        FIFOWP_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - FIFO Write Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn fifowp(&mut self) -> FIFOWP_W<0> {
        FIFOWP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Write Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifowp](index.html) module"]
pub struct FIFOWP_SPEC;
impl crate::RegisterSpec for FIFOWP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fifowp::R](R) reader structure"]
impl crate::Readable for FIFOWP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifowp::W](W) writer structure"]
impl crate::Writable for FIFOWP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOWP to value 0"]
impl crate::Resettable for FIFOWP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
