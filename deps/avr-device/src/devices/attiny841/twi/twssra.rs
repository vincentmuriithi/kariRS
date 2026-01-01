#[doc = "Register `TWSSRA` reader"]
pub struct R(crate::R<TWSSRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWSSRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWSSRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWSSRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWSSRA` writer"]
pub struct W(crate::W<TWSSRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWSSRA_SPEC>;
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
impl From<crate::W<TWSSRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWSSRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWAS` reader - TWI Address or Stop"]
pub type TWAS_R = crate::BitReader<bool>;
#[doc = "Field `TWAS` writer - TWI Address or Stop"]
pub type TWAS_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWSSRA_SPEC, bool, O>;
#[doc = "Field `TWDIR` reader - TWI Read/Write Direction"]
pub type TWDIR_R = crate::BitReader<bool>;
#[doc = "Field `TWDIR` writer - TWI Read/Write Direction"]
pub type TWDIR_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWSSRA_SPEC, bool, O>;
#[doc = "Field `TWBE` reader - TWI Bus Error"]
pub type TWBE_R = crate::BitReader<bool>;
#[doc = "Field `TWBE` writer - TWI Bus Error"]
pub type TWBE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWSSRA_SPEC, bool, O>;
#[doc = "Field `TWC` reader - TWI Collision"]
pub type TWC_R = crate::BitReader<bool>;
#[doc = "Field `TWC` writer - TWI Collision"]
pub type TWC_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWSSRA_SPEC, bool, O>;
#[doc = "Field `TWRA` reader - TWI Receive Acknowledge"]
pub type TWRA_R = crate::BitReader<bool>;
#[doc = "Field `TWRA` writer - TWI Receive Acknowledge"]
pub type TWRA_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWSSRA_SPEC, bool, O>;
#[doc = "Field `TWCH` reader - TWI Clock Hold"]
pub type TWCH_R = crate::BitReader<bool>;
#[doc = "Field `TWCH` writer - TWI Clock Hold"]
pub type TWCH_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWSSRA_SPEC, bool, O>;
#[doc = "Field `TWASIF` reader - TWI Address/Stop Interrupt Flag"]
pub type TWASIF_R = crate::BitReader<bool>;
#[doc = "Field `TWASIF` writer - TWI Address/Stop Interrupt Flag"]
pub type TWASIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWSSRA_SPEC, bool, O>;
#[doc = "Field `TWDIF` reader - TWI Data Interrupt Flag."]
pub type TWDIF_R = crate::BitReader<bool>;
#[doc = "Field `TWDIF` writer - TWI Data Interrupt Flag."]
pub type TWDIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWSSRA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TWI Address or Stop"]
    #[inline(always)]
    pub fn twas(&self) -> TWAS_R {
        TWAS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TWI Read/Write Direction"]
    #[inline(always)]
    pub fn twdir(&self) -> TWDIR_R {
        TWDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TWI Bus Error"]
    #[inline(always)]
    pub fn twbe(&self) -> TWBE_R {
        TWBE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TWI Collision"]
    #[inline(always)]
    pub fn twc(&self) -> TWC_R {
        TWC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TWI Receive Acknowledge"]
    #[inline(always)]
    pub fn twra(&self) -> TWRA_R {
        TWRA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TWI Clock Hold"]
    #[inline(always)]
    pub fn twch(&self) -> TWCH_R {
        TWCH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TWI Address/Stop Interrupt Flag"]
    #[inline(always)]
    pub fn twasif(&self) -> TWASIF_R {
        TWASIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TWI Data Interrupt Flag."]
    #[inline(always)]
    pub fn twdif(&self) -> TWDIF_R {
        TWDIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI Address or Stop"]
    #[inline(always)]
    #[must_use]
    pub fn twas(&mut self) -> TWAS_W<0> {
        TWAS_W::new(self)
    }
    #[doc = "Bit 1 - TWI Read/Write Direction"]
    #[inline(always)]
    #[must_use]
    pub fn twdir(&mut self) -> TWDIR_W<1> {
        TWDIR_W::new(self)
    }
    #[doc = "Bit 2 - TWI Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn twbe(&mut self) -> TWBE_W<2> {
        TWBE_W::new(self)
    }
    #[doc = "Bit 3 - TWI Collision"]
    #[inline(always)]
    #[must_use]
    pub fn twc(&mut self) -> TWC_W<3> {
        TWC_W::new(self)
    }
    #[doc = "Bit 4 - TWI Receive Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn twra(&mut self) -> TWRA_W<4> {
        TWRA_W::new(self)
    }
    #[doc = "Bit 5 - TWI Clock Hold"]
    #[inline(always)]
    #[must_use]
    pub fn twch(&mut self) -> TWCH_W<5> {
        TWCH_W::new(self)
    }
    #[doc = "Bit 6 - TWI Address/Stop Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn twasif(&mut self) -> TWASIF_W<6> {
        TWASIF_W::new(self)
    }
    #[doc = "Bit 7 - TWI Data Interrupt Flag."]
    #[inline(always)]
    #[must_use]
    pub fn twdif(&mut self) -> TWDIF_W<7> {
        TWDIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Slave Status Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twssra](index.html) module"]
pub struct TWSSRA_SPEC;
impl crate::RegisterSpec for TWSSRA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twssra::R](R) reader structure"]
impl crate::Readable for TWSSRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twssra::W](W) writer structure"]
impl crate::Writable for TWSSRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWSSRA to value 0"]
impl crate::Resettable for TWSSRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
