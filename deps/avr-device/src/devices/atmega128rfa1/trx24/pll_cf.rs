#[doc = "Register `PLL_CF` reader"]
pub struct R(crate::R<PLL_CF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_CF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_CF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_CF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_CF` writer"]
pub struct W(crate::W<PLL_CF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_CF_SPEC>;
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
impl From<crate::W<PLL_CF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_CF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_CF_START` reader - Start Center Frequency Calibration"]
pub type PLL_CF_START_R = crate::BitReader<bool>;
#[doc = "Field `PLL_CF_START` writer - Start Center Frequency Calibration"]
pub type PLL_CF_START_W<'a, const O: u8> = crate::BitWriter<'a, u8, PLL_CF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - Start Center Frequency Calibration"]
    #[inline(always)]
    pub fn pll_cf_start(&self) -> PLL_CF_START_R {
        PLL_CF_START_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Start Center Frequency Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn pll_cf_start(&mut self) -> PLL_CF_START_W<7> {
        PLL_CF_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver Center Frequency Calibration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_cf](index.html) module"]
pub struct PLL_CF_SPEC;
impl crate::RegisterSpec for PLL_CF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pll_cf::R](R) reader structure"]
impl crate::Readable for PLL_CF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_cf::W](W) writer structure"]
impl crate::Writable for PLL_CF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_CF to value 0"]
impl crate::Resettable for PLL_CF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
