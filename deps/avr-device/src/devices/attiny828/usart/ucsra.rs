#[doc = "Register `UCSRA` reader"]
pub struct R(crate::R<UCSRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSRA` writer"]
pub struct W(crate::W<UCSRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSRA_SPEC>;
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
impl From<crate::W<UCSRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPCM` reader - Multi-processor Communication Mode"]
pub type MPCM_R = crate::BitReader<bool>;
#[doc = "Field `MPCM` writer - Multi-processor Communication Mode"]
pub type MPCM_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRA_SPEC, bool, O>;
#[doc = "Field `U2X` reader - Double the USART transmission speed"]
pub type U2X_R = crate::BitReader<bool>;
#[doc = "Field `U2X` writer - Double the USART transmission speed"]
pub type U2X_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRA_SPEC, bool, O>;
#[doc = "Field `UPE` reader - Parity Error"]
pub type UPE_R = crate::BitReader<bool>;
#[doc = "Field `DOR` reader - Data overRun"]
pub type DOR_R = crate::BitReader<bool>;
#[doc = "Field `FE` reader - Framing Error"]
pub type FE_R = crate::BitReader<bool>;
#[doc = "Field `UDRE` reader - USART Data Register Empty"]
pub type UDRE_R = crate::BitReader<bool>;
#[doc = "Field `TXC` reader - USART Transmit Complete"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - USART Transmit Complete"]
pub type TXC_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRA_SPEC, bool, O>;
#[doc = "Field `RXC` reader - USART Receive Complete"]
pub type RXC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Multi-processor Communication Mode"]
    #[inline(always)]
    pub fn mpcm(&self) -> MPCM_R {
        MPCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Double the USART transmission speed"]
    #[inline(always)]
    pub fn u2x(&self) -> U2X_R {
        U2X_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error"]
    #[inline(always)]
    pub fn upe(&self) -> UPE_R {
        UPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data overRun"]
    #[inline(always)]
    pub fn dor(&self) -> DOR_R {
        DOR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Framing Error"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART Data Register Empty"]
    #[inline(always)]
    pub fn udre(&self) -> UDRE_R {
        UDRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART Receive Complete"]
    #[inline(always)]
    pub fn rxc(&self) -> RXC_R {
        RXC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Multi-processor Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpcm(&mut self) -> MPCM_W<0> {
        MPCM_W::new(self)
    }
    #[doc = "Bit 1 - Double the USART transmission speed"]
    #[inline(always)]
    #[must_use]
    pub fn u2x(&mut self) -> U2X_W<1> {
        U2X_W::new(self)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<6> {
        TXC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Control and Status Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsra](index.html) module"]
pub struct UCSRA_SPEC;
impl crate::RegisterSpec for UCSRA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsra::R](R) reader structure"]
impl crate::Readable for UCSRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsra::W](W) writer structure"]
impl crate::Writable for UCSRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSRA to value 0"]
impl crate::Resettable for UCSRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
