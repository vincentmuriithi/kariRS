#[doc = "Register `LLCR` reader"]
pub struct R(crate::R<LLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LLCR` writer"]
pub struct W(crate::W<LLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LLCR_SPEC>;
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
impl From<crate::W<LLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLENCAL` reader - Enable Automatic Calibration"]
pub type LLENCAL_R = crate::BitReader<bool>;
#[doc = "Field `LLENCAL` writer - Enable Automatic Calibration"]
pub type LLENCAL_W<'a, const O: u8> = crate::BitWriter<'a, u8, LLCR_SPEC, bool, O>;
#[doc = "Field `LLSHORT` reader - Short Lower Calibration Circuit"]
pub type LLSHORT_R = crate::BitReader<bool>;
#[doc = "Field `LLSHORT` writer - Short Lower Calibration Circuit"]
pub type LLSHORT_W<'a, const O: u8> = crate::BitWriter<'a, u8, LLCR_SPEC, bool, O>;
#[doc = "Field `LLTCO` reader - Temperature Coefficient of Current Source"]
pub type LLTCO_R = crate::BitReader<bool>;
#[doc = "Field `LLTCO` writer - Temperature Coefficient of Current Source"]
pub type LLTCO_W<'a, const O: u8> = crate::BitWriter<'a, u8, LLCR_SPEC, bool, O>;
#[doc = "Field `LLCAL` reader - Calibration Active"]
pub type LLCAL_R = crate::BitReader<bool>;
#[doc = "Field `LLCAL` writer - Calibration Active"]
pub type LLCAL_W<'a, const O: u8> = crate::BitWriter<'a, u8, LLCR_SPEC, bool, O>;
#[doc = "Field `LLCOMP` reader - Comparator Output"]
pub type LLCOMP_R = crate::BitReader<bool>;
#[doc = "Field `LLCOMP` writer - Comparator Output"]
pub type LLCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u8, LLCR_SPEC, bool, O>;
#[doc = "Field `LLDONE` reader - Calibration Done"]
pub type LLDONE_R = crate::BitReader<bool>;
#[doc = "Field `LLDONE` writer - Calibration Done"]
pub type LLDONE_W<'a, const O: u8> = crate::BitWriter<'a, u8, LLCR_SPEC, bool, O>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LLCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Enable Automatic Calibration"]
    #[inline(always)]
    pub fn llencal(&self) -> LLENCAL_R {
        LLENCAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Short Lower Calibration Circuit"]
    #[inline(always)]
    pub fn llshort(&self) -> LLSHORT_R {
        LLSHORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Temperature Coefficient of Current Source"]
    #[inline(always)]
    pub fn lltco(&self) -> LLTCO_R {
        LLTCO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Calibration Active"]
    #[inline(always)]
    pub fn llcal(&self) -> LLCAL_R {
        LLCAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator Output"]
    #[inline(always)]
    pub fn llcomp(&self) -> LLCOMP_R {
        LLCOMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Calibration Done"]
    #[inline(always)]
    pub fn lldone(&self) -> LLDONE_R {
        LLDONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Automatic Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn llencal(&mut self) -> LLENCAL_W<0> {
        LLENCAL_W::new(self)
    }
    #[doc = "Bit 1 - Short Lower Calibration Circuit"]
    #[inline(always)]
    #[must_use]
    pub fn llshort(&mut self) -> LLSHORT_W<1> {
        LLSHORT_W::new(self)
    }
    #[doc = "Bit 2 - Temperature Coefficient of Current Source"]
    #[inline(always)]
    #[must_use]
    pub fn lltco(&mut self) -> LLTCO_W<2> {
        LLTCO_W::new(self)
    }
    #[doc = "Bit 3 - Calibration Active"]
    #[inline(always)]
    #[must_use]
    pub fn llcal(&mut self) -> LLCAL_W<3> {
        LLCAL_W::new(self)
    }
    #[doc = "Bit 4 - Comparator Output"]
    #[inline(always)]
    #[must_use]
    pub fn llcomp(&mut self) -> LLCOMP_W<4> {
        LLCOMP_W::new(self)
    }
    #[doc = "Bit 5 - Calibration Done"]
    #[inline(always)]
    #[must_use]
    pub fn lldone(&mut self) -> LLDONE_W<5> {
        LLDONE_W::new(self)
    }
    #[doc = "Bits 6:7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<6> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Leakage Voltage Regulator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [llcr](index.html) module"]
pub struct LLCR_SPEC;
impl crate::RegisterSpec for LLCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [llcr::R](R) reader structure"]
impl crate::Readable for LLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [llcr::W](W) writer structure"]
impl crate::Writable for LLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LLCR to value 0"]
impl crate::Resettable for LLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
