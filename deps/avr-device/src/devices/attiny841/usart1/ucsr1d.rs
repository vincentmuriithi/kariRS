#[doc = "Register `UCSR1D` reader"]
pub struct R(crate::R<UCSR1D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSR1D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSR1D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSR1D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSR1D` writer"]
pub struct W(crate::W<UCSR1D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSR1D_SPEC>;
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
impl From<crate::W<UCSR1D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSR1D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFDE1` reader - USART RX Start Frame Detection Enable"]
pub type SFDE1_R = crate::BitReader<bool>;
#[doc = "Field `SFDE1` writer - USART RX Start Frame Detection Enable"]
pub type SFDE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR1D_SPEC, bool, O>;
#[doc = "Field `RXS1` reader - USART RX Start Flag"]
pub type RXS1_R = crate::BitReader<bool>;
#[doc = "Field `RXS1` writer - USART RX Start Flag"]
pub type RXS1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR1D_SPEC, bool, O>;
#[doc = "Field `RXSIE1` reader - USART RX Start Interrupt Enable"]
pub type RXSIE1_R = crate::BitReader<bool>;
#[doc = "Field `RXSIE1` writer - USART RX Start Interrupt Enable"]
pub type RXSIE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR1D_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - USART RX Start Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde1(&self) -> SFDE1_R {
        SFDE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART RX Start Flag"]
    #[inline(always)]
    pub fn rxs1(&self) -> RXS1_R {
        RXS1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART RX Start Interrupt Enable"]
    #[inline(always)]
    pub fn rxsie1(&self) -> RXSIE1_R {
        RXSIE1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - USART RX Start Frame Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfde1(&mut self) -> SFDE1_W<5> {
        SFDE1_W::new(self)
    }
    #[doc = "Bit 6 - USART RX Start Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxs1(&mut self) -> RXS1_W<6> {
        RXS1_W::new(self)
    }
    #[doc = "Bit 7 - USART RX Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxsie1(&mut self) -> RXSIE1_W<7> {
        RXSIE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Control and Status Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsr1d](index.html) module"]
pub struct UCSR1D_SPEC;
impl crate::RegisterSpec for UCSR1D_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsr1d::R](R) reader structure"]
impl crate::Readable for UCSR1D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsr1d::W](W) writer structure"]
impl crate::Writable for UCSR1D_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR1D to value 0"]
impl crate::Resettable for UCSR1D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
