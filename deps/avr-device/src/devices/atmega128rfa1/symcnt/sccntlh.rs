#[doc = "Register `SCCNTLH` reader"]
pub struct R(crate::R<SCCNTLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCCNTLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCCNTLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCCNTLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCCNTLH` writer"]
pub struct W(crate::W<SCCNTLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCCNTLH_SPEC>;
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
impl From<crate::W<SCCNTLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCCNTLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCCNTLH` reader - Symbol Counter Register LH-Byte"]
pub type SCCNTLH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCCNTLH` writer - Symbol Counter Register LH-Byte"]
pub type SCCNTLH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCCNTLH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Register LH-Byte"]
    #[inline(always)]
    pub fn sccntlh(&self) -> SCCNTLH_R {
        SCCNTLH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Register LH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sccntlh(&mut self) -> SCCNTLH_W<0> {
        SCCNTLH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Symbol Counter Register LH-Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sccntlh](index.html) module"]
pub struct SCCNTLH_SPEC;
impl crate::RegisterSpec for SCCNTLH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sccntlh::R](R) reader structure"]
impl crate::Readable for SCCNTLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sccntlh::W](W) writer structure"]
impl crate::Writable for SCCNTLH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCNTLH to value 0"]
impl crate::Resettable for SCCNTLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
