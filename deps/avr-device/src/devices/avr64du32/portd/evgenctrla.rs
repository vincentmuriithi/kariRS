#[doc = "Register `EVGENCTRLA` reader"]
pub struct R(crate::R<EVGENCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVGENCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVGENCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVGENCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVGENCTRLA` writer"]
pub struct W(crate::W<EVGENCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVGENCTRLA_SPEC>;
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
impl From<crate::W<EVGENCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVGENCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVGEN0SEL` reader - Event Generator 0 Select"]
pub type EVGEN0SEL_R = crate::FieldReader<u8, EVGEN0SEL_A>;
#[doc = "Event Generator 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVGEN0SEL_A {
    #[doc = "0: Pin 0 used as event generator"]
    PIN0 = 0,
    #[doc = "1: Pin 1 used as event generator"]
    PIN1 = 1,
    #[doc = "2: Pin 2 used as event generator"]
    PIN2 = 2,
    #[doc = "3: Pin 3 used as event generator"]
    PIN3 = 3,
    #[doc = "4: Pin 4 used as event generator"]
    PIN4 = 4,
    #[doc = "5: Pin 5 used as event generator"]
    PIN5 = 5,
    #[doc = "6: Pin 6 used as event generator"]
    PIN6 = 6,
    #[doc = "7: Pin 7 used as event generator"]
    PIN7 = 7,
}
impl From<EVGEN0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EVGEN0SEL_A) -> Self {
        variant as _
    }
}
impl EVGEN0SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVGEN0SEL_A {
        match self.bits {
            0 => EVGEN0SEL_A::PIN0,
            1 => EVGEN0SEL_A::PIN1,
            2 => EVGEN0SEL_A::PIN2,
            3 => EVGEN0SEL_A::PIN3,
            4 => EVGEN0SEL_A::PIN4,
            5 => EVGEN0SEL_A::PIN5,
            6 => EVGEN0SEL_A::PIN6,
            7 => EVGEN0SEL_A::PIN7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EVGEN0SEL_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EVGEN0SEL_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EVGEN0SEL_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EVGEN0SEL_A::PIN3
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == EVGEN0SEL_A::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == EVGEN0SEL_A::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == EVGEN0SEL_A::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == EVGEN0SEL_A::PIN7
    }
}
#[doc = "Field `EVGEN0SEL` writer - Event Generator 0 Select"]
pub type EVGEN0SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, EVGENCTRLA_SPEC, u8, EVGEN0SEL_A, 3, O>;
impl<'a, const O: u8> EVGEN0SEL_W<'a, O> {
    #[doc = "Pin 0 used as event generator"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::PIN0)
    }
    #[doc = "Pin 1 used as event generator"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::PIN1)
    }
    #[doc = "Pin 2 used as event generator"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::PIN2)
    }
    #[doc = "Pin 3 used as event generator"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::PIN3)
    }
    #[doc = "Pin 4 used as event generator"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::PIN4)
    }
    #[doc = "Pin 5 used as event generator"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::PIN5)
    }
    #[doc = "Pin 6 used as event generator"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::PIN6)
    }
    #[doc = "Pin 7 used as event generator"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::PIN7)
    }
}
#[doc = "Field `EVGEN1SEL` reader - Event Generator 1 Select"]
pub type EVGEN1SEL_R = crate::FieldReader<u8, EVGEN1SEL_A>;
#[doc = "Event Generator 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVGEN1SEL_A {
    #[doc = "0: Pin 0 used as event generator"]
    PIN0 = 0,
    #[doc = "1: Pin 1 used as event generator"]
    PIN1 = 1,
    #[doc = "2: Pin 2 used as event generator"]
    PIN2 = 2,
    #[doc = "3: Pin 3 used as event generator"]
    PIN3 = 3,
    #[doc = "4: Pin 4 used as event generator"]
    PIN4 = 4,
    #[doc = "5: Pin 5 used as event generator"]
    PIN5 = 5,
    #[doc = "6: Pin 6 used as event generator"]
    PIN6 = 6,
    #[doc = "7: Pin 7 used as event generator"]
    PIN7 = 7,
}
impl From<EVGEN1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EVGEN1SEL_A) -> Self {
        variant as _
    }
}
impl EVGEN1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVGEN1SEL_A {
        match self.bits {
            0 => EVGEN1SEL_A::PIN0,
            1 => EVGEN1SEL_A::PIN1,
            2 => EVGEN1SEL_A::PIN2,
            3 => EVGEN1SEL_A::PIN3,
            4 => EVGEN1SEL_A::PIN4,
            5 => EVGEN1SEL_A::PIN5,
            6 => EVGEN1SEL_A::PIN6,
            7 => EVGEN1SEL_A::PIN7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EVGEN1SEL_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EVGEN1SEL_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EVGEN1SEL_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EVGEN1SEL_A::PIN3
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == EVGEN1SEL_A::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == EVGEN1SEL_A::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == EVGEN1SEL_A::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == EVGEN1SEL_A::PIN7
    }
}
#[doc = "Field `EVGEN1SEL` writer - Event Generator 1 Select"]
pub type EVGEN1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, EVGENCTRLA_SPEC, u8, EVGEN1SEL_A, 3, O>;
impl<'a, const O: u8> EVGEN1SEL_W<'a, O> {
    #[doc = "Pin 0 used as event generator"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::PIN0)
    }
    #[doc = "Pin 1 used as event generator"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::PIN1)
    }
    #[doc = "Pin 2 used as event generator"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::PIN2)
    }
    #[doc = "Pin 3 used as event generator"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::PIN3)
    }
    #[doc = "Pin 4 used as event generator"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::PIN4)
    }
    #[doc = "Pin 5 used as event generator"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::PIN5)
    }
    #[doc = "Pin 6 used as event generator"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::PIN6)
    }
    #[doc = "Pin 7 used as event generator"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::PIN7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Event Generator 0 Select"]
    #[inline(always)]
    pub fn evgen0sel(&self) -> EVGEN0SEL_R {
        EVGEN0SEL_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - Event Generator 1 Select"]
    #[inline(always)]
    pub fn evgen1sel(&self) -> EVGEN1SEL_R {
        EVGEN1SEL_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event Generator 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn evgen0sel(&mut self) -> EVGEN0SEL_W<0> {
        EVGEN0SEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Event Generator 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn evgen1sel(&mut self) -> EVGEN1SEL_W<4> {
        EVGEN1SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Generation Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evgenctrla](index.html) module"]
pub struct EVGENCTRLA_SPEC;
impl crate::RegisterSpec for EVGENCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [evgenctrla::R](R) reader structure"]
impl crate::Readable for EVGENCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evgenctrla::W](W) writer structure"]
impl crate::Writable for EVGENCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVGENCTRLA to value 0"]
impl crate::Resettable for EVGENCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
