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
pub type WGM5_R = crate::FieldReader<u8, WGM5_A>;
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM5_A {
    #[doc = "0: Normal mode of operation"]
    NORMAL_MODE_OF_OPERATION = 0,
    #[doc = "1: PWM, phase correct, 8-bit"]
    PWM_PHASE_CORRECT_8_BIT = 1,
    #[doc = "2: PWM, phase correct, 9-bit"]
    PWM_PHASE_CORRECT_9_BIT = 2,
    #[doc = "3: PWM, phase correct, 10-bit"]
    PWM_PHASE_CORRECT_10_BIT = 3,
}
impl From<WGM5_A> for u8 {
    #[inline(always)]
    fn from(variant: WGM5_A) -> Self {
        variant as _
    }
}
impl WGM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WGM5_A {
        match self.bits {
            0 => WGM5_A::NORMAL_MODE_OF_OPERATION,
            1 => WGM5_A::PWM_PHASE_CORRECT_8_BIT,
            2 => WGM5_A::PWM_PHASE_CORRECT_9_BIT,
            3 => WGM5_A::PWM_PHASE_CORRECT_10_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE_OF_OPERATION`"]
    #[inline(always)]
    pub fn is_normal_mode_of_operation(&self) -> bool {
        *self == WGM5_A::NORMAL_MODE_OF_OPERATION
    }
    #[doc = "Checks if the value of the field is `PWM_PHASE_CORRECT_8_BIT`"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_8_bit(&self) -> bool {
        *self == WGM5_A::PWM_PHASE_CORRECT_8_BIT
    }
    #[doc = "Checks if the value of the field is `PWM_PHASE_CORRECT_9_BIT`"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_9_bit(&self) -> bool {
        *self == WGM5_A::PWM_PHASE_CORRECT_9_BIT
    }
    #[doc = "Checks if the value of the field is `PWM_PHASE_CORRECT_10_BIT`"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_10_bit(&self) -> bool {
        *self == WGM5_A::PWM_PHASE_CORRECT_10_BIT
    }
}
#[doc = "Field `WGM5` writer - Waveform Generation Mode"]
pub type WGM5_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR5A_SPEC, u8, WGM5_A, 2, O>;
impl<'a, const O: u8> WGM5_W<'a, O> {
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn normal_mode_of_operation(self) -> &'a mut W {
        self.variant(WGM5_A::NORMAL_MODE_OF_OPERATION)
    }
    #[doc = "PWM, phase correct, 8-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_8_bit(self) -> &'a mut W {
        self.variant(WGM5_A::PWM_PHASE_CORRECT_8_BIT)
    }
    #[doc = "PWM, phase correct, 9-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_9_bit(self) -> &'a mut W {
        self.variant(WGM5_A::PWM_PHASE_CORRECT_9_BIT)
    }
    #[doc = "PWM, phase correct, 10-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_10_bit(self) -> &'a mut W {
        self.variant(WGM5_A::PWM_PHASE_CORRECT_10_BIT)
    }
}
#[doc = "Field `COM5C` reader - Compare Output Mode for Channel C"]
pub type COM5C_R = crate::FieldReader<u8, COM5C_A>;
#[doc = "Compare Output Mode for Channel C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM5C_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPERATION = 0,
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
    pub fn variant(&self) -> Option<COM5C_A> {
        match self.bits {
            0 => Some(COM5C_A::NORMAL_OPERATION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION`"]
    #[inline(always)]
    pub fn is_normal_operation(&self) -> bool {
        *self == COM5C_A::NORMAL_OPERATION
    }
}
#[doc = "Field `COM5C` writer - Compare Output Mode for Channel C"]
pub type COM5C_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TCCR5A_SPEC, u8, COM5C_A, 2, O>;
impl<'a, const O: u8> COM5C_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_operation(self) -> &'a mut W {
        self.variant(COM5C_A::NORMAL_OPERATION)
    }
}
#[doc = "Field `COM5B` reader - Compare Output Mode for Channel B"]
pub type COM5B_R = crate::FieldReader<u8, COM5B_A>;
#[doc = "Compare Output Mode for Channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM5B_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPERATION = 0,
}
impl From<COM5B_A> for u8 {
    #[inline(always)]
    fn from(variant: COM5B_A) -> Self {
        variant as _
    }
}
impl COM5B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COM5B_A> {
        match self.bits {
            0 => Some(COM5B_A::NORMAL_OPERATION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION`"]
    #[inline(always)]
    pub fn is_normal_operation(&self) -> bool {
        *self == COM5B_A::NORMAL_OPERATION
    }
}
#[doc = "Field `COM5B` writer - Compare Output Mode for Channel B"]
pub type COM5B_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TCCR5A_SPEC, u8, COM5B_A, 2, O>;
impl<'a, const O: u8> COM5B_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_operation(self) -> &'a mut W {
        self.variant(COM5B_A::NORMAL_OPERATION)
    }
}
#[doc = "Field `COM5A` reader - Compare Output Mode for Channel A"]
pub type COM5A_R = crate::FieldReader<u8, COM5A_A>;
#[doc = "Compare Output Mode for Channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM5A_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPERATION = 0,
}
impl From<COM5A_A> for u8 {
    #[inline(always)]
    fn from(variant: COM5A_A) -> Self {
        variant as _
    }
}
impl COM5A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COM5A_A> {
        match self.bits {
            0 => Some(COM5A_A::NORMAL_OPERATION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION`"]
    #[inline(always)]
    pub fn is_normal_operation(&self) -> bool {
        *self == COM5A_A::NORMAL_OPERATION
    }
}
#[doc = "Field `COM5A` writer - Compare Output Mode for Channel A"]
pub type COM5A_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TCCR5A_SPEC, u8, COM5A_A, 2, O>;
impl<'a, const O: u8> COM5A_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_operation(self) -> &'a mut W {
        self.variant(COM5A_A::NORMAL_OPERATION)
    }
}
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm5(&self) -> WGM5_R {
        WGM5_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Compare Output Mode for Channel C"]
    #[inline(always)]
    pub fn com5c(&self) -> COM5C_R {
        COM5C_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Compare Output Mode for Channel B"]
    #[inline(always)]
    pub fn com5b(&self) -> COM5B_R {
        COM5B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode for Channel A"]
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
    #[doc = "Bits 2:3 - Compare Output Mode for Channel C"]
    #[inline(always)]
    #[must_use]
    pub fn com5c(&mut self) -> COM5C_W<2> {
        COM5C_W::new(self)
    }
    #[doc = "Bits 4:5 - Compare Output Mode for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn com5b(&mut self) -> COM5B_W<4> {
        COM5B_W::new(self)
    }
    #[doc = "Bits 6:7 - Compare Output Mode for Channel A"]
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
