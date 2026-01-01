#[doc = "Register `TWCR` reader"]
pub struct R(crate::R<TWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWCR` writer"]
pub struct W(crate::W<TWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWCR_SPEC>;
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
impl From<crate::W<TWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWIE` reader - TWI Interrupt Enable"]
pub type TWIE_R = crate::BitReader<bool>;
#[doc = "Field `TWIE` writer - TWI Interrupt Enable"]
pub type TWIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWCR_SPEC, bool, O>;
#[doc = "Field `TWEN` reader - TWI Enable Bit"]
pub type TWEN_R = crate::BitReader<bool>;
#[doc = "Field `TWEN` writer - TWI Enable Bit"]
pub type TWEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWCR_SPEC, bool, O>;
#[doc = "Field `TWWC` reader - TWI Write Collition Flag"]
pub type TWWC_R = crate::BitReader<bool>;
#[doc = "Field `TWSTO` reader - TWI Stop Condition Bit"]
pub type TWSTO_R = crate::BitReader<bool>;
#[doc = "Field `TWSTO` writer - TWI Stop Condition Bit"]
pub type TWSTO_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWCR_SPEC, bool, O>;
#[doc = "Field `TWSTA` reader - TWI Start Condition Bit"]
pub type TWSTA_R = crate::BitReader<bool>;
#[doc = "Field `TWSTA` writer - TWI Start Condition Bit"]
pub type TWSTA_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWCR_SPEC, bool, O>;
#[doc = "Field `TWEA` reader - TWI Enable Acknowledge Bit"]
pub type TWEA_R = crate::BitReader<bool>;
#[doc = "Field `TWEA` writer - TWI Enable Acknowledge Bit"]
pub type TWEA_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWCR_SPEC, bool, O>;
#[doc = "Field `TWINT` reader - TWI Interrupt Flag"]
pub type TWINT_R = crate::BitReader<bool>;
#[doc = "Field `TWINT` writer - TWI Interrupt Flag"]
pub type TWINT_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TWI Interrupt Enable"]
    #[inline(always)]
    pub fn twie(&self) -> TWIE_R {
        TWIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TWI Enable Bit"]
    #[inline(always)]
    pub fn twen(&self) -> TWEN_R {
        TWEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TWI Write Collition Flag"]
    #[inline(always)]
    pub fn twwc(&self) -> TWWC_R {
        TWWC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TWI Stop Condition Bit"]
    #[inline(always)]
    pub fn twsto(&self) -> TWSTO_R {
        TWSTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TWI Start Condition Bit"]
    #[inline(always)]
    pub fn twsta(&self) -> TWSTA_R {
        TWSTA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TWI Enable Acknowledge Bit"]
    #[inline(always)]
    pub fn twea(&self) -> TWEA_R {
        TWEA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TWI Interrupt Flag"]
    #[inline(always)]
    pub fn twint(&self) -> TWINT_R {
        TWINT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twie(&mut self) -> TWIE_W<0> {
        TWIE_W::new(self)
    }
    #[doc = "Bit 2 - TWI Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn twen(&mut self) -> TWEN_W<2> {
        TWEN_W::new(self)
    }
    #[doc = "Bit 4 - TWI Stop Condition Bit"]
    #[inline(always)]
    #[must_use]
    pub fn twsto(&mut self) -> TWSTO_W<4> {
        TWSTO_W::new(self)
    }
    #[doc = "Bit 5 - TWI Start Condition Bit"]
    #[inline(always)]
    #[must_use]
    pub fn twsta(&mut self) -> TWSTA_W<5> {
        TWSTA_W::new(self)
    }
    #[doc = "Bit 6 - TWI Enable Acknowledge Bit"]
    #[inline(always)]
    #[must_use]
    pub fn twea(&mut self) -> TWEA_W<6> {
        TWEA_W::new(self)
    }
    #[doc = "Bit 7 - TWI Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn twint(&mut self) -> TWINT_W<7> {
        TWINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twcr](index.html) module"]
pub struct TWCR_SPEC;
impl crate::RegisterSpec for TWCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twcr::R](R) reader structure"]
impl crate::Readable for TWCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twcr::W](W) writer structure"]
impl crate::Writable for TWCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWCR to value 0"]
impl crate::Resettable for TWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
