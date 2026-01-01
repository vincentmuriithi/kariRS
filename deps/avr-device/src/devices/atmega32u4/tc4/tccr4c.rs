#[doc = "Register `TCCR4C` reader"]
pub struct R(crate::R<TCCR4C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR4C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR4C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR4C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR4C` writer"]
pub struct W(crate::W<TCCR4C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR4C_SPEC>;
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
impl From<crate::W<TCCR4C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR4C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM4D` reader - Pulse Width Modulator D Enable"]
pub type PWM4D_R = crate::BitReader<bool>;
#[doc = "Field `PWM4D` writer - Pulse Width Modulator D Enable"]
pub type PWM4D_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4C_SPEC, bool, O>;
#[doc = "Field `FOC4D` writer - Force Output Compare Match 4D"]
pub type FOC4D_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4C_SPEC, bool, O>;
#[doc = "Field `COM4D` reader - Comparator D Output Mode"]
pub type COM4D_R = crate::FieldReader<u8, COM4D_A>;
#[doc = "Comparator D Output Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM4D_A {
    #[doc = "0: Normal port operation, OCix disconnected"]
    DISCONNECTED = 0,
    #[doc = "1: Toggle OCix on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE = 1,
    #[doc = "2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)"]
    MATCH_CLEAR = 2,
    #[doc = "3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)"]
    MATCH_SET = 3,
}
impl From<COM4D_A> for u8 {
    #[inline(always)]
    fn from(variant: COM4D_A) -> Self {
        variant as _
    }
}
impl COM4D_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COM4D_A {
        match self.bits {
            0 => COM4D_A::DISCONNECTED,
            1 => COM4D_A::MATCH_TOGGLE,
            2 => COM4D_A::MATCH_CLEAR,
            3 => COM4D_A::MATCH_SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == COM4D_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `MATCH_TOGGLE`"]
    #[inline(always)]
    pub fn is_match_toggle(&self) -> bool {
        *self == COM4D_A::MATCH_TOGGLE
    }
    #[doc = "Checks if the value of the field is `MATCH_CLEAR`"]
    #[inline(always)]
    pub fn is_match_clear(&self) -> bool {
        *self == COM4D_A::MATCH_CLEAR
    }
    #[doc = "Checks if the value of the field is `MATCH_SET`"]
    #[inline(always)]
    pub fn is_match_set(&self) -> bool {
        *self == COM4D_A::MATCH_SET
    }
}
#[doc = "Field `COM4D` writer - Comparator D Output Mode"]
pub type COM4D_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR4C_SPEC, u8, COM4D_A, 2, O>;
impl<'a, const O: u8> COM4D_W<'a, O> {
    #[doc = "Normal port operation, OCix disconnected"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM4D_A::DISCONNECTED)
    }
    #[doc = "Toggle OCix on Compare Match (Might depend on WGM)"]
    #[inline(always)]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM4D_A::MATCH_TOGGLE)
    }
    #[doc = "Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)"]
    #[inline(always)]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM4D_A::MATCH_CLEAR)
    }
    #[doc = "Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)"]
    #[inline(always)]
    pub fn match_set(self) -> &'a mut W {
        self.variant(COM4D_A::MATCH_SET)
    }
}
#[doc = "Field `COM4B0S` reader - Comparator B Output Mode"]
pub type COM4B0S_R = crate::BitReader<bool>;
#[doc = "Field `COM4B0S` writer - Comparator B Output Mode"]
pub type COM4B0S_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4C_SPEC, bool, O>;
#[doc = "Field `COM4B1S` reader - Comparator B Output Mode"]
pub type COM4B1S_R = crate::BitReader<bool>;
#[doc = "Field `COM4B1S` writer - Comparator B Output Mode"]
pub type COM4B1S_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4C_SPEC, bool, O>;
#[doc = "Field `COM4A0S` reader - Comparator A Output Mode"]
pub type COM4A0S_R = crate::BitReader<bool>;
#[doc = "Field `COM4A0S` writer - Comparator A Output Mode"]
pub type COM4A0S_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4C_SPEC, bool, O>;
#[doc = "Field `COM4A1S` reader - Comparator A Output Mode"]
pub type COM4A1S_R = crate::BitReader<bool>;
#[doc = "Field `COM4A1S` writer - Comparator A Output Mode"]
pub type COM4A1S_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4C_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pulse Width Modulator D Enable"]
    #[inline(always)]
    pub fn pwm4d(&self) -> PWM4D_R {
        PWM4D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator D Output Mode"]
    #[inline(always)]
    pub fn com4d(&self) -> COM4D_R {
        COM4D_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Comparator B Output Mode"]
    #[inline(always)]
    pub fn com4b0s(&self) -> COM4B0S_R {
        COM4B0S_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator B Output Mode"]
    #[inline(always)]
    pub fn com4b1s(&self) -> COM4B1S_R {
        COM4B1S_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparator A Output Mode"]
    #[inline(always)]
    pub fn com4a0s(&self) -> COM4A0S_R {
        COM4A0S_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator A Output Mode"]
    #[inline(always)]
    pub fn com4a1s(&self) -> COM4A1S_R {
        COM4A1S_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Width Modulator D Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm4d(&mut self) -> PWM4D_W<0> {
        PWM4D_W::new(self)
    }
    #[doc = "Bit 1 - Force Output Compare Match 4D"]
    #[inline(always)]
    #[must_use]
    pub fn foc4d(&mut self) -> FOC4D_W<1> {
        FOC4D_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator D Output Mode"]
    #[inline(always)]
    #[must_use]
    pub fn com4d(&mut self) -> COM4D_W<2> {
        COM4D_W::new(self)
    }
    #[doc = "Bit 4 - Comparator B Output Mode"]
    #[inline(always)]
    #[must_use]
    pub fn com4b0s(&mut self) -> COM4B0S_W<4> {
        COM4B0S_W::new(self)
    }
    #[doc = "Bit 5 - Comparator B Output Mode"]
    #[inline(always)]
    #[must_use]
    pub fn com4b1s(&mut self) -> COM4B1S_W<5> {
        COM4B1S_W::new(self)
    }
    #[doc = "Bit 6 - Comparator A Output Mode"]
    #[inline(always)]
    #[must_use]
    pub fn com4a0s(&mut self) -> COM4A0S_W<6> {
        COM4A0S_W::new(self)
    }
    #[doc = "Bit 7 - Comparator A Output Mode"]
    #[inline(always)]
    #[must_use]
    pub fn com4a1s(&mut self) -> COM4A1S_W<7> {
        COM4A1S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter 4 Control Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr4c](index.html) module"]
pub struct TCCR4C_SPEC;
impl crate::RegisterSpec for TCCR4C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr4c::R](R) reader structure"]
impl crate::Readable for TCCR4C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr4c::W](W) writer structure"]
impl crate::Writable for TCCR4C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4C to value 0"]
impl crate::Resettable for TCCR4C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
