#[doc = "Register `MCUCR` reader"]
pub struct R(crate::R<MCUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUCR` writer"]
pub struct W(crate::W<MCUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUCR_SPEC>;
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
impl From<crate::W<MCUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISC0` reader - Interrupt Sense Control 0 bits"]
pub type ISC0_R = crate::FieldReader<u8, ISC0_A>;
#[doc = "Interrupt Sense Control 0 bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC0_A {
    #[doc = "0: The low level of INTx generates an interrupt request"]
    LOW = 0,
    #[doc = "1: Any logical change on INTx generates an interrupt request"]
    TOGGLE = 1,
    #[doc = "2: The falling edge of INTx generates an interrupt request"]
    FALLING = 2,
    #[doc = "3: The rising edge of INTx generates an interrupt request"]
    RISING = 3,
}
impl From<ISC0_A> for u8 {
    #[inline(always)]
    fn from(variant: ISC0_A) -> Self {
        variant as _
    }
}
impl ISC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISC0_A {
        match self.bits {
            0 => ISC0_A::LOW,
            1 => ISC0_A::TOGGLE,
            2 => ISC0_A::FALLING,
            3 => ISC0_A::RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ISC0_A::LOW
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == ISC0_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ISC0_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ISC0_A::RISING
    }
}
#[doc = "Field `ISC0` writer - Interrupt Sense Control 0 bits"]
pub type ISC0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MCUCR_SPEC, u8, ISC0_A, 2, O>;
impl<'a, const O: u8> ISC0_W<'a, O> {
    #[doc = "The low level of INTx generates an interrupt request"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ISC0_A::LOW)
    }
    #[doc = "Any logical change on INTx generates an interrupt request"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ISC0_A::TOGGLE)
    }
    #[doc = "The falling edge of INTx generates an interrupt request"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ISC0_A::FALLING)
    }
    #[doc = "The rising edge of INTx generates an interrupt request"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ISC0_A::RISING)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt Sense Control 0 bits"]
    #[inline(always)]
    pub fn isc0(&self) -> ISC0_R {
        ISC0_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt Sense Control 0 bits"]
    #[inline(always)]
    #[must_use]
    pub fn isc0(&mut self) -> ISC0_W<0> {
        ISC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcucr](index.html) module"]
pub struct MCUCR_SPEC;
impl crate::RegisterSpec for MCUCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mcucr::R](R) reader structure"]
impl crate::Readable for MCUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcucr::W](W) writer structure"]
impl crate::Writable for MCUCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCR to value 0"]
impl crate::Resettable for MCUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
