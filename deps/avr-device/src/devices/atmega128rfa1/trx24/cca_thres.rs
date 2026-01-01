#[doc = "Register `CCA_THRES` reader"]
pub struct R(crate::R<CCA_THRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCA_THRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCA_THRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCA_THRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCA_THRES` writer"]
pub struct W(crate::W<CCA_THRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCA_THRES_SPEC>;
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
impl From<crate::W<CCA_THRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCA_THRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCA_ED_THRES` reader - ED Threshold Level for CCA Measurement"]
pub type CCA_ED_THRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCA_ED_THRES` writer - ED Threshold Level for CCA Measurement"]
pub type CCA_ED_THRES_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CCA_THRES_SPEC, u8, u8, 4, O>;
#[doc = "Field `CCA_CS_THRES` reader - CS Threshold Level for CCA Measurement"]
pub type CCA_CS_THRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCA_CS_THRES` writer - CS Threshold Level for CCA Measurement"]
pub type CCA_CS_THRES_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CCA_THRES_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - ED Threshold Level for CCA Measurement"]
    #[inline(always)]
    pub fn cca_ed_thres(&self) -> CCA_ED_THRES_R {
        CCA_ED_THRES_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - CS Threshold Level for CCA Measurement"]
    #[inline(always)]
    pub fn cca_cs_thres(&self) -> CCA_CS_THRES_R {
        CCA_CS_THRES_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - ED Threshold Level for CCA Measurement"]
    #[inline(always)]
    #[must_use]
    pub fn cca_ed_thres(&mut self) -> CCA_ED_THRES_W<0> {
        CCA_ED_THRES_W::new(self)
    }
    #[doc = "Bits 4:7 - CS Threshold Level for CCA Measurement"]
    #[inline(always)]
    #[must_use]
    pub fn cca_cs_thres(&mut self) -> CCA_CS_THRES_W<4> {
        CCA_CS_THRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver CCA Threshold Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cca_thres](index.html) module"]
pub struct CCA_THRES_SPEC;
impl crate::RegisterSpec for CCA_THRES_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cca_thres::R](R) reader structure"]
impl crate::Readable for CCA_THRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cca_thres::W](W) writer structure"]
impl crate::Writable for CCA_THRES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCA_THRES to value 0"]
impl crate::Resettable for CCA_THRES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
