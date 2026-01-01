#[doc = "Register `UEINTX` reader"]
pub struct R(crate::R<UEINTX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEINTX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEINTX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEINTX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEINTX` writer"]
pub struct W(crate::W<UEINTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEINTX_SPEC>;
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
impl From<crate::W<UEINTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEINTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINI` reader - No Description."]
pub type TXINI_R = crate::BitReader<bool>;
#[doc = "Field `TXINI` writer - No Description."]
pub type TXINI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEINTX_SPEC, bool, O>;
#[doc = "Field `STALLEDI` reader - No Description."]
pub type STALLEDI_R = crate::BitReader<bool>;
#[doc = "Field `STALLEDI` writer - No Description."]
pub type STALLEDI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEINTX_SPEC, bool, O>;
#[doc = "Field `RXOUTI` reader - No Description."]
pub type RXOUTI_R = crate::BitReader<bool>;
#[doc = "Field `RXOUTI` writer - No Description."]
pub type RXOUTI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEINTX_SPEC, bool, O>;
#[doc = "Field `RXSTPI` reader - No Description."]
pub type RXSTPI_R = crate::BitReader<bool>;
#[doc = "Field `RXSTPI` writer - No Description."]
pub type RXSTPI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEINTX_SPEC, bool, O>;
#[doc = "Field `NAKOUTI` reader - No Description."]
pub type NAKOUTI_R = crate::BitReader<bool>;
#[doc = "Field `NAKOUTI` writer - No Description."]
pub type NAKOUTI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEINTX_SPEC, bool, O>;
#[doc = "Field `RWAL` reader - No Description."]
pub type RWAL_R = crate::BitReader<bool>;
#[doc = "Field `RWAL` writer - No Description."]
pub type RWAL_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEINTX_SPEC, bool, O>;
#[doc = "Field `NAKINI` reader - No Description."]
pub type NAKINI_R = crate::BitReader<bool>;
#[doc = "Field `NAKINI` writer - No Description."]
pub type NAKINI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEINTX_SPEC, bool, O>;
#[doc = "Field `FIFOCON` reader - No Description."]
pub type FIFOCON_R = crate::BitReader<bool>;
#[doc = "Field `FIFOCON` writer - No Description."]
pub type FIFOCON_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEINTX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn txini(&self) -> TXINI_R {
        TXINI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn stalledi(&self) -> STALLEDI_R {
        STALLEDI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    pub fn rxouti(&self) -> RXOUTI_R {
        RXOUTI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn rxstpi(&self) -> RXSTPI_R {
        RXSTPI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    pub fn nakouti(&self) -> NAKOUTI_R {
        NAKOUTI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn rwal(&self) -> RWAL_R {
        RWAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    pub fn nakini(&self) -> NAKINI_R {
        NAKINI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn txini(&mut self) -> TXINI_W<0> {
        TXINI_W::new(self)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn stalledi(&mut self) -> STALLEDI_W<1> {
        STALLEDI_W::new(self)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rxouti(&mut self) -> RXOUTI_W<2> {
        RXOUTI_W::new(self)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rxstpi(&mut self) -> RXSTPI_W<3> {
        RXSTPI_W::new(self)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn nakouti(&mut self) -> NAKOUTI_W<4> {
        NAKOUTI_W::new(self)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rwal(&mut self) -> RWAL_W<5> {
        RWAL_W::new(self)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn nakini(&mut self) -> NAKINI_W<6> {
        NAKINI_W::new(self)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn fifocon(&mut self) -> FIFOCON_W<7> {
        FIFOCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ueintx](index.html) module"]
pub struct UEINTX_SPEC;
impl crate::RegisterSpec for UEINTX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ueintx::R](R) reader structure"]
impl crate::Readable for UEINTX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ueintx::W](W) writer structure"]
impl crate::Writable for UEINTX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEINTX to value 0"]
impl crate::Resettable for UEINTX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
