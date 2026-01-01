#[doc = "Register `TCCR3A` reader"]
pub struct R(crate::R<TCCR3A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR3A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR3A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR3A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR3A` writer"]
pub struct W(crate::W<TCCR3A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR3A_SPEC>;
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
impl From<crate::W<TCCR3A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR3A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WGM3` reader - Waveform Generation Mode"]
pub type WGM3_R = crate::FieldReader<u8, WGM3_A>;
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM3_A {
    #[doc = "0: Normal mode of operation"]
    NORMAL_MODE_OF_OPERATION = 0,
    #[doc = "1: PWM, phase correct, 8-bit"]
    PWM_PHASE_CORRECT_8_BIT = 1,
    #[doc = "2: PWM, phase correct, 9-bit"]
    PWM_PHASE_CORRECT_9_BIT = 2,
    #[doc = "3: PWM, phase correct, 10-bit"]
    PWM_PHASE_CORRECT_10_BIT = 3,
}
impl From<WGM3_A> for u8 {
    #[inline(always)]
    fn from(variant: WGM3_A) -> Self {
        variant as _
    }
}
impl WGM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WGM3_A {
        match self.bits {
            0 => WGM3_A::NORMAL_MODE_OF_OPERATION,
            1 => WGM3_A::PWM_PHASE_CORRECT_8_BIT,
            2 => WGM3_A::PWM_PHASE_CORRECT_9_BIT,
            3 => WGM3_A::PWM_PHASE_CORRECT_10_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE_OF_OPERATION`"]
    #[inline(always)]
    pub fn is_normal_mode_of_operation(&self) -> bool {
        *self == WGM3_A::NORMAL_MODE_OF_OPERATION
    }
    #[doc = "Checks if the value of the field is `PWM_PHASE_CORRECT_8_BIT`"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_8_bit(&self) -> bool {
        *self == WGM3_A::PWM_PHASE_CORRECT_8_BIT
    }
    #[doc = "Checks if the value of the field is `PWM_PHASE_CORRECT_9_BIT`"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_9_bit(&self) -> bool {
        *self == WGM3_A::PWM_PHASE_CORRECT_9_BIT
    }
    #[doc = "Checks if the value of the field is `PWM_PHASE_CORRECT_10_BIT`"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_10_bit(&self) -> bool {
        *self == WGM3_A::PWM_PHASE_CORRECT_10_BIT
    }
}
#[doc = "Field `WGM3` writer - Waveform Generation Mode"]
pub type WGM3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR3A_SPEC, u8, WGM3_A, 2, O>;
impl<'a, const O: u8> WGM3_W<'a, O> {
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn normal_mode_of_operation(self) -> &'a mut W {
        self.variant(WGM3_A::NORMAL_MODE_OF_OPERATION)
    }
    #[doc = "PWM, phase correct, 8-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_8_bit(self) -> &'a mut W {
        self.variant(WGM3_A::PWM_PHASE_CORRECT_8_BIT)
    }
    #[doc = "PWM, phase correct, 9-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_9_bit(self) -> &'a mut W {
        self.variant(WGM3_A::PWM_PHASE_CORRECT_9_BIT)
    }
    #[doc = "PWM, phase correct, 10-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_10_bit(self) -> &'a mut W {
        self.variant(WGM3_A::PWM_PHASE_CORRECT_10_BIT)
    }
}
#[doc = "Field `COM3C` reader - Compare Output Mode for Channel C"]
pub type COM3C_R = crate::FieldReader<u8, COM3C_A>;
#[doc = "Compare Output Mode for Channel C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM3C_A {
    #[doc = "0: Normal port operation, OCnA/OCnB/OCnC disconnected."]
    NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED = 0,
    #[doc = "1: Toggle OCnA/OCnB/OCnC on Compare Match."]
    TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH = 1,
    #[doc = "2: Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL = 2,
    #[doc = "3: Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL = 3,
}
impl From<COM3C_A> for u8 {
    #[inline(always)]
    fn from(variant: COM3C_A) -> Self {
        variant as _
    }
}
impl COM3C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COM3C_A {
        match self.bits {
            0 => COM3C_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED,
            1 => COM3C_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH,
            2 => COM3C_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL,
            3 => COM3C_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED`"]
    #[inline(always)]
    pub fn is_normal_port_operation_ocna_ocnb_ocnc_disconnected(&self) -> bool {
        *self == COM3C_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH`"]
    #[inline(always)]
    pub fn is_toggle_ocna_ocnb_ocnc_on_compare_match(&self) -> bool {
        *self == COM3C_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH
    }
    #[doc = "Checks if the value of the field is `CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_clear_ocna_ocnb_ocnc_on_compare_match_set_output_to_low_level(&self) -> bool {
        *self == COM3C_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_set_ocna_ocnb_ocnc_on_compare_match_set_output_to_high_level(&self) -> bool {
        *self == COM3C_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL
    }
}
#[doc = "Field `COM3C` writer - Compare Output Mode for Channel C"]
pub type COM3C_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR3A_SPEC, u8, COM3C_A, 2, O>;
impl<'a, const O: u8> COM3C_W<'a, O> {
    #[doc = "Normal port operation, OCnA/OCnB/OCnC disconnected."]
    #[inline(always)]
    pub fn normal_port_operation_ocna_ocnb_ocnc_disconnected(self) -> &'a mut W {
        self.variant(COM3C_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED)
    }
    #[doc = "Toggle OCnA/OCnB/OCnC on Compare Match."]
    #[inline(always)]
    pub fn toggle_ocna_ocnb_ocnc_on_compare_match(self) -> &'a mut W {
        self.variant(COM3C_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH)
    }
    #[doc = "Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    #[inline(always)]
    pub fn clear_ocna_ocnb_ocnc_on_compare_match_set_output_to_low_level(self) -> &'a mut W {
        self.variant(COM3C_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL)
    }
    #[doc = "Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    #[inline(always)]
    pub fn set_ocna_ocnb_ocnc_on_compare_match_set_output_to_high_level(self) -> &'a mut W {
        self.variant(COM3C_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL)
    }
}
#[doc = "Field `COM3B` reader - Compare Output Mode for Channel B"]
pub type COM3B_R = crate::FieldReader<u8, COM3B_A>;
#[doc = "Compare Output Mode for Channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM3B_A {
    #[doc = "0: Normal port operation, OCnA/OCnB/OCnC disconnected."]
    NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED = 0,
    #[doc = "1: Toggle OCnA/OCnB/OCnC on Compare Match."]
    TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH = 1,
    #[doc = "2: Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL = 2,
    #[doc = "3: Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL = 3,
}
impl From<COM3B_A> for u8 {
    #[inline(always)]
    fn from(variant: COM3B_A) -> Self {
        variant as _
    }
}
impl COM3B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COM3B_A {
        match self.bits {
            0 => COM3B_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED,
            1 => COM3B_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH,
            2 => COM3B_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL,
            3 => COM3B_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED`"]
    #[inline(always)]
    pub fn is_normal_port_operation_ocna_ocnb_ocnc_disconnected(&self) -> bool {
        *self == COM3B_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH`"]
    #[inline(always)]
    pub fn is_toggle_ocna_ocnb_ocnc_on_compare_match(&self) -> bool {
        *self == COM3B_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH
    }
    #[doc = "Checks if the value of the field is `CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_clear_ocna_ocnb_ocnc_on_compare_match_set_output_to_low_level(&self) -> bool {
        *self == COM3B_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_set_ocna_ocnb_ocnc_on_compare_match_set_output_to_high_level(&self) -> bool {
        *self == COM3B_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL
    }
}
#[doc = "Field `COM3B` writer - Compare Output Mode for Channel B"]
pub type COM3B_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR3A_SPEC, u8, COM3B_A, 2, O>;
impl<'a, const O: u8> COM3B_W<'a, O> {
    #[doc = "Normal port operation, OCnA/OCnB/OCnC disconnected."]
    #[inline(always)]
    pub fn normal_port_operation_ocna_ocnb_ocnc_disconnected(self) -> &'a mut W {
        self.variant(COM3B_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED)
    }
    #[doc = "Toggle OCnA/OCnB/OCnC on Compare Match."]
    #[inline(always)]
    pub fn toggle_ocna_ocnb_ocnc_on_compare_match(self) -> &'a mut W {
        self.variant(COM3B_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH)
    }
    #[doc = "Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    #[inline(always)]
    pub fn clear_ocna_ocnb_ocnc_on_compare_match_set_output_to_low_level(self) -> &'a mut W {
        self.variant(COM3B_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL)
    }
    #[doc = "Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    #[inline(always)]
    pub fn set_ocna_ocnb_ocnc_on_compare_match_set_output_to_high_level(self) -> &'a mut W {
        self.variant(COM3B_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL)
    }
}
#[doc = "Field `COM3A` reader - Compare Output Mode for Channel A"]
pub type COM3A_R = crate::FieldReader<u8, COM3A_A>;
#[doc = "Compare Output Mode for Channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM3A_A {
    #[doc = "0: Normal port operation, OCnA/OCnB/OCnC disconnected."]
    NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED = 0,
    #[doc = "1: Toggle OCnA/OCnB/OCnC on Compare Match."]
    TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH = 1,
    #[doc = "2: Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL = 2,
    #[doc = "3: Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL = 3,
}
impl From<COM3A_A> for u8 {
    #[inline(always)]
    fn from(variant: COM3A_A) -> Self {
        variant as _
    }
}
impl COM3A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COM3A_A {
        match self.bits {
            0 => COM3A_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED,
            1 => COM3A_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH,
            2 => COM3A_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL,
            3 => COM3A_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED`"]
    #[inline(always)]
    pub fn is_normal_port_operation_ocna_ocnb_ocnc_disconnected(&self) -> bool {
        *self == COM3A_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH`"]
    #[inline(always)]
    pub fn is_toggle_ocna_ocnb_ocnc_on_compare_match(&self) -> bool {
        *self == COM3A_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH
    }
    #[doc = "Checks if the value of the field is `CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_clear_ocna_ocnb_ocnc_on_compare_match_set_output_to_low_level(&self) -> bool {
        *self == COM3A_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_set_ocna_ocnb_ocnc_on_compare_match_set_output_to_high_level(&self) -> bool {
        *self == COM3A_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL
    }
}
#[doc = "Field `COM3A` writer - Compare Output Mode for Channel A"]
pub type COM3A_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR3A_SPEC, u8, COM3A_A, 2, O>;
impl<'a, const O: u8> COM3A_W<'a, O> {
    #[doc = "Normal port operation, OCnA/OCnB/OCnC disconnected."]
    #[inline(always)]
    pub fn normal_port_operation_ocna_ocnb_ocnc_disconnected(self) -> &'a mut W {
        self.variant(COM3A_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED)
    }
    #[doc = "Toggle OCnA/OCnB/OCnC on Compare Match."]
    #[inline(always)]
    pub fn toggle_ocna_ocnb_ocnc_on_compare_match(self) -> &'a mut W {
        self.variant(COM3A_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH)
    }
    #[doc = "Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    #[inline(always)]
    pub fn clear_ocna_ocnb_ocnc_on_compare_match_set_output_to_low_level(self) -> &'a mut W {
        self.variant(COM3A_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL)
    }
    #[doc = "Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    #[inline(always)]
    pub fn set_ocna_ocnb_ocnc_on_compare_match_set_output_to_high_level(self) -> &'a mut W {
        self.variant(COM3A_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL)
    }
}
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm3(&self) -> WGM3_R {
        WGM3_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Compare Output Mode for Channel C"]
    #[inline(always)]
    pub fn com3c(&self) -> COM3C_R {
        COM3C_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Compare Output Mode for Channel B"]
    #[inline(always)]
    pub fn com3b(&self) -> COM3B_R {
        COM3B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode for Channel A"]
    #[inline(always)]
    pub fn com3a(&self) -> COM3A_R {
        COM3A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm3(&mut self) -> WGM3_W<0> {
        WGM3_W::new(self)
    }
    #[doc = "Bits 2:3 - Compare Output Mode for Channel C"]
    #[inline(always)]
    #[must_use]
    pub fn com3c(&mut self) -> COM3C_W<2> {
        COM3C_W::new(self)
    }
    #[doc = "Bits 4:5 - Compare Output Mode for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn com3b(&mut self) -> COM3B_W<4> {
        COM3B_W::new(self)
    }
    #[doc = "Bits 6:7 - Compare Output Mode for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn com3a(&mut self) -> COM3A_W<6> {
        COM3A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter3 Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr3a](index.html) module"]
pub struct TCCR3A_SPEC;
impl crate::RegisterSpec for TCCR3A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr3a::R](R) reader structure"]
impl crate::Readable for TCCR3A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr3a::W](W) writer structure"]
impl crate::Writable for TCCR3A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR3A to value 0"]
impl crate::Resettable for TCCR3A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
