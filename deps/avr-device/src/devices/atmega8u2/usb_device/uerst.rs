#[doc = "Register `UERST` reader"]
pub struct R(crate::R<UERST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UERST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UERST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UERST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UERST` writer"]
pub struct W(crate::W<UERST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UERST_SPEC>;
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
impl From<crate::W<UERST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UERST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPRST` reader - Endpoint FIFO Reset Bits"]
pub type EPRST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPRST` writer - Endpoint FIFO Reset Bits"]
pub type EPRST_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UERST_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Endpoint FIFO Reset Bits"]
    #[inline(always)]
    pub fn eprst(&self) -> EPRST_R {
        EPRST_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Endpoint FIFO Reset Bits"]
    #[inline(always)]
    #[must_use]
    pub fn eprst(&mut self) -> EPRST_W<0> {
        EPRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uerst](index.html) module"]
pub struct UERST_SPEC;
impl crate::RegisterSpec for UERST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uerst::R](R) reader structure"]
impl crate::Readable for UERST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uerst::W](W) writer structure"]
impl crate::Writable for UERST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UERST to value 0"]
impl crate::Resettable for UERST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
