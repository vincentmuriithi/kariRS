#[doc = "Register `TCCR4A` reader"]
pub struct R(crate::R<TCCR4A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR4A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR4A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR4A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR4A` writer"]
pub struct W(crate::W<TCCR4A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR4A_SPEC>;
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
impl From<crate::W<TCCR4A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR4A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WGM4` reader - Waveform Generation Mode"]
pub type WGM4_R = crate::FieldReader<u8, WGM4_A>;
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM4_A {
    #[doc = "0: Normal mode of operation"]
    NORMAL_MODE_OF_OPERATION = 0,
    #[doc = "1: PWM, phase correct, 8-bit"]
    PWM_PHASE_CORRECT_8_BIT = 1,
    #[doc = "2: PWM, phase correct, 9-bit"]
    PWM_PHASE_CORRECT_9_BIT = 2,
    #[doc = "3: PWM, phase correct, 10-bit"]
    PWM_PHASE_CORRECT_10_BIT = 3,
}
impl From<WGM4_A> for u8 {
    #[inline(always)]
    fn from(variant: WGM4_A) -> Self {
        variant as _
    }
}
impl WGM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WGM4_A {
        match self.bits {
            0 => WGM4_A::NORMAL_MODE_OF_OPERATION,
            1 => WGM4_A::PWM_PHASE_CORRECT_8_BIT,
            2 => WGM4_A::PWM_PHASE_CORRECT_9_BIT,
            3 => WGM4_A::PWM_PHASE_CORRECT_10_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE_OF_OPERATION`"]
    #[inline(always)]
    pub fn is_normal_mode_of_operation(&self) -> bool {
        *self == WGM4_A::NORMAL_MODE_OF_OPERATION
    }
    #[doc = "Checks if the value of the field is `PWM_PHASE_CORRECT_8_BIT`"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_8_bit(&self) -> bool {
        *self == WGM4_A::PWM_PHASE_CORRECT_8_BIT
    }
    #[doc = "Checks if the value of the field is `PWM_PHASE_CORRECT_9_BIT`"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_9_bit(&self) -> bool {
        *self == WGM4_A::PWM_PHASE_CORRECT_9_BIT
    }
    #[doc = "Checks if the value of the field is `PWM_PHASE_CORRECT_10_BIT`"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_10_bit(&self) -> bool {
        *self == WGM4_A::PWM_PHASE_CORRECT_10_BIT
    }
}
#[doc = "Field `WGM4` writer - Waveform Generation Mode"]
pub type WGM4_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR4A_SPEC, u8, WGM4_A, 2, O>;
impl<'a, const O: u8> WGM4_W<'a, O> {
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn normal_mode_of_operation(self) -> &'a mut W {
        self.variant(WGM4_A::NORMAL_MODE_OF_OPERATION)
    }
    #[doc = "PWM, phase correct, 8-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_8_bit(self) -> &'a mut W {
        self.variant(WGM4_A::PWM_PHASE_CORRECT_8_BIT)
    }
    #[doc = "PWM, phase correct, 9-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_9_bit(self) -> &'a mut W {
        self.variant(WGM4_A::PWM_PHASE_CORRECT_9_BIT)
    }
    #[doc = "PWM, phase correct, 10-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_10_bit(self) -> &'a mut W {
        self.variant(WGM4_A::PWM_PHASE_CORRECT_10_BIT)
    }
}
#[doc = "Field `COM4C` reader - Compare Output Mode for Channel C"]
pub type COM4C_R = crate::FieldReader<u8, COM4C_A>;
#[doc = "Compare Output Mode for Channel C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM4C_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPERATION = 0,
}
impl From<COM4C_A> for u8 {
    #[inline(always)]
    fn from(variant: COM4C_A) -> Self {
        variant as _
    }
}
impl COM4C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COM4C_A> {
        match self.bits {
            0 => Some(COM4C_A::NORMAL_OPERATION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION`"]
    #[inline(always)]
    pub fn is_normal_operation(&self) -> bool {
        *self == COM4C_A::NORMAL_OPERATION
    }
}
#[doc = "Field `COM4C` writer - Compare Output Mode for Channel C"]
pub type COM4C_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TCCR4A_SPEC, u8, COM4C_A, 2, O>;
impl<'a, const O: u8> COM4C_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_operation(self) -> &'a mut W {
        self.variant(COM4C_A::NORMAL_OPERATION)
    }
}
#[doc = "Field `COM4B` reader - Compare Output Mode for Channel B"]
pub type COM4B_R = crate::FieldReader<u8, COM4B_A>;
#[doc = "Compare Output Mode for Channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM4B_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPERATION = 0,
}
impl From<COM4B_A> for u8 {
    #[inline(always)]
    fn from(variant: COM4B_A) -> Self {
        variant as _
    }
}
impl COM4B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COM4B_A> {
        match self.bits {
            0 => Some(COM4B_A::NORMAL_OPERATION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION`"]
    #[inline(always)]
    pub fn is_normal_operation(&self) -> bool {
        *self == COM4B_A::NORMAL_OPERATION
    }
}
#[doc = "Field `COM4B` writer - Compare Output Mode for Channel B"]
pub type COM4B_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TCCR4A_SPEC, u8, COM4B_A, 2, O>;
impl<'a, const O: u8> COM4B_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_operation(self) -> &'a mut W {
        self.variant(COM4B_A::NORMAL_OPERATION)
    }
}
#[doc = "Field `COM4A` reader - Compare Output Mode for Channel A"]
pub type COM4A_R = crate::FieldReader<u8, COM4A_A>;
#[doc = "Compare Output Mode for Channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM4A_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPERATION = 0,
}
impl From<COM4A_A> for u8 {
    #[inline(always)]
    fn from(variant: COM4A_A) -> Self {
        variant as _
    }
}
impl COM4A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COM4A_A> {
        match self.bits {
            0 => Some(COM4A_A::NORMAL_OPERATION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION`"]
    #[inline(always)]
    pub fn is_normal_operation(&self) -> bool {
        *self == COM4A_A::NORMAL_OPERATION
    }
}
#[doc = "Field `COM4A` writer - Compare Output Mode for Channel A"]
pub type COM4A_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TCCR4A_SPEC, u8, COM4A_A, 2, O>;
impl<'a, const O: u8> COM4A_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_operation(self) -> &'a mut W {
        self.variant(COM4A_A::NORMAL_OPERATION)
    }
}
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm4(&self) -> WGM4_R {
        WGM4_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Compare Output Mode for Channel C"]
    #[inline(always)]
    pub fn com4c(&self) -> COM4C_R {
        COM4C_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Compare Output Mode for Channel B"]
    #[inline(always)]
    pub fn com4b(&self) -> COM4B_R {
        COM4B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode for Channel A"]
    #[inline(always)]
    pub fn com4a(&self) -> COM4A_R {
        COM4A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm4(&mut self) -> WGM4_W<0> {
        WGM4_W::new(self)
    }
    #[doc = "Bits 2:3 - Compare Output Mode for Channel C"]
    #[inline(always)]
    #[must_use]
    pub fn com4c(&mut self) -> COM4C_W<2> {
        COM4C_W::new(self)
    }
    #[doc = "Bits 4:5 - Compare Output Mode for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn com4b(&mut self) -> COM4B_W<4> {
        COM4B_W::new(self)
    }
    #[doc = "Bits 6:7 - Compare Output Mode for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn com4a(&mut self) -> COM4A_W<6> {
        COM4A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter4 Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr4a](index.html) module"]
pub struct TCCR4A_SPEC;
impl crate::RegisterSpec for TCCR4A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr4a::R](R) reader structure"]
impl crate::Readable for TCCR4A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr4a::W](W) writer structure"]
impl crate::Writable for TCCR4A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4A to value 0"]
impl crate::Resettable for TCCR4A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
