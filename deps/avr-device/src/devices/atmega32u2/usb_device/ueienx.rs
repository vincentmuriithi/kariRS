#[doc = "Register `UEIENX` reader"]
pub struct R(crate::R<UEIENX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEIENX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEIENX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEIENX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEIENX` writer"]
pub struct W(crate::W<UEIENX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEIENX_SPEC>;
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
impl From<crate::W<UEIENX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEIENX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINE` reader - Transmitter Ready Interrupt Enable Flag"]
pub type TXINE_R = crate::BitReader<bool>;
#[doc = "Field `TXINE` writer - Transmitter Ready Interrupt Enable Flag"]
pub type TXINE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEIENX_SPEC, bool, O>;
#[doc = "Field `STALLEDE` reader - Stalled Interrupt Enable Flag"]
pub type STALLEDE_R = crate::BitReader<bool>;
#[doc = "Field `STALLEDE` writer - Stalled Interrupt Enable Flag"]
pub type STALLEDE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEIENX_SPEC, bool, O>;
#[doc = "Field `RXOUTE` reader - Received OUT Data Interrupt Enable Flag"]
pub type RXOUTE_R = crate::BitReader<bool>;
#[doc = "Field `RXOUTE` writer - Received OUT Data Interrupt Enable Flag"]
pub type RXOUTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEIENX_SPEC, bool, O>;
#[doc = "Field `RXSTPE` reader - Received SETUP Interrupt Enable Flag"]
pub type RXSTPE_R = crate::BitReader<bool>;
#[doc = "Field `RXSTPE` writer - Received SETUP Interrupt Enable Flag"]
pub type RXSTPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEIENX_SPEC, bool, O>;
#[doc = "Field `NAKOUTE` reader - NAK OUT Interrupt Enable Bit"]
pub type NAKOUTE_R = crate::BitReader<bool>;
#[doc = "Field `NAKOUTE` writer - NAK OUT Interrupt Enable Bit"]
pub type NAKOUTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEIENX_SPEC, bool, O>;
#[doc = "Field `NAKINE` reader - NAK IN Interrupt Enable Bit"]
pub type NAKINE_R = crate::BitReader<bool>;
#[doc = "Field `NAKINE` writer - NAK IN Interrupt Enable Bit"]
pub type NAKINE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEIENX_SPEC, bool, O>;
#[doc = "Field `FLERRE` reader - Flow Error Interrupt Enable Flag"]
pub type FLERRE_R = crate::BitReader<bool>;
#[doc = "Field `FLERRE` writer - Flow Error Interrupt Enable Flag"]
pub type FLERRE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UEIENX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmitter Ready Interrupt Enable Flag"]
    #[inline(always)]
    pub fn txine(&self) -> TXINE_R {
        TXINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stalled Interrupt Enable Flag"]
    #[inline(always)]
    pub fn stallede(&self) -> STALLEDE_R {
        STALLEDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received OUT Data Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rxoute(&self) -> RXOUTE_R {
        RXOUTE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Received SETUP Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rxstpe(&self) -> RXSTPE_R {
        RXSTPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK OUT Interrupt Enable Bit"]
    #[inline(always)]
    pub fn nakoute(&self) -> NAKOUTE_R {
        NAKOUTE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - NAK IN Interrupt Enable Bit"]
    #[inline(always)]
    pub fn nakine(&self) -> NAKINE_R {
        NAKINE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Flow Error Interrupt Enable Flag"]
    #[inline(always)]
    pub fn flerre(&self) -> FLERRE_R {
        FLERRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter Ready Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txine(&mut self) -> TXINE_W<0> {
        TXINE_W::new(self)
    }
    #[doc = "Bit 1 - Stalled Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn stallede(&mut self) -> STALLEDE_W<1> {
        STALLEDE_W::new(self)
    }
    #[doc = "Bit 2 - Received OUT Data Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxoute(&mut self) -> RXOUTE_W<2> {
        RXOUTE_W::new(self)
    }
    #[doc = "Bit 3 - Received SETUP Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpe(&mut self) -> RXSTPE_W<3> {
        RXSTPE_W::new(self)
    }
    #[doc = "Bit 4 - NAK OUT Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn nakoute(&mut self) -> NAKOUTE_W<4> {
        NAKOUTE_W::new(self)
    }
    #[doc = "Bit 6 - NAK IN Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn nakine(&mut self) -> NAKINE_W<6> {
        NAKINE_W::new(self)
    }
    #[doc = "Bit 7 - Flow Error Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flerre(&mut self) -> FLERRE_W<7> {
        FLERRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ueienx](index.html) module"]
pub struct UEIENX_SPEC;
impl crate::RegisterSpec for UEIENX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ueienx::R](R) reader structure"]
impl crate::Readable for UEIENX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ueienx::W](W) writer structure"]
impl crate::Writable for UEIENX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEIENX to value 0"]
impl crate::Resettable for UEIENX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
