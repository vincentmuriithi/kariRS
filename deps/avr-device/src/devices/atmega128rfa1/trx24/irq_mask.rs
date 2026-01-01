#[doc = "Register `IRQ_MASK` reader"]
pub struct R(crate::R<IRQ_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_MASK` writer"]
pub struct W(crate::W<IRQ_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_MASK_SPEC>;
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
impl From<crate::W<IRQ_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_LOCK_EN` reader - PLL Lock Interrupt Enable"]
pub type PLL_LOCK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PLL_LOCK_EN` writer - PLL Lock Interrupt Enable"]
pub type PLL_LOCK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, IRQ_MASK_SPEC, bool, O>;
#[doc = "Field `PLL_UNLOCK_EN` reader - PLL Unlock Interrupt Enable"]
pub type PLL_UNLOCK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PLL_UNLOCK_EN` writer - PLL Unlock Interrupt Enable"]
pub type PLL_UNLOCK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, IRQ_MASK_SPEC, bool, O>;
#[doc = "Field `RX_START_EN` reader - RX_START Interrupt Enable"]
pub type RX_START_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_START_EN` writer - RX_START Interrupt Enable"]
pub type RX_START_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, IRQ_MASK_SPEC, bool, O>;
#[doc = "Field `RX_END_EN` reader - RX_END Interrupt Enable"]
pub type RX_END_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_END_EN` writer - RX_END Interrupt Enable"]
pub type RX_END_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, IRQ_MASK_SPEC, bool, O>;
#[doc = "Field `CCA_ED_DONE_EN` reader - End of ED Measurement Interrupt Enable"]
pub type CCA_ED_DONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `CCA_ED_DONE_EN` writer - End of ED Measurement Interrupt Enable"]
pub type CCA_ED_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, IRQ_MASK_SPEC, bool, O>;
#[doc = "Field `AMI_EN` reader - Address Match Interrupt Enable"]
pub type AMI_EN_R = crate::BitReader<bool>;
#[doc = "Field `AMI_EN` writer - Address Match Interrupt Enable"]
pub type AMI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, IRQ_MASK_SPEC, bool, O>;
#[doc = "Field `TX_END_EN` reader - TX_END Interrupt Enable"]
pub type TX_END_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_END_EN` writer - TX_END Interrupt Enable"]
pub type TX_END_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, IRQ_MASK_SPEC, bool, O>;
#[doc = "Field `AWAKE_EN` reader - Awake Interrupt Enable"]
pub type AWAKE_EN_R = crate::BitReader<bool>;
#[doc = "Field `AWAKE_EN` writer - Awake Interrupt Enable"]
pub type AWAKE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, IRQ_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PLL Lock Interrupt Enable"]
    #[inline(always)]
    pub fn pll_lock_en(&self) -> PLL_LOCK_EN_R {
        PLL_LOCK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Unlock Interrupt Enable"]
    #[inline(always)]
    pub fn pll_unlock_en(&self) -> PLL_UNLOCK_EN_R {
        PLL_UNLOCK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX_START Interrupt Enable"]
    #[inline(always)]
    pub fn rx_start_en(&self) -> RX_START_EN_R {
        RX_START_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX_END Interrupt Enable"]
    #[inline(always)]
    pub fn rx_end_en(&self) -> RX_END_EN_R {
        RX_END_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of ED Measurement Interrupt Enable"]
    #[inline(always)]
    pub fn cca_ed_done_en(&self) -> CCA_ED_DONE_EN_R {
        CCA_ED_DONE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Address Match Interrupt Enable"]
    #[inline(always)]
    pub fn ami_en(&self) -> AMI_EN_R {
        AMI_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX_END Interrupt Enable"]
    #[inline(always)]
    pub fn tx_end_en(&self) -> TX_END_EN_R {
        TX_END_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Awake Interrupt Enable"]
    #[inline(always)]
    pub fn awake_en(&self) -> AWAKE_EN_R {
        AWAKE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Lock Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock_en(&mut self) -> PLL_LOCK_EN_W<0> {
        PLL_LOCK_EN_W::new(self)
    }
    #[doc = "Bit 1 - PLL Unlock Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll_unlock_en(&mut self) -> PLL_UNLOCK_EN_W<1> {
        PLL_UNLOCK_EN_W::new(self)
    }
    #[doc = "Bit 2 - RX_START Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_start_en(&mut self) -> RX_START_EN_W<2> {
        RX_START_EN_W::new(self)
    }
    #[doc = "Bit 3 - RX_END Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_end_en(&mut self) -> RX_END_EN_W<3> {
        RX_END_EN_W::new(self)
    }
    #[doc = "Bit 4 - End of ED Measurement Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cca_ed_done_en(&mut self) -> CCA_ED_DONE_EN_W<4> {
        CCA_ED_DONE_EN_W::new(self)
    }
    #[doc = "Bit 5 - Address Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ami_en(&mut self) -> AMI_EN_W<5> {
        AMI_EN_W::new(self)
    }
    #[doc = "Bit 6 - TX_END Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_end_en(&mut self) -> TX_END_EN_W<6> {
        TX_END_EN_W::new(self)
    }
    #[doc = "Bit 7 - Awake Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn awake_en(&mut self) -> AWAKE_EN_W<7> {
        AWAKE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_mask](index.html) module"]
pub struct IRQ_MASK_SPEC;
impl crate::RegisterSpec for IRQ_MASK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [irq_mask::R](R) reader structure"]
impl crate::Readable for IRQ_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_mask::W](W) writer structure"]
impl crate::Writable for IRQ_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ_MASK to value 0"]
impl crate::Resettable for IRQ_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
