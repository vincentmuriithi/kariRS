#[doc = "Register `SCCNTLL` reader"]
pub struct R(crate::R<SCCNTLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCCNTLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCCNTLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCCNTLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCCNTLL` writer"]
pub struct W(crate::W<SCCNTLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCCNTLL_SPEC>;
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
impl From<crate::W<SCCNTLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCCNTLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCCNTLL` reader - Symbol Counter Register LL-Byte"]
pub type SCCNTLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCCNTLL` writer - Symbol Counter Register LL-Byte"]
pub type SCCNTLL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCCNTLL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Register LL-Byte"]
    #[inline(always)]
    pub fn sccntll(&self) -> SCCNTLL_R {
        SCCNTLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Register LL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sccntll(&mut self) -> SCCNTLL_W<0> {
        SCCNTLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Symbol Counter Register LL-Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sccntll](index.html) module"]
pub struct SCCNTLL_SPEC;
impl crate::RegisterSpec for SCCNTLL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sccntll::R](R) reader structure"]
impl crate::Readable for SCCNTLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sccntll::W](W) writer structure"]
impl crate::Writable for SCCNTLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCNTLL to value 0"]
impl crate::Resettable for SCCNTLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
