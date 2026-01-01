#[doc = "Register `INTCTRL` reader"]
pub struct R(crate::R<INTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCTRL` writer"]
pub struct W(crate::W<INTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCTRL_SPEC>;
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
impl From<crate::W<INTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `SSIE` reader - Client Select Trigger Interrupt Enable"]
pub type SSIE_R = crate::BitReader<bool>;
#[doc = "Field `SSIE` writer - Client Select Trigger Interrupt Enable"]
pub type SSIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `DREIE` reader - Data Register Empty Interrupt Enable"]
pub type DREIE_R = crate::BitReader<bool>;
#[doc = "Field `DREIE` writer - Data Register Empty Interrupt Enable"]
pub type DREIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `TXCIE` reader - Transfer Complete Interrupt Enable"]
pub type TXCIE_R = crate::BitReader<bool>;
#[doc = "Field `TXCIE` writer - Transfer Complete Interrupt Enable"]
pub type TXCIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `RXCIE` reader - Receive Complete Interrupt Enable"]
pub type RXCIE_R = crate::BitReader<bool>;
#[doc = "Field `RXCIE` writer - Receive Complete Interrupt Enable"]
pub type RXCIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Client Select Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&self) -> SSIE_R {
        SSIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn dreie(&self) -> DREIE_R {
        DREIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txcie(&self) -> TXCIE_R {
        TXCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxcie(&self) -> RXCIE_R {
        RXCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<0> {
        IE_W::new(self)
    }
    #[doc = "Bit 4 - Client Select Trigger Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssie(&mut self) -> SSIE_W<4> {
        SSIE_W::new(self)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dreie(&mut self) -> DREIE_W<5> {
        DREIE_W::new(self)
    }
    #[doc = "Bit 6 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcie(&mut self) -> TXCIE_W<6> {
        TXCIE_W::new(self)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcie(&mut self) -> RXCIE_W<7> {
        RXCIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intctrl](index.html) module"]
pub struct INTCTRL_SPEC;
impl crate::RegisterSpec for INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intctrl::R](R) reader structure"]
impl crate::Readable for INTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intctrl::W](W) writer structure"]
impl crate::Writable for INTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::Resettable for INTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
