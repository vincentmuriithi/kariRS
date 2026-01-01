#[doc = "Register `TCCR1C` reader"]
pub struct R(crate::R<TCCR1C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR1C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR1C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR1C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR1C` writer"]
pub struct W(crate::W<TCCR1C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR1C_SPEC>;
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
impl From<crate::W<TCCR1C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR1C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM1D` reader - Pulse Width Modulator D Enable"]
pub type PWM1D_R = crate::BitReader<bool>;
#[doc = "Field `PWM1D` writer - Pulse Width Modulator D Enable"]
pub type PWM1D_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1C_SPEC, bool, O>;
#[doc = "Field `FOC1D` writer - Force Output Compare Match 1D"]
pub type FOC1D_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1C_SPEC, bool, O>;
#[doc = "Field `COM1D` reader - Comparator D output mode"]
pub type COM1D_R = crate::FieldReader<u8, COM1D_A>;
#[doc = "Comparator D output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM1D_A {
    #[doc = "0: Normal port operation, OCix disconnected"]
    DISCONNECTED = 0,
    #[doc = "1: Toggle OCix on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE = 1,
    #[doc = "2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)"]
    MATCH_CLEAR = 2,
    #[doc = "3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)"]
    MATCH_SET = 3,
}
impl From<COM1D_A> for u8 {
    #[inline(always)]
    fn from(variant: COM1D_A) -> Self {
        variant as _
    }
}
impl COM1D_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COM1D_A {
        match self.bits {
            0 => COM1D_A::DISCONNECTED,
            1 => COM1D_A::MATCH_TOGGLE,
            2 => COM1D_A::MATCH_CLEAR,
            3 => COM1D_A::MATCH_SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == COM1D_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `MATCH_TOGGLE`"]
    #[inline(always)]
    pub fn is_match_toggle(&self) -> bool {
        *self == COM1D_A::MATCH_TOGGLE
    }
    #[doc = "Checks if the value of the field is `MATCH_CLEAR`"]
    #[inline(always)]
    pub fn is_match_clear(&self) -> bool {
        *self == COM1D_A::MATCH_CLEAR
    }
    #[doc = "Checks if the value of the field is `MATCH_SET`"]
    #[inline(always)]
    pub fn is_match_set(&self) -> bool {
        *self == COM1D_A::MATCH_SET
    }
}
#[doc = "Field `COM1D` writer - Comparator D output mode"]
pub type COM1D_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR1C_SPEC, u8, COM1D_A, 2, O>;
impl<'a, const O: u8> COM1D_W<'a, O> {
    #[doc = "Normal port operation, OCix disconnected"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM1D_A::DISCONNECTED)
    }
    #[doc = "Toggle OCix on Compare Match (Might depend on WGM)"]
    #[inline(always)]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM1D_A::MATCH_TOGGLE)
    }
    #[doc = "Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)"]
    #[inline(always)]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM1D_A::MATCH_CLEAR)
    }
    #[doc = "Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)"]
    #[inline(always)]
    pub fn match_set(self) -> &'a mut W {
        self.variant(COM1D_A::MATCH_SET)
    }
}
#[doc = "Field `COM1B0S` reader - COM1B0 Shadow Bit"]
pub type COM1B0S_R = crate::BitReader<bool>;
#[doc = "Field `COM1B0S` writer - COM1B0 Shadow Bit"]
pub type COM1B0S_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1C_SPEC, bool, O>;
#[doc = "Field `COM1B1S` reader - COM1B1 Shadow Bit"]
pub type COM1B1S_R = crate::BitReader<bool>;
#[doc = "Field `COM1B1S` writer - COM1B1 Shadow Bit"]
pub type COM1B1S_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1C_SPEC, bool, O>;
#[doc = "Field `COM1A0S` reader - COM1A0 Shadow Bit"]
pub type COM1A0S_R = crate::BitReader<bool>;
#[doc = "Field `COM1A0S` writer - COM1A0 Shadow Bit"]
pub type COM1A0S_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1C_SPEC, bool, O>;
#[doc = "Field `COM1A1S` reader - COM1A1 Shadow Bit"]
pub type COM1A1S_R = crate::BitReader<bool>;
#[doc = "Field `COM1A1S` writer - COM1A1 Shadow Bit"]
pub type COM1A1S_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1C_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pulse Width Modulator D Enable"]
    #[inline(always)]
    pub fn pwm1d(&self) -> PWM1D_R {
        PWM1D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator D output mode"]
    #[inline(always)]
    pub fn com1d(&self) -> COM1D_R {
        COM1D_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - COM1B0 Shadow Bit"]
    #[inline(always)]
    pub fn com1b0s(&self) -> COM1B0S_R {
        COM1B0S_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM1B1 Shadow Bit"]
    #[inline(always)]
    pub fn com1b1s(&self) -> COM1B1S_R {
        COM1B1S_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - COM1A0 Shadow Bit"]
    #[inline(always)]
    pub fn com1a0s(&self) -> COM1A0S_R {
        COM1A0S_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - COM1A1 Shadow Bit"]
    #[inline(always)]
    pub fn com1a1s(&self) -> COM1A1S_R {
        COM1A1S_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Width Modulator D Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1d(&mut self) -> PWM1D_W<0> {
        PWM1D_W::new(self)
    }
    #[doc = "Bit 1 - Force Output Compare Match 1D"]
    #[inline(always)]
    #[must_use]
    pub fn foc1d(&mut self) -> FOC1D_W<1> {
        FOC1D_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator D output mode"]
    #[inline(always)]
    #[must_use]
    pub fn com1d(&mut self) -> COM1D_W<2> {
        COM1D_W::new(self)
    }
    #[doc = "Bit 4 - COM1B0 Shadow Bit"]
    #[inline(always)]
    #[must_use]
    pub fn com1b0s(&mut self) -> COM1B0S_W<4> {
        COM1B0S_W::new(self)
    }
    #[doc = "Bit 5 - COM1B1 Shadow Bit"]
    #[inline(always)]
    #[must_use]
    pub fn com1b1s(&mut self) -> COM1B1S_W<5> {
        COM1B1S_W::new(self)
    }
    #[doc = "Bit 6 - COM1A0 Shadow Bit"]
    #[inline(always)]
    #[must_use]
    pub fn com1a0s(&mut self) -> COM1A0S_W<6> {
        COM1A0S_W::new(self)
    }
    #[doc = "Bit 7 - COM1A1 Shadow Bit"]
    #[inline(always)]
    #[must_use]
    pub fn com1a1s(&mut self) -> COM1A1S_W<7> {
        COM1A1S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter Control Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr1c](index.html) module"]
pub struct TCCR1C_SPEC;
impl crate::RegisterSpec for TCCR1C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr1c::R](R) reader structure"]
impl crate::Readable for TCCR1C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr1c::W](W) writer structure"]
impl crate::Writable for TCCR1C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1C to value 0"]
impl crate::Resettable for TCCR1C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
