#[doc = "Register `PLL_DCU` reader"]
pub struct R(crate::R<PLL_DCU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_DCU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_DCU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_DCU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_DCU` writer"]
pub struct W(crate::W<PLL_DCU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_DCU_SPEC>;
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
impl From<crate::W<PLL_DCU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_DCU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_DCU_START` reader - Start Delay Cell Calibration"]
pub type PLL_DCU_START_R = crate::BitReader<bool>;
#[doc = "Field `PLL_DCU_START` writer - Start Delay Cell Calibration"]
pub type PLL_DCU_START_W<'a, const O: u8> = crate::BitWriter<'a, u8, PLL_DCU_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - Start Delay Cell Calibration"]
    #[inline(always)]
    pub fn pll_dcu_start(&self) -> PLL_DCU_START_R {
        PLL_DCU_START_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Start Delay Cell Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn pll_dcu_start(&mut self) -> PLL_DCU_START_W<7> {
        PLL_DCU_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver Delay Cell Calibration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_dcu](index.html) module"]
pub struct PLL_DCU_SPEC;
impl crate::RegisterSpec for PLL_DCU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pll_dcu::R](R) reader structure"]
impl crate::Readable for PLL_DCU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_dcu::W](W) writer structure"]
impl crate::Writable for PLL_DCU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_DCU to value 0"]
impl crate::Resettable for PLL_DCU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
