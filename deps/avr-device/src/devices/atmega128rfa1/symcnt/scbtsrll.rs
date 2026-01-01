#[doc = "Register `SCBTSRLL` reader"]
pub struct R(crate::R<SCBTSRLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCBTSRLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCBTSRLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCBTSRLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCBTSRLL` writer"]
pub struct W(crate::W<SCBTSRLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCBTSRLL_SPEC>;
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
impl From<crate::W<SCBTSRLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCBTSRLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCBTSRLL` reader - Symbol Counter Beacon Timestamp Register LL-Byte"]
pub type SCBTSRLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCBTSRLL` writer - Symbol Counter Beacon Timestamp Register LL-Byte"]
pub type SCBTSRLL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCBTSRLL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register LL-Byte"]
    #[inline(always)]
    pub fn scbtsrll(&self) -> SCBTSRLL_R {
        SCBTSRLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register LL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scbtsrll(&mut self) -> SCBTSRLL_W<0> {
        SCBTSRLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Symbol Counter Beacon Timestamp Register LL-Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scbtsrll](index.html) module"]
pub struct SCBTSRLL_SPEC;
impl crate::RegisterSpec for SCBTSRLL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scbtsrll::R](R) reader structure"]
impl crate::Readable for SCBTSRLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scbtsrll::W](W) writer structure"]
impl crate::Writable for SCBTSRLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCBTSRLL to value 0"]
impl crate::Resettable for SCBTSRLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
