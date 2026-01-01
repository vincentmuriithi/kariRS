#[doc = "Register `UCSR0D` reader"]
pub struct R(crate::R<UCSR0D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSR0D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSR0D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSR0D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSR0D` writer"]
pub struct W(crate::W<UCSR0D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSR0D_SPEC>;
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
impl From<crate::W<UCSR0D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSR0D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFDE` reader - Start frame detection enable"]
pub type SFDE_R = crate::BitReader<bool>;
#[doc = "Field `SFDE` writer - Start frame detection enable"]
pub type SFDE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0D_SPEC, bool, O>;
#[doc = "Field `RXS` reader - USART RX Start"]
pub type RXS_R = crate::BitReader<bool>;
#[doc = "Field `RXS` writer - USART RX Start"]
pub type RXS_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0D_SPEC, bool, O>;
#[doc = "Field `RXSIE` reader - USART RX Start Interrupt Enable"]
pub type RXSIE_R = crate::BitReader<bool>;
#[doc = "Field `RXSIE` writer - USART RX Start Interrupt Enable"]
pub type RXSIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0D_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - Start frame detection enable"]
    #[inline(always)]
    pub fn sfde(&self) -> SFDE_R {
        SFDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART RX Start"]
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
    #[doc = "Bit 5 - Start frame detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfde(&mut self) -> SFDE_W<5> {
        SFDE_W::new(self)
    }
    #[doc = "Bit 6 - USART RX Start"]
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
#[doc = "USART Control and Status Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsr0d](index.html) module"]
pub struct UCSR0D_SPEC;
impl crate::RegisterSpec for UCSR0D_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsr0d::R](R) reader structure"]
impl crate::Readable for UCSR0D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsr0d::W](W) writer structure"]
impl crate::Writable for UCSR0D_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR0D to value 0"]
impl crate::Resettable for UCSR0D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
