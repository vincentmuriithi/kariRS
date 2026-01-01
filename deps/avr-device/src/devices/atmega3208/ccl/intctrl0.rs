#[doc = "Register `INTCTRL0` reader"]
pub struct R(crate::R<INTCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCTRL0` writer"]
pub struct W(crate::W<INTCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCTRL0_SPEC>;
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
impl From<crate::W<INTCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTMODE0` reader - Interrupt Mode for LUT0"]
pub type INTMODE0_R = crate::FieldReader<u8, INTMODE0_A>;
#[doc = "Interrupt Mode for LUT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTMODE0_A {
    #[doc = "0: Interrupt disabled"]
    INTDISABLE = 0,
    #[doc = "1: Sense rising edge"]
    RISING = 1,
    #[doc = "2: Sense falling edge"]
    FALLING = 2,
    #[doc = "3: Sense both edges"]
    BOTH = 3,
}
impl From<INTMODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE0_A) -> Self {
        variant as _
    }
}
impl INTMODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTMODE0_A {
        match self.bits {
            0 => INTMODE0_A::INTDISABLE,
            1 => INTMODE0_A::RISING,
            2 => INTMODE0_A::FALLING,
            3 => INTMODE0_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INTDISABLE`"]
    #[inline(always)]
    pub fn is_intdisable(&self) -> bool {
        *self == INTMODE0_A::INTDISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTMODE0_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTMODE0_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTMODE0_A::BOTH
    }
}
#[doc = "Field `INTMODE0` writer - Interrupt Mode for LUT0"]
pub type INTMODE0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, INTCTRL0_SPEC, u8, INTMODE0_A, 2, O>;
impl<'a, const O: u8> INTMODE0_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn intdisable(self) -> &'a mut W {
        self.variant(INTMODE0_A::INTDISABLE)
    }
    #[doc = "Sense rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTMODE0_A::RISING)
    }
    #[doc = "Sense falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTMODE0_A::FALLING)
    }
    #[doc = "Sense both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(INTMODE0_A::BOTH)
    }
}
#[doc = "Field `INTMODE1` reader - Interrupt Mode for LUT1"]
pub type INTMODE1_R = crate::FieldReader<u8, INTMODE1_A>;
#[doc = "Interrupt Mode for LUT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTMODE1_A {
    #[doc = "0: Interrupt disabled"]
    INTDISABLE = 0,
    #[doc = "1: Sense rising edge"]
    RISING = 1,
    #[doc = "2: Sense falling edge"]
    FALLING = 2,
    #[doc = "3: Sense both edges"]
    BOTH = 3,
}
impl From<INTMODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE1_A) -> Self {
        variant as _
    }
}
impl INTMODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTMODE1_A {
        match self.bits {
            0 => INTMODE1_A::INTDISABLE,
            1 => INTMODE1_A::RISING,
            2 => INTMODE1_A::FALLING,
            3 => INTMODE1_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INTDISABLE`"]
    #[inline(always)]
    pub fn is_intdisable(&self) -> bool {
        *self == INTMODE1_A::INTDISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTMODE1_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTMODE1_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTMODE1_A::BOTH
    }
}
#[doc = "Field `INTMODE1` writer - Interrupt Mode for LUT1"]
pub type INTMODE1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, INTCTRL0_SPEC, u8, INTMODE1_A, 2, O>;
impl<'a, const O: u8> INTMODE1_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn intdisable(self) -> &'a mut W {
        self.variant(INTMODE1_A::INTDISABLE)
    }
    #[doc = "Sense rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTMODE1_A::RISING)
    }
    #[doc = "Sense falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTMODE1_A::FALLING)
    }
    #[doc = "Sense both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(INTMODE1_A::BOTH)
    }
}
#[doc = "Field `INTMODE2` reader - Interrupt Mode for LUT2"]
pub type INTMODE2_R = crate::FieldReader<u8, INTMODE2_A>;
#[doc = "Interrupt Mode for LUT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTMODE2_A {
    #[doc = "0: Interrupt disabled"]
    INTDISABLE = 0,
    #[doc = "1: Sense rising edge"]
    RISING = 1,
    #[doc = "2: Sense falling edge"]
    FALLING = 2,
    #[doc = "3: Sense both edges"]
    BOTH = 3,
}
impl From<INTMODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE2_A) -> Self {
        variant as _
    }
}
impl INTMODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTMODE2_A {
        match self.bits {
            0 => INTMODE2_A::INTDISABLE,
            1 => INTMODE2_A::RISING,
            2 => INTMODE2_A::FALLING,
            3 => INTMODE2_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INTDISABLE`"]
    #[inline(always)]
    pub fn is_intdisable(&self) -> bool {
        *self == INTMODE2_A::INTDISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTMODE2_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTMODE2_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTMODE2_A::BOTH
    }
}
#[doc = "Field `INTMODE2` writer - Interrupt Mode for LUT2"]
pub type INTMODE2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, INTCTRL0_SPEC, u8, INTMODE2_A, 2, O>;
impl<'a, const O: u8> INTMODE2_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn intdisable(self) -> &'a mut W {
        self.variant(INTMODE2_A::INTDISABLE)
    }
    #[doc = "Sense rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTMODE2_A::RISING)
    }
    #[doc = "Sense falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTMODE2_A::FALLING)
    }
    #[doc = "Sense both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(INTMODE2_A::BOTH)
    }
}
#[doc = "Field `INTMODE3` reader - Interrupt Mode for LUT3"]
pub type INTMODE3_R = crate::FieldReader<u8, INTMODE3_A>;
#[doc = "Interrupt Mode for LUT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTMODE3_A {
    #[doc = "0: Interrupt disabled"]
    INTDISABLE = 0,
    #[doc = "1: Sense rising edge"]
    RISING = 1,
    #[doc = "2: Sense falling edge"]
    FALLING = 2,
    #[doc = "3: Sense both edges"]
    BOTH = 3,
}
impl From<INTMODE3_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE3_A) -> Self {
        variant as _
    }
}
impl INTMODE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTMODE3_A {
        match self.bits {
            0 => INTMODE3_A::INTDISABLE,
            1 => INTMODE3_A::RISING,
            2 => INTMODE3_A::FALLING,
            3 => INTMODE3_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INTDISABLE`"]
    #[inline(always)]
    pub fn is_intdisable(&self) -> bool {
        *self == INTMODE3_A::INTDISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTMODE3_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTMODE3_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTMODE3_A::BOTH
    }
}
#[doc = "Field `INTMODE3` writer - Interrupt Mode for LUT3"]
pub type INTMODE3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, INTCTRL0_SPEC, u8, INTMODE3_A, 2, O>;
impl<'a, const O: u8> INTMODE3_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn intdisable(self) -> &'a mut W {
        self.variant(INTMODE3_A::INTDISABLE)
    }
    #[doc = "Sense rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTMODE3_A::RISING)
    }
    #[doc = "Sense falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTMODE3_A::FALLING)
    }
    #[doc = "Sense both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(INTMODE3_A::BOTH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt Mode for LUT0"]
    #[inline(always)]
    pub fn intmode0(&self) -> INTMODE0_R {
        INTMODE0_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Interrupt Mode for LUT1"]
    #[inline(always)]
    pub fn intmode1(&self) -> INTMODE1_R {
        INTMODE1_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Interrupt Mode for LUT2"]
    #[inline(always)]
    pub fn intmode2(&self) -> INTMODE2_R {
        INTMODE2_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Interrupt Mode for LUT3"]
    #[inline(always)]
    pub fn intmode3(&self) -> INTMODE3_R {
        INTMODE3_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt Mode for LUT0"]
    #[inline(always)]
    #[must_use]
    pub fn intmode0(&mut self) -> INTMODE0_W<0> {
        INTMODE0_W::new(self)
    }
    #[doc = "Bits 2:3 - Interrupt Mode for LUT1"]
    #[inline(always)]
    #[must_use]
    pub fn intmode1(&mut self) -> INTMODE1_W<2> {
        INTMODE1_W::new(self)
    }
    #[doc = "Bits 4:5 - Interrupt Mode for LUT2"]
    #[inline(always)]
    #[must_use]
    pub fn intmode2(&mut self) -> INTMODE2_W<4> {
        INTMODE2_W::new(self)
    }
    #[doc = "Bits 6:7 - Interrupt Mode for LUT3"]
    #[inline(always)]
    #[must_use]
    pub fn intmode3(&mut self) -> INTMODE3_W<6> {
        INTMODE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intctrl0](index.html) module"]
pub struct INTCTRL0_SPEC;
impl crate::RegisterSpec for INTCTRL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intctrl0::R](R) reader structure"]
impl crate::Readable for INTCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intctrl0::W](W) writer structure"]
impl crate::Writable for INTCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL0 to value 0"]
impl crate::Resettable for INTCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
