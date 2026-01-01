#[doc = "Register `OSCHFTUNE` reader"]
pub struct R(crate::R<OSCHFTUNE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCHFTUNE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCHFTUNE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCHFTUNE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCHFTUNE` writer"]
pub struct W(crate::W<OSCHFTUNE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCHFTUNE_SPEC>;
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
impl From<crate::W<OSCHFTUNE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCHFTUNE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNE` reader - Tune"]
pub type TUNE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNE` writer - Tune"]
pub type TUNE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, OSCHFTUNE_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Tune"]
    #[inline(always)]
    pub fn tune(&self) -> TUNE_R {
        TUNE_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Tune"]
    #[inline(always)]
    #[must_use]
    pub fn tune(&mut self) -> TUNE_W<0> {
        TUNE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSCHF Tune\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oschftune](index.html) module"]
pub struct OSCHFTUNE_SPEC;
impl crate::RegisterSpec for OSCHFTUNE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [oschftune::R](R) reader structure"]
impl crate::Readable for OSCHFTUNE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oschftune::W](W) writer structure"]
impl crate::Writable for OSCHFTUNE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCHFTUNE to value 0"]
impl crate::Resettable for OSCHFTUNE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
