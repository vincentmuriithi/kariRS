#[doc = "Register `SCOCR3LL` reader"]
pub struct R(crate::R<SCOCR3LL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCOCR3LL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCOCR3LL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCOCR3LL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCOCR3LL` writer"]
pub struct W(crate::W<SCOCR3LL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCOCR3LL_SPEC>;
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
impl From<crate::W<SCOCR3LL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCOCR3LL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCOCR3LL` reader - Symbol Counter Output Compare Register 3 LL-Byte"]
pub type SCOCR3LL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCOCR3LL` writer - Symbol Counter Output Compare Register 3 LL-Byte"]
pub type SCOCR3LL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCOCR3LL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 3 LL-Byte"]
    #[inline(always)]
    pub fn scocr3ll(&self) -> SCOCR3LL_R {
        SCOCR3LL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 3 LL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scocr3ll(&mut self) -> SCOCR3LL_W<0> {
        SCOCR3LL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Symbol Counter Output Compare Register 3 LL-Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scocr3ll](index.html) module"]
pub struct SCOCR3LL_SPEC;
impl crate::RegisterSpec for SCOCR3LL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scocr3ll::R](R) reader structure"]
impl crate::Readable for SCOCR3LL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scocr3ll::W](W) writer structure"]
impl crate::Writable for SCOCR3LL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCOCR3LL to value 0"]
impl crate::Resettable for SCOCR3LL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
