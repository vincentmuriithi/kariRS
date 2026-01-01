#[doc = "Register `SYNCCH0` reader"]
pub struct R(crate::R<SYNCCH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCCH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCCH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCCH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCCH0` writer"]
pub struct W(crate::W<SYNCCH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCCH0_SPEC>;
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
impl From<crate::W<SYNCCH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCCH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCCH0` reader - Synchronous Channel 0 Generator Selection"]
pub type SYNCCH0_R = crate::FieldReader<u8, SYNCCH0_A>;
#[doc = "Synchronous Channel 0 Generator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCCH0_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Timer/Counter B0"]
    TCB0 = 1,
    #[doc = "2: Timer/Counter A0 overflow"]
    TCA0_OVF_LUNF = 2,
    #[doc = "3: Timer/Counter A0 underflow high byte (split mode)"]
    TCA0_HUNF = 3,
    #[doc = "4: Timer/Counter A0 compare 0"]
    TCA0_CMP0 = 4,
    #[doc = "5: Timer/Counter A0 compare 1"]
    TCA0_CMP1 = 5,
    #[doc = "6: Timer/Counter A0 compare 2"]
    TCA0_CMP2 = 6,
    #[doc = "7: Synchronous Event from Pin PC0"]
    PORTC_PIN0 = 7,
    #[doc = "8: Synchronous Event from Pin PC1"]
    PORTC_PIN1 = 8,
    #[doc = "9: Synchronous Event from Pin PC2"]
    PORTC_PIN2 = 9,
    #[doc = "10: Synchronous Event from Pin PC3"]
    PORTC_PIN3 = 10,
    #[doc = "11: Synchronous Event from Pin PC4"]
    PORTC_PIN4 = 11,
    #[doc = "12: Synchronous Event from Pin PC5"]
    PORTC_PIN5 = 12,
    #[doc = "13: Synchronous Event from Pin PA0"]
    PORTA_PIN0 = 13,
    #[doc = "14: Synchronous Event from Pin PA1"]
    PORTA_PIN1 = 14,
    #[doc = "15: Synchronous Event from Pin PA2"]
    PORTA_PIN2 = 15,
    #[doc = "16: Synchronous Event from Pin PA3"]
    PORTA_PIN3 = 16,
    #[doc = "17: Synchronous Event from Pin PA4"]
    PORTA_PIN4 = 17,
    #[doc = "18: Synchronous Event from Pin PA5"]
    PORTA_PIN5 = 18,
    #[doc = "19: Synchronous Event from Pin PA6"]
    PORTA_PIN6 = 19,
    #[doc = "20: Synchronous Event from Pin PA7"]
    PORTA_PIN7 = 20,
    #[doc = "21: Timer/Counter B1"]
    TCB1 = 21,
}
impl From<SYNCCH0_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCCH0_A) -> Self {
        variant as _
    }
}
impl SYNCCH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCCH0_A> {
        match self.bits {
            0 => Some(SYNCCH0_A::OFF),
            1 => Some(SYNCCH0_A::TCB0),
            2 => Some(SYNCCH0_A::TCA0_OVF_LUNF),
            3 => Some(SYNCCH0_A::TCA0_HUNF),
            4 => Some(SYNCCH0_A::TCA0_CMP0),
            5 => Some(SYNCCH0_A::TCA0_CMP1),
            6 => Some(SYNCCH0_A::TCA0_CMP2),
            7 => Some(SYNCCH0_A::PORTC_PIN0),
            8 => Some(SYNCCH0_A::PORTC_PIN1),
            9 => Some(SYNCCH0_A::PORTC_PIN2),
            10 => Some(SYNCCH0_A::PORTC_PIN3),
            11 => Some(SYNCCH0_A::PORTC_PIN4),
            12 => Some(SYNCCH0_A::PORTC_PIN5),
            13 => Some(SYNCCH0_A::PORTA_PIN0),
            14 => Some(SYNCCH0_A::PORTA_PIN1),
            15 => Some(SYNCCH0_A::PORTA_PIN2),
            16 => Some(SYNCCH0_A::PORTA_PIN3),
            17 => Some(SYNCCH0_A::PORTA_PIN4),
            18 => Some(SYNCCH0_A::PORTA_PIN5),
            19 => Some(SYNCCH0_A::PORTA_PIN6),
            20 => Some(SYNCCH0_A::PORTA_PIN7),
            21 => Some(SYNCCH0_A::TCB1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SYNCCH0_A::OFF
    }
    #[doc = "Checks if the value of the field is `TCB0`"]
    #[inline(always)]
    pub fn is_tcb0(&self) -> bool {
        *self == SYNCCH0_A::TCB0
    }
    #[doc = "Checks if the value of the field is `TCA0_OVF_LUNF`"]
    #[inline(always)]
    pub fn is_tca0_ovf_lunf(&self) -> bool {
        *self == SYNCCH0_A::TCA0_OVF_LUNF
    }
    #[doc = "Checks if the value of the field is `TCA0_HUNF`"]
    #[inline(always)]
    pub fn is_tca0_hunf(&self) -> bool {
        *self == SYNCCH0_A::TCA0_HUNF
    }
    #[doc = "Checks if the value of the field is `TCA0_CMP0`"]
    #[inline(always)]
    pub fn is_tca0_cmp0(&self) -> bool {
        *self == SYNCCH0_A::TCA0_CMP0
    }
    #[doc = "Checks if the value of the field is `TCA0_CMP1`"]
    #[inline(always)]
    pub fn is_tca0_cmp1(&self) -> bool {
        *self == SYNCCH0_A::TCA0_CMP1
    }
    #[doc = "Checks if the value of the field is `TCA0_CMP2`"]
    #[inline(always)]
    pub fn is_tca0_cmp2(&self) -> bool {
        *self == SYNCCH0_A::TCA0_CMP2
    }
    #[doc = "Checks if the value of the field is `PORTC_PIN0`"]
    #[inline(always)]
    pub fn is_portc_pin0(&self) -> bool {
        *self == SYNCCH0_A::PORTC_PIN0
    }
    #[doc = "Checks if the value of the field is `PORTC_PIN1`"]
    #[inline(always)]
    pub fn is_portc_pin1(&self) -> bool {
        *self == SYNCCH0_A::PORTC_PIN1
    }
    #[doc = "Checks if the value of the field is `PORTC_PIN2`"]
    #[inline(always)]
    pub fn is_portc_pin2(&self) -> bool {
        *self == SYNCCH0_A::PORTC_PIN2
    }
    #[doc = "Checks if the value of the field is `PORTC_PIN3`"]
    #[inline(always)]
    pub fn is_portc_pin3(&self) -> bool {
        *self == SYNCCH0_A::PORTC_PIN3
    }
    #[doc = "Checks if the value of the field is `PORTC_PIN4`"]
    #[inline(always)]
    pub fn is_portc_pin4(&self) -> bool {
        *self == SYNCCH0_A::PORTC_PIN4
    }
    #[doc = "Checks if the value of the field is `PORTC_PIN5`"]
    #[inline(always)]
    pub fn is_portc_pin5(&self) -> bool {
        *self == SYNCCH0_A::PORTC_PIN5
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN0`"]
    #[inline(always)]
    pub fn is_porta_pin0(&self) -> bool {
        *self == SYNCCH0_A::PORTA_PIN0
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN1`"]
    #[inline(always)]
    pub fn is_porta_pin1(&self) -> bool {
        *self == SYNCCH0_A::PORTA_PIN1
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN2`"]
    #[inline(always)]
    pub fn is_porta_pin2(&self) -> bool {
        *self == SYNCCH0_A::PORTA_PIN2
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN3`"]
    #[inline(always)]
    pub fn is_porta_pin3(&self) -> bool {
        *self == SYNCCH0_A::PORTA_PIN3
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN4`"]
    #[inline(always)]
    pub fn is_porta_pin4(&self) -> bool {
        *self == SYNCCH0_A::PORTA_PIN4
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN5`"]
    #[inline(always)]
    pub fn is_porta_pin5(&self) -> bool {
        *self == SYNCCH0_A::PORTA_PIN5
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN6`"]
    #[inline(always)]
    pub fn is_porta_pin6(&self) -> bool {
        *self == SYNCCH0_A::PORTA_PIN6
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN7`"]
    #[inline(always)]
    pub fn is_porta_pin7(&self) -> bool {
        *self == SYNCCH0_A::PORTA_PIN7
    }
    #[doc = "Checks if the value of the field is `TCB1`"]
    #[inline(always)]
    pub fn is_tcb1(&self) -> bool {
        *self == SYNCCH0_A::TCB1
    }
}
#[doc = "Field `SYNCCH0` writer - Synchronous Channel 0 Generator Selection"]
pub type SYNCCH0_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SYNCCH0_SPEC, u8, SYNCCH0_A, 8, O>;
impl<'a, const O: u8> SYNCCH0_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SYNCCH0_A::OFF)
    }
    #[doc = "Timer/Counter B0"]
    #[inline(always)]
    pub fn tcb0(self) -> &'a mut W {
        self.variant(SYNCCH0_A::TCB0)
    }
    #[doc = "Timer/Counter A0 overflow"]
    #[inline(always)]
    pub fn tca0_ovf_lunf(self) -> &'a mut W {
        self.variant(SYNCCH0_A::TCA0_OVF_LUNF)
    }
    #[doc = "Timer/Counter A0 underflow high byte (split mode)"]
    #[inline(always)]
    pub fn tca0_hunf(self) -> &'a mut W {
        self.variant(SYNCCH0_A::TCA0_HUNF)
    }
    #[doc = "Timer/Counter A0 compare 0"]
    #[inline(always)]
    pub fn tca0_cmp0(self) -> &'a mut W {
        self.variant(SYNCCH0_A::TCA0_CMP0)
    }
    #[doc = "Timer/Counter A0 compare 1"]
    #[inline(always)]
    pub fn tca0_cmp1(self) -> &'a mut W {
        self.variant(SYNCCH0_A::TCA0_CMP1)
    }
    #[doc = "Timer/Counter A0 compare 2"]
    #[inline(always)]
    pub fn tca0_cmp2(self) -> &'a mut W {
        self.variant(SYNCCH0_A::TCA0_CMP2)
    }
    #[doc = "Synchronous Event from Pin PC0"]
    #[inline(always)]
    pub fn portc_pin0(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTC_PIN0)
    }
    #[doc = "Synchronous Event from Pin PC1"]
    #[inline(always)]
    pub fn portc_pin1(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTC_PIN1)
    }
    #[doc = "Synchronous Event from Pin PC2"]
    #[inline(always)]
    pub fn portc_pin2(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTC_PIN2)
    }
    #[doc = "Synchronous Event from Pin PC3"]
    #[inline(always)]
    pub fn portc_pin3(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTC_PIN3)
    }
    #[doc = "Synchronous Event from Pin PC4"]
    #[inline(always)]
    pub fn portc_pin4(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTC_PIN4)
    }
    #[doc = "Synchronous Event from Pin PC5"]
    #[inline(always)]
    pub fn portc_pin5(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTC_PIN5)
    }
    #[doc = "Synchronous Event from Pin PA0"]
    #[inline(always)]
    pub fn porta_pin0(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTA_PIN0)
    }
    #[doc = "Synchronous Event from Pin PA1"]
    #[inline(always)]
    pub fn porta_pin1(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTA_PIN1)
    }
    #[doc = "Synchronous Event from Pin PA2"]
    #[inline(always)]
    pub fn porta_pin2(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTA_PIN2)
    }
    #[doc = "Synchronous Event from Pin PA3"]
    #[inline(always)]
    pub fn porta_pin3(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTA_PIN3)
    }
    #[doc = "Synchronous Event from Pin PA4"]
    #[inline(always)]
    pub fn porta_pin4(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTA_PIN4)
    }
    #[doc = "Synchronous Event from Pin PA5"]
    #[inline(always)]
    pub fn porta_pin5(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTA_PIN5)
    }
    #[doc = "Synchronous Event from Pin PA6"]
    #[inline(always)]
    pub fn porta_pin6(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTA_PIN6)
    }
    #[doc = "Synchronous Event from Pin PA7"]
    #[inline(always)]
    pub fn porta_pin7(self) -> &'a mut W {
        self.variant(SYNCCH0_A::PORTA_PIN7)
    }
    #[doc = "Timer/Counter B1"]
    #[inline(always)]
    pub fn tcb1(self) -> &'a mut W {
        self.variant(SYNCCH0_A::TCB1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Synchronous Channel 0 Generator Selection"]
    #[inline(always)]
    pub fn syncch0(&self) -> SYNCCH0_R {
        SYNCCH0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronous Channel 0 Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn syncch0(&mut self) -> SYNCCH0_W<0> {
        SYNCCH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous Channel 0 Generator Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncch0](index.html) module"]
pub struct SYNCCH0_SPEC;
impl crate::RegisterSpec for SYNCCH0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [syncch0::R](R) reader structure"]
impl crate::Readable for SYNCCH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syncch0::W](W) writer structure"]
impl crate::Writable for SYNCCH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCCH0 to value 0"]
impl crate::Resettable for SYNCCH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
