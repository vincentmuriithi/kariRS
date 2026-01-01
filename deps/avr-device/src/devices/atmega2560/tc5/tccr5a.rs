#[doc = "Register `TCCR5A` reader"]
pub struct R(crate::R<TCCR5A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR5A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR5A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR5A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR5A` writer"]
pub struct W(crate::W<TCCR5A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR5A_SPEC>;
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
impl From<crate::W<TCCR5A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR5A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WGM5` reader - Waveform Generation Mode"]
pub type WGM5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WGM5` writer - Waveform Generation Mode"]
pub type WGM5_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR5A_SPEC, u8, u8, 2, O>;
#[doc = "Field `COM5C` reader - Compare Output Mode 5C, bits"]
pub type COM5C_R = crate::FieldReader<u8, COM5C_A>;
#[doc = "Compare Output Mode 5C, bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM5C_A {
    #[doc = "0: Normal port operation, OCix disconnected"]
    DISCONNECTED = 0,
    #[doc = "1: Toggle OCix on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE = 1,
    #[doc = "2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)"]
    MATCH_CLEAR = 2,
    #[doc = "3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)"]
    MATCH_SET = 3,
}
impl From<COM5C_A> for u8 {
    #[inline(always)]
    fn from(variant: COM5C_A) -> Self {
        variant as _
    }
}
impl COM5C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COM5C_A {
        match self.bits {
            0 => COM5C_A::DISCONNECTED,
            1 => COM5C_A::MATCH_TOGGLE,
            2 => COM5C_A::MATCH_CLEAR,
            3 => COM5C_A::MATCH_SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == COM5C_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `MATCH_TOGGLE`"]
    #[inline(always)]
    pub fn is_match_toggle(&self) -> bool {
        *self == COM5C_A::MATCH_TOGGLE
    }
    #[doc = "Checks if the value of the field is `MATCH_CLEAR`"]
    #[inline(always)]
    pub fn is_match_clear(&self) -> bool {
        *self == COM5C_A::MATCH_CLEAR
    }
    #[doc = "Checks if the value of the field is `MATCH_SET`"]
    #[inline(always)]
    pub fn is_match_set(&self) -> bool {
        *self == COM5C_A::MATCH_SET
    }
}
#[doc = "Field `COM5C` writer - Compare Output Mode 5C, bits"]
pub type COM5C_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR5A_SPEC, u8, COM5C_A, 2, O>;
impl<'a, const O: u8> COM5C_W<'a, O> {
    #[doc = "Normal port operation, OCix disconnected"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM5C_A::DISCONNECTED)
    }
    #[doc = "Toggle OCix on Compare Match (Might depend on WGM)"]
    #[inline(always)]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM5C_A::MATCH_TOGGLE)
    }
    #[doc = "Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)"]
    #[inline(always)]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM5C_A::MATCH_CLEAR)
    }
    #[doc = "Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)"]
    #[inline(always)]
    pub fn match_set(self) -> &'a mut W {
        self.variant(COM5C_A::MATCH_SET)
    }
}
#[doc = "Field `COM5B` reader - Compare Output Mode 5B, bits"]
pub use COM5C_R as COM5B_R;
#[doc = "Field `COM5A` reader - Compare Output Mode 1A, bits"]
pub use COM5C_R as COM5A_R;
#[doc = "Field `COM5B` writer - Compare Output Mode 5B, bits"]
pub use COM5C_W as COM5B_W;
#[doc = "Field `COM5A` writer - Compare Output Mode 1A, bits"]
pub use COM5C_W as COM5A_W;
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm5(&self) -> WGM5_R {
        WGM5_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Compare Output Mode 5C, bits"]
    #[inline(always)]
    pub fn com5c(&self) -> COM5C_R {
        COM5C_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Compare Output Mode 5B, bits"]
    #[inline(always)]
    pub fn com5b(&self) -> COM5B_R {
        COM5B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode 1A, bits"]
    #[inline(always)]
    pub fn com5a(&self) -> COM5A_R {
        COM5A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm5(&mut self) -> WGM5_W<0> {
        WGM5_W::new(self)
    }
    #[doc = "Bits 2:3 - Compare Output Mode 5C, bits"]
    #[inline(always)]
    #[must_use]
    pub fn com5c(&mut self) -> COM5C_W<2> {
        COM5C_W::new(self)
    }
    #[doc = "Bits 4:5 - Compare Output Mode 5B, bits"]
    #[inline(always)]
    #[must_use]
    pub fn com5b(&mut self) -> COM5B_W<4> {
        COM5B_W::new(self)
    }
    #[doc = "Bits 6:7 - Compare Output Mode 1A, bits"]
    #[inline(always)]
    #[must_use]
    pub fn com5a(&mut self) -> COM5A_W<6> {
        COM5A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter5 Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr5a](index.html) module"]
pub struct TCCR5A_SPEC;
impl crate::RegisterSpec for TCCR5A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr5a::R](R) reader structure"]
impl crate::Readable for TCCR5A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr5a::W](W) writer structure"]
impl crate::Writable for TCCR5A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR5A to value 0"]
impl crate::Resettable for TCCR5A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
