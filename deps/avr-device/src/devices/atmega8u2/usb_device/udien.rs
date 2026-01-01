#[doc = "Register `UDIEN` reader"]
pub struct R(crate::R<UDIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDIEN` writer"]
pub struct W(crate::W<UDIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDIEN_SPEC>;
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
impl From<crate::W<UDIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPE` reader - Suspend Interrupt Enable Bit"]
pub type SUSPE_R = crate::BitReader<bool>;
#[doc = "Field `SUSPE` writer - Suspend Interrupt Enable Bit"]
pub type SUSPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDIEN_SPEC, bool, O>;
#[doc = "Field `SOFE` reader - Start Of Frame Interrupt Enable Bit"]
pub type SOFE_R = crate::BitReader<bool>;
#[doc = "Field `SOFE` writer - Start Of Frame Interrupt Enable Bit"]
pub type SOFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDIEN_SPEC, bool, O>;
#[doc = "Field `EORSTE` reader - End Of Reset Interrupt Enable Bit"]
pub type EORSTE_R = crate::BitReader<bool>;
#[doc = "Field `EORSTE` writer - End Of Reset Interrupt Enable Bit"]
pub type EORSTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDIEN_SPEC, bool, O>;
#[doc = "Field `WAKEUPE` reader - Wake-up CPU Interrupt Enable Bit"]
pub type WAKEUPE_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUPE` writer - Wake-up CPU Interrupt Enable Bit"]
pub type WAKEUPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDIEN_SPEC, bool, O>;
#[doc = "Field `EORSME` reader - End Of Resume Interrupt Enable Bit"]
pub type EORSME_R = crate::BitReader<bool>;
#[doc = "Field `EORSME` writer - End Of Resume Interrupt Enable Bit"]
pub type EORSME_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDIEN_SPEC, bool, O>;
#[doc = "Field `UPRSME` reader - Upstream Resume Interrupt Enable Bit"]
pub type UPRSME_R = crate::BitReader<bool>;
#[doc = "Field `UPRSME` writer - Upstream Resume Interrupt Enable Bit"]
pub type UPRSME_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDIEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Suspend Interrupt Enable Bit"]
    #[inline(always)]
    pub fn suspe(&self) -> SUSPE_R {
        SUSPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Start Of Frame Interrupt Enable Bit"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End Of Reset Interrupt Enable Bit"]
    #[inline(always)]
    pub fn eorste(&self) -> EORSTE_R {
        EORSTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-up CPU Interrupt Enable Bit"]
    #[inline(always)]
    pub fn wakeupe(&self) -> WAKEUPE_R {
        WAKEUPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt Enable Bit"]
    #[inline(always)]
    pub fn eorsme(&self) -> EORSME_R {
        EORSME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Enable Bit"]
    #[inline(always)]
    pub fn uprsme(&self) -> UPRSME_R {
        UPRSME_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn suspe(&mut self) -> SUSPE_W<0> {
        SUSPE_W::new(self)
    }
    #[doc = "Bit 2 - Start Of Frame Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn sofe(&mut self) -> SOFE_W<2> {
        SOFE_W::new(self)
    }
    #[doc = "Bit 3 - End Of Reset Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn eorste(&mut self) -> EORSTE_W<3> {
        EORSTE_W::new(self)
    }
    #[doc = "Bit 4 - Wake-up CPU Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupe(&mut self) -> WAKEUPE_W<4> {
        WAKEUPE_W::new(self)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn eorsme(&mut self) -> EORSME_W<5> {
        EORSME_W::new(self)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn uprsme(&mut self) -> UPRSME_W<6> {
        UPRSME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udien](index.html) module"]
pub struct UDIEN_SPEC;
impl crate::RegisterSpec for UDIEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [udien::R](R) reader structure"]
impl crate::Readable for UDIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udien::W](W) writer structure"]
impl crate::Writable for UDIEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDIEN to value 0"]
impl crate::Resettable for UDIEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
