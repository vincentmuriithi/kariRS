#[doc = "Register `TCCR0A` reader"]
pub struct R(crate::R<TCCR0A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR0A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR0A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR0A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR0A` writer"]
pub struct W(crate::W<TCCR0A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR0A_SPEC>;
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
impl From<crate::W<TCCR0A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR0A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WGM0` reader - Waveform Generation Mode"]
pub type WGM0_R = crate::FieldReader<u8, WGM0_A>;
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM0_A {
    #[doc = "0: Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*"]
    NORMAL_TOP = 0,
    #[doc = "1: Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*"]
    PWM_PHASE = 1,
    #[doc = "2: CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*"]
    CTC = 2,
    #[doc = "3: Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*"]
    PWM_FAST = 3,
}
impl From<WGM0_A> for u8 {
    #[inline(always)]
    fn from(variant: WGM0_A) -> Self {
        variant as _
    }
}
impl WGM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WGM0_A {
        match self.bits {
            0 => WGM0_A::NORMAL_TOP,
            1 => WGM0_A::PWM_PHASE,
            2 => WGM0_A::CTC,
            3 => WGM0_A::PWM_FAST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_TOP`"]
    #[inline(always)]
    pub fn is_normal_top(&self) -> bool {
        *self == WGM0_A::NORMAL_TOP
    }
    #[doc = "Checks if the value of the field is `PWM_PHASE`"]
    #[inline(always)]
    pub fn is_pwm_phase(&self) -> bool {
        *self == WGM0_A::PWM_PHASE
    }
    #[doc = "Checks if the value of the field is `CTC`"]
    #[inline(always)]
    pub fn is_ctc(&self) -> bool {
        *self == WGM0_A::CTC
    }
    #[doc = "Checks if the value of the field is `PWM_FAST`"]
    #[inline(always)]
    pub fn is_pwm_fast(&self) -> bool {
        *self == WGM0_A::PWM_FAST
    }
}
#[doc = "Field `WGM0` writer - Waveform Generation Mode"]
pub type WGM0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR0A_SPEC, u8, WGM0_A, 2, O>;
impl<'a, const O: u8> WGM0_W<'a, O> {
    #[doc = "Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*"]
    #[inline(always)]
    pub fn normal_top(self) -> &'a mut W {
        self.variant(WGM0_A::NORMAL_TOP)
    }
    #[doc = "Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn pwm_phase(self) -> &'a mut W {
        self.variant(WGM0_A::PWM_PHASE)
    }
    #[doc = "CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*"]
    #[inline(always)]
    pub fn ctc(self) -> &'a mut W {
        self.variant(WGM0_A::CTC)
    }
    #[doc = "Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*"]
    #[inline(always)]
    pub fn pwm_fast(self) -> &'a mut W {
        self.variant(WGM0_A::PWM_FAST)
    }
}
#[doc = "Field `COM0B` reader - Compare Output B Mode"]
pub type COM0B_R = crate::FieldReader<u8, COM0B_A>;
#[doc = "Compare Output B Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM0B_A {
    #[doc = "0: Normal port operation, OCix disconnected"]
    DISCONNECTED = 0,
    #[doc = "1: Toggle OCix on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE = 1,
    #[doc = "2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)"]
    MATCH_CLEAR = 2,
    #[doc = "3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)"]
    MATCH_SET = 3,
}
impl From<COM0B_A> for u8 {
    #[inline(always)]
    fn from(variant: COM0B_A) -> Self {
        variant as _
    }
}
impl COM0B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COM0B_A {
        match self.bits {
            0 => COM0B_A::DISCONNECTED,
            1 => COM0B_A::MATCH_TOGGLE,
            2 => COM0B_A::MATCH_CLEAR,
            3 => COM0B_A::MATCH_SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == COM0B_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `MATCH_TOGGLE`"]
    #[inline(always)]
    pub fn is_match_toggle(&self) -> bool {
        *self == COM0B_A::MATCH_TOGGLE
    }
    #[doc = "Checks if the value of the field is `MATCH_CLEAR`"]
    #[inline(always)]
    pub fn is_match_clear(&self) -> bool {
        *self == COM0B_A::MATCH_CLEAR
    }
    #[doc = "Checks if the value of the field is `MATCH_SET`"]
    #[inline(always)]
    pub fn is_match_set(&self) -> bool {
        *self == COM0B_A::MATCH_SET
    }
}
#[doc = "Field `COM0B` writer - Compare Output B Mode"]
pub type COM0B_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR0A_SPEC, u8, COM0B_A, 2, O>;
impl<'a, const O: u8> COM0B_W<'a, O> {
    #[doc = "Normal port operation, OCix disconnected"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(COM0B_A::DISCONNECTED)
    }
    #[doc = "Toggle OCix on Compare Match (Might depend on WGM)"]
    #[inline(always)]
    pub fn match_toggle(self) -> &'a mut W {
        self.variant(COM0B_A::MATCH_TOGGLE)
    }
    #[doc = "Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)"]
    #[inline(always)]
    pub fn match_clear(self) -> &'a mut W {
        self.variant(COM0B_A::MATCH_CLEAR)
    }
    #[doc = "Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)"]
    #[inline(always)]
    pub fn match_set(self) -> &'a mut W {
        self.variant(COM0B_A::MATCH_SET)
    }
}
#[doc = "Field `COM0A` reader - Compare Output A Mode"]
pub use COM0B_R as COM0A_R;
#[doc = "Field `COM0A` writer - Compare Output A Mode"]
pub use COM0B_W as COM0A_W;
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm0(&self) -> WGM0_R {
        WGM0_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - Compare Output B Mode"]
    #[inline(always)]
    pub fn com0b(&self) -> COM0B_R {
        COM0B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output A Mode"]
    #[inline(always)]
    pub fn com0a(&self) -> COM0A_R {
        COM0A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm0(&mut self) -> WGM0_W<0> {
        WGM0_W::new(self)
    }
    #[doc = "Bits 4:5 - Compare Output B Mode"]
    #[inline(always)]
    #[must_use]
    pub fn com0b(&mut self) -> COM0B_W<4> {
        COM0B_W::new(self)
    }
    #[doc = "Bits 6:7 - Compare Output A Mode"]
    #[inline(always)]
    #[must_use]
    pub fn com0a(&mut self) -> COM0A_W<6> {
        COM0A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr0a](index.html) module"]
pub struct TCCR0A_SPEC;
impl crate::RegisterSpec for TCCR0A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr0a::R](R) reader structure"]
impl crate::Readable for TCCR0A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr0a::W](W) writer structure"]
impl crate::Writable for TCCR0A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR0A to value 0"]
impl crate::Resettable for TCCR0A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
