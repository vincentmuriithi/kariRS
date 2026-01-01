#[doc = "Register `REMAP` reader"]
pub struct R(crate::R<REMAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REMAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REMAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REMAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REMAP` writer"]
pub struct W(crate::W<REMAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REMAP_SPEC>;
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
impl From<crate::W<REMAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REMAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIMAP` reader - SPI Pin Mapping"]
pub type SPIMAP_R = crate::BitReader<bool>;
#[doc = "Field `SPIMAP` writer - SPI Pin Mapping"]
pub type SPIMAP_W<'a, const O: u8> = crate::BitWriter<'a, u8, REMAP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SPI Pin Mapping"]
    #[inline(always)]
    pub fn spimap(&self) -> SPIMAP_R {
        SPIMAP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SPI Pin Mapping"]
    #[inline(always)]
    #[must_use]
    pub fn spimap(&mut self) -> SPIMAP_W<1> {
        SPIMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Remap Port Pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remap](index.html) module"]
pub struct REMAP_SPEC;
impl crate::RegisterSpec for REMAP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [remap::R](R) reader structure"]
impl crate::Readable for REMAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [remap::W](W) writer structure"]
impl crate::Writable for REMAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP to value 0"]
impl crate::Resettable for REMAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
