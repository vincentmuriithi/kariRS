#[doc = "Register `TCCR4B` reader"]
pub struct R(crate::R<TCCR4B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR4B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR4B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR4B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR4B` writer"]
pub struct W(crate::W<TCCR4B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR4B_SPEC>;
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
impl From<crate::W<TCCR4B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR4B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS4` reader - Clock Select"]
pub type CS4_R = crate::FieldReader<u8, CS4_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS4_A {
    #[doc = "0: No clock source (Timer/Counter stopped)"]
    NO_CLOCK_SOURCE_TIMER_COUNTER_STOPPED = 0,
    #[doc = "1: clk_IO/1 (no prescaling)"]
    CLK_IO_1_NO_PRESCALING = 1,
    #[doc = "2: clk_IO/8 (from prescaler)"]
    CLK_IO_8_FROM_PRESCALER = 2,
    #[doc = "3: clk_IO/64 (from prescaler)"]
    CLK_IO_64_FROM_PRESCALER = 3,
    #[doc = "4: clk_IO/256 (from prescaler)"]
    CLK_IO_256_FROM_PRESCALER = 4,
    #[doc = "5: clk_IO/1024 (from prescaler)"]
    CLK_IO_1024_FROM_PRESCALER = 5,
}
impl From<CS4_A> for u8 {
    #[inline(always)]
    fn from(variant: CS4_A) -> Self {
        variant as _
    }
}
impl CS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CS4_A> {
        match self.bits {
            0 => Some(CS4_A::NO_CLOCK_SOURCE_TIMER_COUNTER_STOPPED),
            1 => Some(CS4_A::CLK_IO_1_NO_PRESCALING),
            2 => Some(CS4_A::CLK_IO_8_FROM_PRESCALER),
            3 => Some(CS4_A::CLK_IO_64_FROM_PRESCALER),
            4 => Some(CS4_A::CLK_IO_256_FROM_PRESCALER),
            5 => Some(CS4_A::CLK_IO_1024_FROM_PRESCALER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK_SOURCE_TIMER_COUNTER_STOPPED`"]
    #[inline(always)]
    pub fn is_no_clock_source_timer_counter_stopped(&self) -> bool {
        *self == CS4_A::NO_CLOCK_SOURCE_TIMER_COUNTER_STOPPED
    }
    #[doc = "Checks if the value of the field is `CLK_IO_1_NO_PRESCALING`"]
    #[inline(always)]
    pub fn is_clk_io_1_no_prescaling(&self) -> bool {
        *self == CS4_A::CLK_IO_1_NO_PRESCALING
    }
    #[doc = "Checks if the value of the field is `CLK_IO_8_FROM_PRESCALER`"]
    #[inline(always)]
    pub fn is_clk_io_8_from_prescaler(&self) -> bool {
        *self == CS4_A::CLK_IO_8_FROM_PRESCALER
    }
    #[doc = "Checks if the value of the field is `CLK_IO_64_FROM_PRESCALER`"]
    #[inline(always)]
    pub fn is_clk_io_64_from_prescaler(&self) -> bool {
        *self == CS4_A::CLK_IO_64_FROM_PRESCALER
    }
    #[doc = "Checks if the value of the field is `CLK_IO_256_FROM_PRESCALER`"]
    #[inline(always)]
    pub fn is_clk_io_256_from_prescaler(&self) -> bool {
        *self == CS4_A::CLK_IO_256_FROM_PRESCALER
    }
    #[doc = "Checks if the value of the field is `CLK_IO_1024_FROM_PRESCALER`"]
    #[inline(always)]
    pub fn is_clk_io_1024_from_prescaler(&self) -> bool {
        *self == CS4_A::CLK_IO_1024_FROM_PRESCALER
    }
}
#[doc = "Field `CS4` writer - Clock Select"]
pub type CS4_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TCCR4B_SPEC, u8, CS4_A, 3, O>;
impl<'a, const O: u8> CS4_W<'a, O> {
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock_source_timer_counter_stopped(self) -> &'a mut W {
        self.variant(CS4_A::NO_CLOCK_SOURCE_TIMER_COUNTER_STOPPED)
    }
    #[doc = "clk_IO/1 (no prescaling)"]
    #[inline(always)]
    pub fn clk_io_1_no_prescaling(self) -> &'a mut W {
        self.variant(CS4_A::CLK_IO_1_NO_PRESCALING)
    }
    #[doc = "clk_IO/8 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_8_from_prescaler(self) -> &'a mut W {
        self.variant(CS4_A::CLK_IO_8_FROM_PRESCALER)
    }
    #[doc = "clk_IO/64 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_64_from_prescaler(self) -> &'a mut W {
        self.variant(CS4_A::CLK_IO_64_FROM_PRESCALER)
    }
    #[doc = "clk_IO/256 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_256_from_prescaler(self) -> &'a mut W {
        self.variant(CS4_A::CLK_IO_256_FROM_PRESCALER)
    }
    #[doc = "clk_IO/1024 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_1024_from_prescaler(self) -> &'a mut W {
        self.variant(CS4_A::CLK_IO_1024_FROM_PRESCALER)
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
pub type WGM4_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR4B_SPEC, u8, WGM4_A, 2, O>;
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
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::BitReader<bool>;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4B_SPEC, bool, O>;
#[doc = "Field `ICES4` reader - Input Capture 4 Edge Select"]
pub type ICES4_R = crate::BitReader<bool>;
#[doc = "Field `ICES4` writer - Input Capture 4 Edge Select"]
pub type ICES4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4B_SPEC, bool, O>;
#[doc = "Field `ICNC4` reader - Input Capture 4 Noise Canceller"]
pub type ICNC4_R = crate::BitReader<bool>;
#[doc = "Field `ICNC4` writer - Input Capture 4 Noise Canceller"]
pub type ICNC4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4B_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn cs4(&self) -> CS4_R {
        CS4_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm4(&self) -> WGM4_R {
        WGM4_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input Capture 4 Edge Select"]
    #[inline(always)]
    pub fn ices4(&self) -> ICES4_R {
        ICES4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Capture 4 Noise Canceller"]
    #[inline(always)]
    pub fn icnc4(&self) -> ICNC4_R {
        ICNC4_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cs4(&mut self) -> CS4_W<0> {
        CS4_W::new(self)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm4(&mut self) -> WGM4_W<3> {
        WGM4_W::new(self)
    }
    #[doc = "Bit 5 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<5> {
        RES_W::new(self)
    }
    #[doc = "Bit 6 - Input Capture 4 Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ices4(&mut self) -> ICES4_W<6> {
        ICES4_W::new(self)
    }
    #[doc = "Bit 7 - Input Capture 4 Noise Canceller"]
    #[inline(always)]
    #[must_use]
    pub fn icnc4(&mut self) -> ICNC4_W<7> {
        ICNC4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter4 Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr4b](index.html) module"]
pub struct TCCR4B_SPEC;
impl crate::RegisterSpec for TCCR4B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr4b::R](R) reader structure"]
impl crate::Readable for TCCR4B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr4b::W](W) writer structure"]
impl crate::Writable for TCCR4B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4B to value 0"]
impl crate::Resettable for TCCR4B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
