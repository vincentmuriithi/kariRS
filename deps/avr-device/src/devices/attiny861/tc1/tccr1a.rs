#[doc = "Register `TCCR1A` reader"]
pub struct R(crate::R<TCCR1A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR1A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR1A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR1A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR1A` writer"]
pub struct W(crate::W<TCCR1A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR1A_SPEC>;
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
impl From<crate::W<TCCR1A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR1A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM1B` reader - Pulse Width Modulator Enable"]
pub type PWM1B_R = crate::BitReader<bool>;
#[doc = "Field `PWM1B` writer - Pulse Width Modulator Enable"]
pub type PWM1B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1A_SPEC, bool, O>;
#[doc = "Field `PWM1A` reader - Pulse Width Modulator Enable"]
pub type PWM1A_R = crate::BitReader<bool>;
#[doc = "Field `PWM1A` writer - Pulse Width Modulator Enable"]
pub type PWM1A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1A_SPEC, bool, O>;
#[doc = "Field `FOC1B` writer - Force Output Compare Match 1B"]
pub type FOC1B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1A_SPEC, bool, O>;
#[doc = "Field `FOC1A` writer - Force Output Compare Match 1A"]
pub type FOC1A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1A_SPEC, bool, O>;
#[doc = "Field `COM1B` reader - Compare Output Mode, Bits"]
pub type COM1B_R = crate::FieldReader<u8, COM1B_A>;
#[doc = "Compare Output Mode, Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM1B_A {
    #[doc = "0: Normal port operation, OCix disconnected"]
    DISCONNECTED = 0,
    #[doc = "1: Toggle OCix on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE = 1,
    #[doc = "2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)"]
    MATCH_CLEAR = 2,
    #[doc = "3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)"]
    MATCH_SET = 3,
}
impl From<COM1B_A> for u8 {
    #[inline(always)]
    fn from(variant: COM1B_A) -> Self {
        variant as _
    }
}
impl COM1B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COM1B_A {
        match self.bits {
            0 => COM1B_A::DISCONNECTED,
            1 => COM1B_A::MATCH_TOGGLE,
            2 => COM1B_A::MATCH_CLEAR,
            3 => COM1B_A::MATCH_SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == COM1B_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `MATCH_TOGGLE`"]
    #[inline(always)]
    pub fn is_match_toggle(&self) -> bool {
        *self == COM1B_A::MATCH_TOGGLE
    }
    #[doc = "Checks if the value of the field is `MATCH_CLEAR`"]
    #[inline(always)]
    pub fn is_match_clear(&self) -> bool {
        *self == COM1B_A::MATCH_CLEAR
    }
    #[doc = "Checks if the value of the field is `MATCH_SET`"]
    #[inline(always)]
    pub fn is_match_set(&self) -> bool {
        *self == COM1B_A::MATCH_SET
    }
}
#[doc = "Field `COM1B` writer - Compare Output Mode, Bits"]
pub type COM1B_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR1A_SPEC, u8, COM1B_A, 2, O>;
impl<'a, const O: u8> COM1B_W<'a, O> {
    #[doc = "Normal port operation, OCix disconnected"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM1B_A::DISCONNECTED)
    }
    #[doc = "Toggle OCix on Compare Match (Might depend on WGM)"]
    #[inline(always)]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM1B_A::MATCH_TOGGLE)
    }
    #[doc = "Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)"]
    #[inline(always)]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM1B_A::MATCH_CLEAR)
    }
    #[doc = "Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)"]
    #[inline(always)]
    pub fn match_set(self) -> &'a mut W {
        self.variant(COM1B_A::MATCH_SET)
    }
}
#[doc = "Field `COM1A` reader - Compare Output Mode, Bits"]
pub use COM1B_R as COM1A_R;
#[doc = "Field `COM1A` writer - Compare Output Mode, Bits"]
pub use COM1B_W as COM1A_W;
impl R {
    #[doc = "Bit 0 - Pulse Width Modulator Enable"]
    #[inline(always)]
    pub fn pwm1b(&self) -> PWM1B_R {
        PWM1B_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Width Modulator Enable"]
    #[inline(always)]
    pub fn pwm1a(&self) -> PWM1A_R {
        PWM1A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Compare Output Mode, Bits"]
    #[inline(always)]
    pub fn com1b(&self) -> COM1B_R {
        COM1B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode, Bits"]
    #[inline(always)]
    pub fn com1a(&self) -> COM1A_R {
        COM1A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Width Modulator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1b(&mut self) -> PWM1B_W<0> {
        PWM1B_W::new(self)
    }
    #[doc = "Bit 1 - Pulse Width Modulator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1a(&mut self) -> PWM1A_W<1> {
        PWM1A_W::new(self)
    }
    #[doc = "Bit 2 - Force Output Compare Match 1B"]
    #[inline(always)]
    #[must_use]
    pub fn foc1b(&mut self) -> FOC1B_W<2> {
        FOC1B_W::new(self)
    }
    #[doc = "Bit 3 - Force Output Compare Match 1A"]
    #[inline(always)]
    #[must_use]
    pub fn foc1a(&mut self) -> FOC1A_W<3> {
        FOC1A_W::new(self)
    }
    #[doc = "Bits 4:5 - Compare Output Mode, Bits"]
    #[inline(always)]
    #[must_use]
    pub fn com1b(&mut self) -> COM1B_W<4> {
        COM1B_W::new(self)
    }
    #[doc = "Bits 6:7 - Compare Output Mode, Bits"]
    #[inline(always)]
    #[must_use]
    pub fn com1a(&mut self) -> COM1A_W<6> {
        COM1A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr1a](index.html) module"]
pub struct TCCR1A_SPEC;
impl crate::RegisterSpec for TCCR1A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr1a::R](R) reader structure"]
impl crate::Readable for TCCR1A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr1a::W](W) writer structure"]
impl crate::Writable for TCCR1A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1A to value 0"]
impl crate::Resettable for TCCR1A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
