#[doc = "Register `UCSRD` reader"]
pub struct R(crate::R<UCSRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSRD` writer"]
pub struct W(crate::W<UCSRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSRD_SPEC>;
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
impl From<crate::W<UCSRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFDE` reader - USART RX Start Frame Detection Enable"]
pub type SFDE_R = crate::BitReader<bool>;
#[doc = "Field `SFDE` writer - USART RX Start Frame Detection Enable"]
pub type SFDE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRD_SPEC, bool, O>;
#[doc = "Field `RXS` reader - USART RX Start Flag"]
pub type RXS_R = crate::BitReader<bool>;
#[doc = "Field `RXS` writer - USART RX Start Flag"]
pub type RXS_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRD_SPEC, bool, O>;
#[doc = "Field `RXSIE` reader - USART RX Start Interrupt Enable"]
pub type RXSIE_R = crate::BitReader<bool>;
#[doc = "Field `RXSIE` writer - USART RX Start Interrupt Enable"]
pub type RXSIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - USART RX Start Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde(&self) -> SFDE_R {
        SFDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART RX Start Flag"]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART RX Start Interrupt Enable"]
    #[inline(always)]
    pub fn rxsie(&self) -> RXSIE_R {
        RXSIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - USART RX Start Frame Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfde(&mut self) -> SFDE_W<5> {
        SFDE_W::new(self)
    }
    #[doc = "Bit 6 - USART RX Start Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxs(&mut self) -> RXS_W<6> {
        RXS_W::new(self)
    }
    #[doc = "Bit 7 - USART RX Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxsie(&mut self) -> RXSIE_W<7> {
        RXSIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Control and Status Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsrd](index.html) module"]
pub struct UCSRD_SPEC;
impl crate::RegisterSpec for UCSRD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsrd::R](R) reader structure"]
impl crate::Readable for UCSRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsrd::W](W) writer structure"]
impl crate::Writable for UCSRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSRD to value 0"]
impl crate::Resettable for UCSRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
