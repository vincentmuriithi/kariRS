#[doc = "Register `EICRA` reader"]
pub struct R(crate::R<EICRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EICRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EICRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EICRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EICRA` writer"]
pub struct W(crate::W<EICRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EICRA_SPEC>;
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
impl From<crate::W<EICRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EICRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISC0` reader - External Interrupt Sense Control Bit"]
pub type ISC0_R = crate::FieldReader<u8, ISC0_A>;
#[doc = "External Interrupt Sense Control Bit\n\nValue on reset: 0"]
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
#[doc = "Field `ISC0` writer - External Interrupt Sense Control Bit"]
pub type ISC0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EICRA_SPEC, u8, ISC0_A, 2, O>;
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
#[doc = "Field `ISC1` reader - External Interrupt Sense Control Bit"]
pub type ISC1_R = crate::FieldReader<u8, ISC1_A>;
#[doc = "External Interrupt Sense Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC1_A {
    #[doc = "0: Low Level of INTX"]
    LOW_LEVEL_OF_INTX = 0,
    #[doc = "1: Any Logical Change of INTX"]
    ANY_LOGICAL_CHANGE_OF_INTX = 1,
    #[doc = "2: Falling Edge of INTX"]
    FALLING_EDGE_OF_INTX = 2,
    #[doc = "3: Rising Edge of INTX"]
    RISING_EDGE_OF_INTX = 3,
}
impl From<ISC1_A> for u8 {
    #[inline(always)]
    fn from(variant: ISC1_A) -> Self {
        variant as _
    }
}
impl ISC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISC1_A {
        match self.bits {
            0 => ISC1_A::LOW_LEVEL_OF_INTX,
            1 => ISC1_A::ANY_LOGICAL_CHANGE_OF_INTX,
            2 => ISC1_A::FALLING_EDGE_OF_INTX,
            3 => ISC1_A::RISING_EDGE_OF_INTX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL_OF_INTX`"]
    #[inline(always)]
    pub fn is_low_level_of_intx(&self) -> bool {
        *self == ISC1_A::LOW_LEVEL_OF_INTX
    }
    #[doc = "Checks if the value of the field is `ANY_LOGICAL_CHANGE_OF_INTX`"]
    #[inline(always)]
    pub fn is_any_logical_change_of_intx(&self) -> bool {
        *self == ISC1_A::ANY_LOGICAL_CHANGE_OF_INTX
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_OF_INTX`"]
    #[inline(always)]
    pub fn is_falling_edge_of_intx(&self) -> bool {
        *self == ISC1_A::FALLING_EDGE_OF_INTX
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_OF_INTX`"]
    #[inline(always)]
    pub fn is_rising_edge_of_intx(&self) -> bool {
        *self == ISC1_A::RISING_EDGE_OF_INTX
    }
}
#[doc = "Field `ISC1` writer - External Interrupt Sense Control Bit"]
pub type ISC1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EICRA_SPEC, u8, ISC1_A, 2, O>;
impl<'a, const O: u8> ISC1_W<'a, O> {
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn low_level_of_intx(self) -> &'a mut W {
        self.variant(ISC1_A::LOW_LEVEL_OF_INTX)
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn any_logical_change_of_intx(self) -> &'a mut W {
        self.variant(ISC1_A::ANY_LOGICAL_CHANGE_OF_INTX)
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn falling_edge_of_intx(self) -> &'a mut W {
        self.variant(ISC1_A::FALLING_EDGE_OF_INTX)
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn rising_edge_of_intx(self) -> &'a mut W {
        self.variant(ISC1_A::RISING_EDGE_OF_INTX)
    }
}
#[doc = "Field `ISC2` reader - External Interrupt Sense Control Bit"]
pub type ISC2_R = crate::FieldReader<u8, ISC2_A>;
#[doc = "External Interrupt Sense Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC2_A {
    #[doc = "0: Low Level of INTX"]
    LOW_LEVEL_OF_INTX = 0,
    #[doc = "1: Any Logical Change of INTX"]
    ANY_LOGICAL_CHANGE_OF_INTX = 1,
    #[doc = "2: Falling Edge of INTX"]
    FALLING_EDGE_OF_INTX = 2,
    #[doc = "3: Rising Edge of INTX"]
    RISING_EDGE_OF_INTX = 3,
}
impl From<ISC2_A> for u8 {
    #[inline(always)]
    fn from(variant: ISC2_A) -> Self {
        variant as _
    }
}
impl ISC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISC2_A {
        match self.bits {
            0 => ISC2_A::LOW_LEVEL_OF_INTX,
            1 => ISC2_A::ANY_LOGICAL_CHANGE_OF_INTX,
            2 => ISC2_A::FALLING_EDGE_OF_INTX,
            3 => ISC2_A::RISING_EDGE_OF_INTX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL_OF_INTX`"]
    #[inline(always)]
    pub fn is_low_level_of_intx(&self) -> bool {
        *self == ISC2_A::LOW_LEVEL_OF_INTX
    }
    #[doc = "Checks if the value of the field is `ANY_LOGICAL_CHANGE_OF_INTX`"]
    #[inline(always)]
    pub fn is_any_logical_change_of_intx(&self) -> bool {
        *self == ISC2_A::ANY_LOGICAL_CHANGE_OF_INTX
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_OF_INTX`"]
    #[inline(always)]
    pub fn is_falling_edge_of_intx(&self) -> bool {
        *self == ISC2_A::FALLING_EDGE_OF_INTX
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_OF_INTX`"]
    #[inline(always)]
    pub fn is_rising_edge_of_intx(&self) -> bool {
        *self == ISC2_A::RISING_EDGE_OF_INTX
    }
}
#[doc = "Field `ISC2` writer - External Interrupt Sense Control Bit"]
pub type ISC2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EICRA_SPEC, u8, ISC2_A, 2, O>;
impl<'a, const O: u8> ISC2_W<'a, O> {
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn low_level_of_intx(self) -> &'a mut W {
        self.variant(ISC2_A::LOW_LEVEL_OF_INTX)
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn any_logical_change_of_intx(self) -> &'a mut W {
        self.variant(ISC2_A::ANY_LOGICAL_CHANGE_OF_INTX)
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn falling_edge_of_intx(self) -> &'a mut W {
        self.variant(ISC2_A::FALLING_EDGE_OF_INTX)
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn rising_edge_of_intx(self) -> &'a mut W {
        self.variant(ISC2_A::RISING_EDGE_OF_INTX)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    pub fn isc0(&self) -> ISC0_R {
        ISC0_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    pub fn isc1(&self) -> ISC1_R {
        ISC1_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    pub fn isc2(&self) -> ISC2_R {
        ISC2_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc0(&mut self) -> ISC0_W<0> {
        ISC0_W::new(self)
    }
    #[doc = "Bits 2:3 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc1(&mut self) -> ISC1_W<2> {
        ISC1_W::new(self)
    }
    #[doc = "Bits 4:5 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc2(&mut self) -> ISC2_W<4> {
        ISC2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eicra](index.html) module"]
pub struct EICRA_SPEC;
impl crate::RegisterSpec for EICRA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eicra::R](R) reader structure"]
impl crate::Readable for EICRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eicra::W](W) writer structure"]
impl crate::Writable for EICRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EICRA to value 0"]
impl crate::Resettable for EICRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
