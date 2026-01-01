#[doc = "Register `TCCR4D` reader"]
pub struct R(crate::R<TCCR4D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR4D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR4D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR4D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR4D` writer"]
pub struct W(crate::W<TCCR4D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR4D_SPEC>;
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
impl From<crate::W<TCCR4D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR4D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WGM4` reader - Waveform Generation Mode bits"]
pub type WGM4_R = crate::FieldReader<u8, WGM4_A>;
#[doc = "Waveform Generation Mode bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM4_A {
    #[doc = "0: Fast PWM, Update: *TOP*, Flag: *TOP*"]
    PWM_FAST = 0,
    #[doc = "1: Phase and Frequency Correct PWM, Update: *BOTTOM*, Flag: *BOTTOM*"]
    PWM_CORRECT = 1,
    #[doc = "2: PWM6 / Single-slope, Update: *TOP*, Flag: *TOP*"]
    PWM_SINGLE_SLOPE = 2,
    #[doc = "3: PWM6 / Dual-slope, Update: *BOTTOM*, Flag: *BOTTOM*"]
    PWM_DUAL_SLOPE = 3,
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
            0 => WGM4_A::PWM_FAST,
            1 => WGM4_A::PWM_CORRECT,
            2 => WGM4_A::PWM_SINGLE_SLOPE,
            3 => WGM4_A::PWM_DUAL_SLOPE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_FAST`"]
    #[inline(always)]
    pub fn is_pwm_fast(&self) -> bool {
        *self == WGM4_A::PWM_FAST
    }
    #[doc = "Checks if the value of the field is `PWM_CORRECT`"]
    #[inline(always)]
    pub fn is_pwm_correct(&self) -> bool {
        *self == WGM4_A::PWM_CORRECT
    }
    #[doc = "Checks if the value of the field is `PWM_SINGLE_SLOPE`"]
    #[inline(always)]
    pub fn is_pwm_single_slope(&self) -> bool {
        *self == WGM4_A::PWM_SINGLE_SLOPE
    }
    #[doc = "Checks if the value of the field is `PWM_DUAL_SLOPE`"]
    #[inline(always)]
    pub fn is_pwm_dual_slope(&self) -> bool {
        *self == WGM4_A::PWM_DUAL_SLOPE
    }
}
#[doc = "Field `WGM4` writer - Waveform Generation Mode bits"]
pub type WGM4_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR4D_SPEC, u8, WGM4_A, 2, O>;
impl<'a, const O: u8> WGM4_W<'a, O> {
    #[doc = "Fast PWM, Update: *TOP*, Flag: *TOP*"]
    #[inline(always)]
    pub fn pwm_fast(self) -> &'a mut W {
        self.variant(WGM4_A::PWM_FAST)
    }
    #[doc = "Phase and Frequency Correct PWM, Update: *BOTTOM*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn pwm_correct(self) -> &'a mut W {
        self.variant(WGM4_A::PWM_CORRECT)
    }
    #[doc = "PWM6 / Single-slope, Update: *TOP*, Flag: *TOP*"]
    #[inline(always)]
    pub fn pwm_single_slope(self) -> &'a mut W {
        self.variant(WGM4_A::PWM_SINGLE_SLOPE)
    }
    #[doc = "PWM6 / Dual-slope, Update: *BOTTOM*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn pwm_dual_slope(self) -> &'a mut W {
        self.variant(WGM4_A::PWM_DUAL_SLOPE)
    }
}
#[doc = "Field `FPF4` reader - Fault Protection Interrupt Flag"]
pub type FPF4_R = crate::BitReader<bool>;
#[doc = "Field `FPF4` writer - Fault Protection Interrupt Flag"]
pub type FPF4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4D_SPEC, bool, O>;
#[doc = "Field `FPAC4` reader - Fault Protection Analog Comparator Enable"]
pub type FPAC4_R = crate::BitReader<bool>;
#[doc = "Field `FPAC4` writer - Fault Protection Analog Comparator Enable"]
pub type FPAC4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4D_SPEC, bool, O>;
#[doc = "Field `FPES4` reader - Fault Protection Edge Select"]
pub type FPES4_R = crate::BitReader<bool>;
#[doc = "Field `FPES4` writer - Fault Protection Edge Select"]
pub type FPES4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4D_SPEC, bool, O>;
#[doc = "Field `FPNC4` reader - Fault Protection Noise Canceler"]
pub type FPNC4_R = crate::BitReader<bool>;
#[doc = "Field `FPNC4` writer - Fault Protection Noise Canceler"]
pub type FPNC4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4D_SPEC, bool, O>;
#[doc = "Field `FPEN4` reader - Fault Protection Mode Enable"]
pub type FPEN4_R = crate::BitReader<bool>;
#[doc = "Field `FPEN4` writer - Fault Protection Mode Enable"]
pub type FPEN4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4D_SPEC, bool, O>;
#[doc = "Field `FPIE4` reader - Fault Protection Interrupt Enable"]
pub type FPIE4_R = crate::BitReader<bool>;
#[doc = "Field `FPIE4` writer - Fault Protection Interrupt Enable"]
pub type FPIE4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4D_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode bits"]
    #[inline(always)]
    pub fn wgm4(&self) -> WGM4_R {
        WGM4_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Fault Protection Interrupt Flag"]
    #[inline(always)]
    pub fn fpf4(&self) -> FPF4_R {
        FPF4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Protection Analog Comparator Enable"]
    #[inline(always)]
    pub fn fpac4(&self) -> FPAC4_R {
        FPAC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault Protection Edge Select"]
    #[inline(always)]
    pub fn fpes4(&self) -> FPES4_R {
        FPES4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault Protection Noise Canceler"]
    #[inline(always)]
    pub fn fpnc4(&self) -> FPNC4_R {
        FPNC4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fault Protection Mode Enable"]
    #[inline(always)]
    pub fn fpen4(&self) -> FPEN4_R {
        FPEN4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fault Protection Interrupt Enable"]
    #[inline(always)]
    pub fn fpie4(&self) -> FPIE4_R {
        FPIE4_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn wgm4(&mut self) -> WGM4_W<0> {
        WGM4_W::new(self)
    }
    #[doc = "Bit 2 - Fault Protection Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpf4(&mut self) -> FPF4_W<2> {
        FPF4_W::new(self)
    }
    #[doc = "Bit 3 - Fault Protection Analog Comparator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpac4(&mut self) -> FPAC4_W<3> {
        FPAC4_W::new(self)
    }
    #[doc = "Bit 4 - Fault Protection Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn fpes4(&mut self) -> FPES4_W<4> {
        FPES4_W::new(self)
    }
    #[doc = "Bit 5 - Fault Protection Noise Canceler"]
    #[inline(always)]
    #[must_use]
    pub fn fpnc4(&mut self) -> FPNC4_W<5> {
        FPNC4_W::new(self)
    }
    #[doc = "Bit 6 - Fault Protection Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpen4(&mut self) -> FPEN4_W<6> {
        FPEN4_W::new(self)
    }
    #[doc = "Bit 7 - Fault Protection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpie4(&mut self) -> FPIE4_W<7> {
        FPIE4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter 4 Control Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr4d](index.html) module"]
pub struct TCCR4D_SPEC;
impl crate::RegisterSpec for TCCR4D_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr4d::R](R) reader structure"]
impl crate::Readable for TCCR4D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr4d::W](W) writer structure"]
impl crate::Writable for TCCR4D_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4D to value 0"]
impl crate::Resettable for TCCR4D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
