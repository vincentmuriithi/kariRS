#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTMODE` reader - Timer Mode"]
pub type CNTMODE_R = crate::FieldReader<u8, CNTMODE_A>;
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTMODE_A {
    #[doc = "0: Periodic Interrupt"]
    INT = 0,
    #[doc = "1: Periodic Timeout"]
    TIMEOUT = 1,
    #[doc = "2: Input Capture Event"]
    CAPT = 2,
    #[doc = "3: Input Capture Frequency measurement"]
    FRQ = 3,
    #[doc = "4: Input Capture Pulse-Width measurement"]
    PW = 4,
    #[doc = "5: Input Capture Frequency and Pulse-Width measurement"]
    FRQPW = 5,
    #[doc = "6: Single Shot"]
    SINGLE = 6,
    #[doc = "7: 8-bit PWM"]
    PWM8 = 7,
}
impl From<CNTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTMODE_A) -> Self {
        variant as _
    }
}
impl CNTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTMODE_A {
        match self.bits {
            0 => CNTMODE_A::INT,
            1 => CNTMODE_A::TIMEOUT,
            2 => CNTMODE_A::CAPT,
            3 => CNTMODE_A::FRQ,
            4 => CNTMODE_A::PW,
            5 => CNTMODE_A::FRQPW,
            6 => CNTMODE_A::SINGLE,
            7 => CNTMODE_A::PWM8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == CNTMODE_A::INT
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == CNTMODE_A::TIMEOUT
    }
    #[doc = "Checks if the value of the field is `CAPT`"]
    #[inline(always)]
    pub fn is_capt(&self) -> bool {
        *self == CNTMODE_A::CAPT
    }
    #[doc = "Checks if the value of the field is `FRQ`"]
    #[inline(always)]
    pub fn is_frq(&self) -> bool {
        *self == CNTMODE_A::FRQ
    }
    #[doc = "Checks if the value of the field is `PW`"]
    #[inline(always)]
    pub fn is_pw(&self) -> bool {
        *self == CNTMODE_A::PW
    }
    #[doc = "Checks if the value of the field is `FRQPW`"]
    #[inline(always)]
    pub fn is_frqpw(&self) -> bool {
        *self == CNTMODE_A::FRQPW
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CNTMODE_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `PWM8`"]
    #[inline(always)]
    pub fn is_pwm8(&self) -> bool {
        *self == CNTMODE_A::PWM8
    }
}
#[doc = "Field `CNTMODE` writer - Timer Mode"]
pub type CNTMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CTRLB_SPEC, u8, CNTMODE_A, 3, O>;
impl<'a, const O: u8> CNTMODE_W<'a, O> {
    #[doc = "Periodic Interrupt"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(CNTMODE_A::INT)
    }
    #[doc = "Periodic Timeout"]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut W {
        self.variant(CNTMODE_A::TIMEOUT)
    }
    #[doc = "Input Capture Event"]
    #[inline(always)]
    pub fn capt(self) -> &'a mut W {
        self.variant(CNTMODE_A::CAPT)
    }
    #[doc = "Input Capture Frequency measurement"]
    #[inline(always)]
    pub fn frq(self) -> &'a mut W {
        self.variant(CNTMODE_A::FRQ)
    }
    #[doc = "Input Capture Pulse-Width measurement"]
    #[inline(always)]
    pub fn pw(self) -> &'a mut W {
        self.variant(CNTMODE_A::PW)
    }
    #[doc = "Input Capture Frequency and Pulse-Width measurement"]
    #[inline(always)]
    pub fn frqpw(self) -> &'a mut W {
        self.variant(CNTMODE_A::FRQPW)
    }
    #[doc = "Single Shot"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CNTMODE_A::SINGLE)
    }
    #[doc = "8-bit PWM"]
    #[inline(always)]
    pub fn pwm8(self) -> &'a mut W {
        self.variant(CNTMODE_A::PWM8)
    }
}
#[doc = "Field `CCMPEN` reader - Pin Output Enable"]
pub type CCMPEN_R = crate::BitReader<bool>;
#[doc = "Field `CCMPEN` writer - Pin Output Enable"]
pub type CCMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `CCMPINIT` reader - Pin Initial State"]
pub type CCMPINIT_R = crate::BitReader<bool>;
#[doc = "Field `CCMPINIT` writer - Pin Initial State"]
pub type CCMPINIT_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `TCB_ASYNC` reader - Asynchronous Enable"]
pub type TCB_ASYNC_R = crate::BitReader<bool>;
#[doc = "Field `TCB_ASYNC` writer - Asynchronous Enable"]
pub type TCB_ASYNC_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Timer Mode"]
    #[inline(always)]
    pub fn cntmode(&self) -> CNTMODE_R {
        CNTMODE_R::new(self.bits & 7)
    }
    #[doc = "Bit 4 - Pin Output Enable"]
    #[inline(always)]
    pub fn ccmpen(&self) -> CCMPEN_R {
        CCMPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin Initial State"]
    #[inline(always)]
    pub fn ccmpinit(&self) -> CCMPINIT_R {
        CCMPINIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Asynchronous Enable"]
    #[inline(always)]
    pub fn tcb_async(&self) -> TCB_ASYNC_R {
        TCB_ASYNC_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cntmode(&mut self) -> CNTMODE_W<0> {
        CNTMODE_W::new(self)
    }
    #[doc = "Bit 4 - Pin Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccmpen(&mut self) -> CCMPEN_W<4> {
        CCMPEN_W::new(self)
    }
    #[doc = "Bit 5 - Pin Initial State"]
    #[inline(always)]
    #[must_use]
    pub fn ccmpinit(&mut self) -> CCMPINIT_W<5> {
        CCMPINIT_W::new(self)
    }
    #[doc = "Bit 6 - Asynchronous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcb_async(&mut self) -> TCB_ASYNC_W<6> {
        TCB_ASYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
