#[doc = "Register `MCTRLB` reader"]
pub struct R(crate::R<MCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTRLB` writer"]
pub struct W(crate::W<MCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTRLB_SPEC>;
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
impl From<crate::W<MCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCMD` reader - Command"]
pub type MCMD_R = crate::FieldReader<u8, MCMD_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCMD_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: Execute Acknowledge Action followed by repeated Start."]
    REPSTART = 1,
    #[doc = "2: Execute Acknowledge Action followed by a byte read/write operation. Read/write is defined by DIR."]
    RECVTRANS = 2,
    #[doc = "3: Execute Acknowledge Action followed by issuing a Stop condition."]
    STOP = 3,
}
impl From<MCMD_A> for u8 {
    #[inline(always)]
    fn from(variant: MCMD_A) -> Self {
        variant as _
    }
}
impl MCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCMD_A {
        match self.bits {
            0 => MCMD_A::NOACT,
            1 => MCMD_A::REPSTART,
            2 => MCMD_A::RECVTRANS,
            3 => MCMD_A::STOP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == MCMD_A::NOACT
    }
    #[doc = "Checks if the value of the field is `REPSTART`"]
    #[inline(always)]
    pub fn is_repstart(&self) -> bool {
        *self == MCMD_A::REPSTART
    }
    #[doc = "Checks if the value of the field is `RECVTRANS`"]
    #[inline(always)]
    pub fn is_recvtrans(&self) -> bool {
        *self == MCMD_A::RECVTRANS
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == MCMD_A::STOP
    }
}
#[doc = "Field `MCMD` writer - Command"]
pub type MCMD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MCTRLB_SPEC, u8, MCMD_A, 2, O>;
impl<'a, const O: u8> MCMD_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut W {
        self.variant(MCMD_A::NOACT)
    }
    #[doc = "Execute Acknowledge Action followed by repeated Start."]
    #[inline(always)]
    pub fn repstart(self) -> &'a mut W {
        self.variant(MCMD_A::REPSTART)
    }
    #[doc = "Execute Acknowledge Action followed by a byte read/write operation. Read/write is defined by DIR."]
    #[inline(always)]
    pub fn recvtrans(self) -> &'a mut W {
        self.variant(MCMD_A::RECVTRANS)
    }
    #[doc = "Execute Acknowledge Action followed by issuing a Stop condition."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(MCMD_A::STOP)
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
pub type ACKACT_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTRLB_SPEC, ACKACT_A, O>;
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
#[doc = "Field `FLUSH` reader - Flush"]
pub type FLUSH_R = crate::BitReader<bool>;
#[doc = "Field `FLUSH` writer - Flush"]
pub type FLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Command"]
    #[inline(always)]
    pub fn mcmd(&self) -> MCMD_R {
        MCMD_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Acknowledge Action"]
    #[inline(always)]
    pub fn ackact(&self) -> ACKACT_R {
        ACKACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flush"]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn mcmd(&mut self) -> MCMD_W<0> {
        MCMD_W::new(self)
    }
    #[doc = "Bit 2 - Acknowledge Action"]
    #[inline(always)]
    #[must_use]
    pub fn ackact(&mut self) -> ACKACT_W<2> {
        ACKACT_W::new(self)
    }
    #[doc = "Bit 3 - Flush"]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<3> {
        FLUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrlb](index.html) module"]
pub struct MCTRLB_SPEC;
impl crate::RegisterSpec for MCTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mctrlb::R](R) reader structure"]
impl crate::Readable for MCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctrlb::W](W) writer structure"]
impl crate::Writable for MCTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTRLB to value 0"]
impl crate::Resettable for MCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
