#[doc = "Register `SCOCR1HH` reader"]
pub struct R(crate::R<SCOCR1HH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCOCR1HH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCOCR1HH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCOCR1HH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCOCR1HH` writer"]
pub struct W(crate::W<SCOCR1HH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCOCR1HH_SPEC>;
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
impl From<crate::W<SCOCR1HH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCOCR1HH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCOCR1HH` reader - Symbol Counter Output Compare Register 1 HH-Byte"]
pub type SCOCR1HH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCOCR1HH` writer - Symbol Counter Output Compare Register 1 HH-Byte"]
pub type SCOCR1HH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCOCR1HH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 1 HH-Byte"]
    #[inline(always)]
    pub fn scocr1hh(&self) -> SCOCR1HH_R {
        SCOCR1HH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 1 HH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scocr1hh(&mut self) -> SCOCR1HH_W<0> {
        SCOCR1HH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Symbol Counter Output Compare Register 1 HH-Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scocr1hh](index.html) module"]
pub struct SCOCR1HH_SPEC;
impl crate::RegisterSpec for SCOCR1HH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scocr1hh::R](R) reader structure"]
impl crate::Readable for SCOCR1HH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scocr1hh::W](W) writer structure"]
impl crate::Writable for SCOCR1HH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCOCR1HH to value 0"]
impl crate::Resettable for SCOCR1HH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
