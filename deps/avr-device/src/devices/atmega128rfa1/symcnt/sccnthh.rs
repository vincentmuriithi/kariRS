#[doc = "Register `SCCNTHH` reader"]
pub struct R(crate::R<SCCNTHH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCCNTHH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCCNTHH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCCNTHH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCCNTHH` writer"]
pub struct W(crate::W<SCCNTHH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCCNTHH_SPEC>;
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
impl From<crate::W<SCCNTHH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCCNTHH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCCNTHH` reader - Symbol Counter Register HH-Byte"]
pub type SCCNTHH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCCNTHH` writer - Symbol Counter Register HH-Byte"]
pub type SCCNTHH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCCNTHH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Register HH-Byte"]
    #[inline(always)]
    pub fn sccnthh(&self) -> SCCNTHH_R {
        SCCNTHH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Register HH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sccnthh(&mut self) -> SCCNTHH_W<0> {
        SCCNTHH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Symbol Counter Register HH-Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sccnthh](index.html) module"]
pub struct SCCNTHH_SPEC;
impl crate::RegisterSpec for SCCNTHH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sccnthh::R](R) reader structure"]
impl crate::Readable for SCCNTHH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sccnthh::W](W) writer structure"]
impl crate::Writable for SCCNTHH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCNTHH to value 0"]
impl crate::Resettable for SCCNTHH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
