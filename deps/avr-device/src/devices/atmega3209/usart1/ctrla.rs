#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS485` reader - RS485 Mode internal transmitter"]
pub type RS485_R = crate::FieldReader<u8, RS485_A>;
#[doc = "RS485 Mode internal transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RS485_A {
    #[doc = "0: RS485 Mode disabled"]
    OFF = 0,
    #[doc = "1: RS485 Mode External drive"]
    EXT = 1,
    #[doc = "2: RS485 Mode Internal drive"]
    INT = 2,
}
impl From<RS485_A> for u8 {
    #[inline(always)]
    fn from(variant: RS485_A) -> Self {
        variant as _
    }
}
impl RS485_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RS485_A> {
        match self.bits {
            0 => Some(RS485_A::OFF),
            1 => Some(RS485_A::EXT),
            2 => Some(RS485_A::INT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RS485_A::OFF
    }
    #[doc = "Checks if the value of the field is `EXT`"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == RS485_A::EXT
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == RS485_A::INT
    }
}
#[doc = "Field `RS485` writer - RS485 Mode internal transmitter"]
pub type RS485_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLA_SPEC, u8, RS485_A, 2, O>;
impl<'a, const O: u8> RS485_W<'a, O> {
    #[doc = "RS485 Mode disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(RS485_A::OFF)
    }
    #[doc = "RS485 Mode External drive"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut W {
        self.variant(RS485_A::EXT)
    }
    #[doc = "RS485 Mode Internal drive"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(RS485_A::INT)
    }
}
#[doc = "Field `ABEIE` reader - Auto-baud Error Interrupt Enable"]
pub type ABEIE_R = crate::BitReader<bool>;
#[doc = "Field `ABEIE` writer - Auto-baud Error Interrupt Enable"]
pub type ABEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `LBME` reader - Loop-back Mode Enable"]
pub type LBME_R = crate::BitReader<bool>;
#[doc = "Field `LBME` writer - Loop-back Mode Enable"]
pub type LBME_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `RXSIE` reader - Receiver Start Frame Interrupt Enable"]
pub type RXSIE_R = crate::BitReader<bool>;
#[doc = "Field `RXSIE` writer - Receiver Start Frame Interrupt Enable"]
pub type RXSIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `DREIE` reader - Data Register Empty Interrupt Enable"]
pub type DREIE_R = crate::BitReader<bool>;
#[doc = "Field `DREIE` writer - Data Register Empty Interrupt Enable"]
pub type DREIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `TXCIE` reader - Transmit Complete Interrupt Enable"]
pub type TXCIE_R = crate::BitReader<bool>;
#[doc = "Field `TXCIE` writer - Transmit Complete Interrupt Enable"]
pub type TXCIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `RXCIE` reader - Receive Complete Interrupt Enable"]
pub type RXCIE_R = crate::BitReader<bool>;
#[doc = "Field `RXCIE` writer - Receive Complete Interrupt Enable"]
pub type RXCIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - RS485 Mode internal transmitter"]
    #[inline(always)]
    pub fn rs485(&self) -> RS485_R {
        RS485_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Auto-baud Error Interrupt Enable"]
    #[inline(always)]
    pub fn abeie(&self) -> ABEIE_R {
        ABEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Loop-back Mode Enable"]
    #[inline(always)]
    pub fn lbme(&self) -> LBME_R {
        LBME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receiver Start Frame Interrupt Enable"]
    #[inline(always)]
    pub fn rxsie(&self) -> RXSIE_R {
        RXSIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn dreie(&self) -> DREIE_R {
        DREIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Complete Interrupt Enable"]
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
    #[doc = "Bits 0:1 - RS485 Mode internal transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn rs485(&mut self) -> RS485_W<0> {
        RS485_W::new(self)
    }
    #[doc = "Bit 2 - Auto-baud Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn abeie(&mut self) -> ABEIE_W<2> {
        ABEIE_W::new(self)
    }
    #[doc = "Bit 3 - Loop-back Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbme(&mut self) -> LBME_W<3> {
        LBME_W::new(self)
    }
    #[doc = "Bit 4 - Receiver Start Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxsie(&mut self) -> RXSIE_W<4> {
        RXSIE_W::new(self)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dreie(&mut self) -> DREIE_W<5> {
        DREIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Complete Interrupt Enable"]
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
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
