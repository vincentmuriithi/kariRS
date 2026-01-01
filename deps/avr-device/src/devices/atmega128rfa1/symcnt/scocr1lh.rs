#[doc = "Register `SCOCR1LH` reader"]
pub struct R(crate::R<SCOCR1LH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCOCR1LH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCOCR1LH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCOCR1LH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCOCR1LH` writer"]
pub struct W(crate::W<SCOCR1LH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCOCR1LH_SPEC>;
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
impl From<crate::W<SCOCR1LH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCOCR1LH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCOCR1LH` reader - Symbol Counter Output Compare Register 1 LH-Byte"]
pub type SCOCR1LH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCOCR1LH` writer - Symbol Counter Output Compare Register 1 LH-Byte"]
pub type SCOCR1LH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCOCR1LH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 1 LH-Byte"]
    #[inline(always)]
    pub fn scocr1lh(&self) -> SCOCR1LH_R {
        SCOCR1LH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 1 LH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scocr1lh(&mut self) -> SCOCR1LH_W<0> {
        SCOCR1LH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Symbol Counter Output Compare Register 1 LH-Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scocr1lh](index.html) module"]
pub struct SCOCR1LH_SPEC;
impl crate::RegisterSpec for SCOCR1LH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scocr1lh::R](R) reader structure"]
impl crate::Readable for SCOCR1LH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scocr1lh::W](W) writer structure"]
impl crate::Writable for SCOCR1LH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCOCR1LH to value 0"]
impl crate::Resettable for SCOCR1LH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
