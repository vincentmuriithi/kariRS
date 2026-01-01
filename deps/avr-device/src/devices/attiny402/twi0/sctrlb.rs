#[doc = "Register `SCTRLB` reader"]
pub struct R(crate::R<SCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTRLB` writer"]
pub struct W(crate::W<SCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTRLB_SPEC>;
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
impl From<crate::W<SCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCMD` reader - Command"]
pub type SCMD_R = crate::FieldReader<u8, SCMD_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCMD_A {
    #[doc = "0: No Action"]
    NOACT = 0,
    #[doc = "2: Used To Complete a Transaction"]
    COMPTRANS = 2,
    #[doc = "3: Used in Response to Address/Data Interrupt"]
    RESPONSE = 3,
}
impl From<SCMD_A> for u8 {
    #[inline(always)]
    fn from(variant: SCMD_A) -> Self {
        variant as _
    }
}
impl SCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCMD_A> {
        match self.bits {
            0 => Some(SCMD_A::NOACT),
            2 => Some(SCMD_A::COMPTRANS),
            3 => Some(SCMD_A::RESPONSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == SCMD_A::NOACT
    }
    #[doc = "Checks if the value of the field is `COMPTRANS`"]
    #[inline(always)]
    pub fn is_comptrans(&self) -> bool {
        *self == SCMD_A::COMPTRANS
    }
    #[doc = "Checks if the value of the field is `RESPONSE`"]
    #[inline(always)]
    pub fn is_response(&self) -> bool {
        *self == SCMD_A::RESPONSE
    }
}
#[doc = "Field `SCMD` writer - Command"]
pub type SCMD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SCTRLB_SPEC, u8, SCMD_A, 2, O>;
impl<'a, const O: u8> SCMD_W<'a, O> {
    #[doc = "No Action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut W {
        self.variant(SCMD_A::NOACT)
    }
    #[doc = "Used To Complete a Transaction"]
    #[inline(always)]
    pub fn comptrans(self) -> &'a mut W {
        self.variant(SCMD_A::COMPTRANS)
    }
    #[doc = "Used in Response to Address/Data Interrupt"]
    #[inline(always)]
    pub fn response(self) -> &'a mut W {
        self.variant(SCMD_A::RESPONSE)
    }
}
#[doc = "Field `ACKACT` reader - Acknowledge Action"]
pub type ACKACT_R = crate::BitReader<ACKACT_A>;
#[doc = "Acknowledge Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKACT_A {
    #[doc = "0: Send ACK"]
    ACK = 0,
    #[doc = "1: Send NACK"]
    NACK = 1,
}
impl From<ACKACT_A> for bool {
    #[inline(always)]
    fn from(variant: ACKACT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKACT_A {
        match self.bits {
            false => ACKACT_A::ACK,
            true => ACKACT_A::NACK,
        }
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == ACKACT_A::ACK
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == ACKACT_A::NACK
    }
}
#[doc = "Field `ACKACT` writer - Acknowledge Action"]
pub type ACKACT_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCTRLB_SPEC, ACKACT_A, O>;
impl<'a, const O: u8> ACKACT_W<'a, O> {
    #[doc = "Send ACK"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(ACKACT_A::ACK)
    }
    #[doc = "Send NACK"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(ACKACT_A::NACK)
    }
}
impl R {
    #[doc = "Bits 0:1 - Command"]
    #[inline(always)]
    pub fn scmd(&self) -> SCMD_R {
        SCMD_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Acknowledge Action"]
    #[inline(always)]
    pub fn ackact(&self) -> ACKACT_R {
        ACKACT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn scmd(&mut self) -> SCMD_W<0> {
        SCMD_W::new(self)
    }
    #[doc = "Bit 2 - Acknowledge Action"]
    #[inline(always)]
    #[must_use]
    pub fn ackact(&mut self) -> ACKACT_W<2> {
        ACKACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Client Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctrlb](index.html) module"]
pub struct SCTRLB_SPEC;
impl crate::RegisterSpec for SCTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sctrlb::R](R) reader structure"]
impl crate::Readable for SCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctrlb::W](W) writer structure"]
impl crate::Writable for SCTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTRLB to value 0"]
impl crate::Resettable for SCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
