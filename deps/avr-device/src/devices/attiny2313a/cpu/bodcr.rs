#[doc = "Register `BODCR` reader"]
pub struct R(crate::R<BODCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BODCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BODCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BODCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BODCR` writer"]
pub struct W(crate::W<BODCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BODCR_SPEC>;
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
impl From<crate::W<BODCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BODCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BPDSE` reader - No Description."]
pub type BPDSE_R = crate::BitReader<bool>;
#[doc = "Field `BPDSE` writer - No Description."]
pub type BPDSE_W<'a, const O: u8> = crate::BitWriter<'a, u8, BODCR_SPEC, bool, O>;
#[doc = "Field `BPDS` reader - No Description."]
pub type BPDS_R = crate::BitReader<bool>;
#[doc = "Field `BPDS` writer - No Description."]
pub type BPDS_W<'a, const O: u8> = crate::BitWriter<'a, u8, BODCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn bpdse(&self) -> BPDSE_R {
        BPDSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn bpds(&self) -> BPDS_R {
        BPDS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn bpdse(&mut self) -> BPDSE_W<0> {
        BPDSE_W::new(self)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn bpds(&mut self) -> BPDS_W<1> {
        BPDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodcr](index.html) module"]
pub struct BODCR_SPEC;
impl crate::RegisterSpec for BODCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bodcr::R](R) reader structure"]
impl crate::Readable for BODCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bodcr::W](W) writer structure"]
impl crate::Writable for BODCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BODCR to value 0"]
impl crate::Resettable for BODCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
