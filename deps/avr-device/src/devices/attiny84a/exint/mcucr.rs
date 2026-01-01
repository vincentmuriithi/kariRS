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
#[doc = "Field `ISC0` reader - Interrupt Sense Control 0"]
pub type ISC0_R = crate::FieldReader<u8, ISC0_A>;
#[doc = "Interrupt Sense Control 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC0_A {
    #[doc = "0: Low Level of INTX"]
    LOW_LEVEL_OF_INTX = 0,
    #[doc = "1: Any Logical Change of INTX"]
    ANY_LOGICAL_CHANGE_OF_INTX = 1,
    #[doc = "2: Falling Edge of INTX"]
    FALLING_EDGE_OF_INTX = 2,
    #[doc = "3: Rising Edge of INTX"]
    RISING_EDGE_OF_INTX = 3,
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
            0 => ISC0_A::LOW_LEVEL_OF_INTX,
            1 => ISC0_A::ANY_LOGICAL_CHANGE_OF_INTX,
            2 => ISC0_A::FALLING_EDGE_OF_INTX,
            3 => ISC0_A::RISING_EDGE_OF_INTX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL_OF_INTX`"]
    #[inline(always)]
    pub fn is_low_level_of_intx(&self) -> bool {
        *self == ISC0_A::LOW_LEVEL_OF_INTX
    }
    #[doc = "Checks if the value of the field is `ANY_LOGICAL_CHANGE_OF_INTX`"]
    #[inline(always)]
    pub fn is_any_logical_change_of_intx(&self) -> bool {
        *self == ISC0_A::ANY_LOGICAL_CHANGE_OF_INTX
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_OF_INTX`"]
    #[inline(always)]
    pub fn is_falling_edge_of_intx(&self) -> bool {
        *self == ISC0_A::FALLING_EDGE_OF_INTX
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_OF_INTX`"]
    #[inline(always)]
    pub fn is_rising_edge_of_intx(&self) -> bool {
        *self == ISC0_A::RISING_EDGE_OF_INTX
    }
}
#[doc = "Field `ISC0` writer - Interrupt Sense Control 0"]
pub type ISC0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MCUCR_SPEC, u8, ISC0_A, 2, O>;
impl<'a, const O: u8> ISC0_W<'a, O> {
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn low_level_of_intx(self) -> &'a mut W {
        self.variant(ISC0_A::LOW_LEVEL_OF_INTX)
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn any_logical_change_of_intx(self) -> &'a mut W {
        self.variant(ISC0_A::ANY_LOGICAL_CHANGE_OF_INTX)
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn falling_edge_of_intx(self) -> &'a mut W {
        self.variant(ISC0_A::FALLING_EDGE_OF_INTX)
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn rising_edge_of_intx(self) -> &'a mut W {
        self.variant(ISC0_A::RISING_EDGE_OF_INTX)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt Sense Control 0"]
    #[inline(always)]
    pub fn isc0(&self) -> ISC0_R {
        ISC0_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt Sense Control 0"]
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
