#[doc = "Register `PORTCR` reader"]
pub struct R(crate::R<PORTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTCR` writer"]
pub struct W(crate::W<PORTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTCR_SPEC>;
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
impl From<crate::W<PORTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUDA` reader - No Description."]
pub type PUDA_R = crate::BitReader<bool>;
#[doc = "Field `PUDA` writer - No Description."]
pub type PUDA_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTCR_SPEC, bool, O>;
#[doc = "Field `PUDB` reader - No Description."]
pub type PUDB_R = crate::BitReader<bool>;
#[doc = "Field `PUDB` writer - No Description."]
pub type PUDB_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTCR_SPEC, bool, O>;
#[doc = "Field `BBMA` reader - No Description."]
pub type BBMA_R = crate::BitReader<bool>;
#[doc = "Field `BBMA` writer - No Description."]
pub type BBMA_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTCR_SPEC, bool, O>;
#[doc = "Field `BBMB` reader - No Description."]
pub type BBMB_R = crate::BitReader<bool>;
#[doc = "Field `BBMB` writer - No Description."]
pub type BBMB_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn puda(&self) -> PUDA_R {
        PUDA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn pudb(&self) -> PUDB_R {
        PUDB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    pub fn bbma(&self) -> BBMA_R {
        BBMA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn bbmb(&self) -> BBMB_R {
        BBMB_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn puda(&mut self) -> PUDA_W<0> {
        PUDA_W::new(self)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn pudb(&mut self) -> PUDB_W<1> {
        PUDB_W::new(self)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn bbma(&mut self) -> BBMA_W<4> {
        BBMA_W::new(self)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn bbmb(&mut self) -> BBMB_W<5> {
        BBMB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portcr](index.html) module"]
pub struct PORTCR_SPEC;
impl crate::RegisterSpec for PORTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [portcr::R](R) reader structure"]
impl crate::Readable for PORTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portcr::W](W) writer structure"]
impl crate::Writable for PORTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTCR to value 0"]
impl crate::Resettable for PORTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
