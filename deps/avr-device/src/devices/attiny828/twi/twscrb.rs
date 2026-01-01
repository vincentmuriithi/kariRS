#[doc = "Register `TWSCRB` reader"]
pub struct R(crate::R<TWSCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWSCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWSCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWSCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWSCRB` writer"]
pub struct W(crate::W<TWSCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWSCRB_SPEC>;
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
impl From<crate::W<TWSCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWSCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWCMD` reader - No Description."]
pub type TWCMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWCMD` writer - No Description."]
pub type TWCMD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TWSCRB_SPEC, u8, u8, 2, O>;
#[doc = "Field `TWAA` reader - TWI Acknowledge Action"]
pub type TWAA_R = crate::BitReader<bool>;
#[doc = "Field `TWAA` writer - TWI Acknowledge Action"]
pub type TWAA_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWSCRB_SPEC, bool, O>;
#[doc = "Field `TWHNM` reader - TWI High Noise Mode"]
pub type TWHNM_R = crate::BitReader<bool>;
#[doc = "Field `TWHNM` writer - TWI High Noise Mode"]
pub type TWHNM_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWSCRB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - No Description."]
    #[inline(always)]
    pub fn twcmd(&self) -> TWCMD_R {
        TWCMD_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - TWI Acknowledge Action"]
    #[inline(always)]
    pub fn twaa(&self) -> TWAA_R {
        TWAA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TWI High Noise Mode"]
    #[inline(always)]
    pub fn twhnm(&self) -> TWHNM_R {
        TWHNM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn twcmd(&mut self) -> TWCMD_W<0> {
        TWCMD_W::new(self)
    }
    #[doc = "Bit 2 - TWI Acknowledge Action"]
    #[inline(always)]
    #[must_use]
    pub fn twaa(&mut self) -> TWAA_W<2> {
        TWAA_W::new(self)
    }
    #[doc = "Bit 3 - TWI High Noise Mode"]
    #[inline(always)]
    #[must_use]
    pub fn twhnm(&mut self) -> TWHNM_W<3> {
        TWHNM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Slave Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twscrb](index.html) module"]
pub struct TWSCRB_SPEC;
impl crate::RegisterSpec for TWSCRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twscrb::R](R) reader structure"]
impl crate::Readable for TWSCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twscrb::W](W) writer structure"]
impl crate::Writable for TWSCRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWSCRB to value 0"]
impl crate::Resettable for TWSCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
