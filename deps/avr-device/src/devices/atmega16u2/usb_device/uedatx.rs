#[doc = "Register `UEDATX` reader"]
pub struct R(crate::R<UEDATX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEDATX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEDATX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEDATX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEDATX` writer"]
pub struct W(crate::W<UEDATX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEDATX_SPEC>;
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
impl From<crate::W<UEDATX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEDATX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT` reader - Data bits"]
pub type DAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAT` writer - Data bits"]
pub type DAT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UEDATX_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data bits"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data bits"]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self) -> DAT_W<0> {
        DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "USB Data Endpoint\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uedatx](index.html) module"]
pub struct UEDATX_SPEC;
impl crate::RegisterSpec for UEDATX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uedatx::R](R) reader structure"]
impl crate::Readable for UEDATX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uedatx::W](W) writer structure"]
impl crate::Writable for UEDATX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEDATX to value 0"]
impl crate::Resettable for UEDATX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
