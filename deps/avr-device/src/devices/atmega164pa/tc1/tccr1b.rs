#[doc = "Register `TCCR1B` reader"]
pub struct R(crate::R<TCCR1B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR1B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR1B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR1B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR1B` writer"]
pub struct W(crate::W<TCCR1B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR1B_SPEC>;
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
impl From<crate::W<TCCR1B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR1B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS1` reader - Clock Select1 bits"]
pub type CS1_R = crate::FieldReader<u8, CS1_A>;
#[doc = "Clock Select1 bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS1_A {
    #[doc = "0: No Clock Source (Stopped)"]
    NO_CLOCK_SOURCE_STOPPED = 0,
    #[doc = "1: Running, No Prescaling"]
    RUNNING_NO_PRESCALING = 1,
    #[doc = "2: Running, CLK/8"]
    RUNNING_CLK_8 = 2,
    #[doc = "3: Running, CLK/64"]
    RUNNING_CLK_64 = 3,
    #[doc = "4: Running, CLK/256"]
    RUNNING_CLK_256 = 4,
    #[doc = "5: Running, CLK/1024"]
    RUNNING_CLK_1024 = 5,
    #[doc = "6: Running, ExtClk Tn Falling Edge"]
    RUNNING_EXTCLK_TN_FALLING_EDGE = 6,
    #[doc = "7: Running, ExtClk Tn Rising Edge"]
    RUNNING_EXTCLK_TN_RISING_EDGE = 7,
}
impl From<CS1_A> for u8 {
    #[inline(always)]
    fn from(variant: CS1_A) -> Self {
        variant as _
    }
}
impl CS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS1_A {
        match self.bits {
            0 => CS1_A::NO_CLOCK_SOURCE_STOPPED,
            1 => CS1_A::RUNNING_NO_PRESCALING,
            2 => CS1_A::RUNNING_CLK_8,
            3 => CS1_A::RUNNING_CLK_64,
            4 => CS1_A::RUNNING_CLK_256,
            5 => CS1_A::RUNNING_CLK_1024,
            6 => CS1_A::RUNNING_EXTCLK_TN_FALLING_EDGE,
            7 => CS1_A::RUNNING_EXTCLK_TN_RISING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK_SOURCE_STOPPED`"]
    #[inline(always)]
    pub fn is_no_clock_source_stopped(&self) -> bool {
        *self == CS1_A::NO_CLOCK_SOURCE_STOPPED
    }
    #[doc = "Checks if the value of the field is `RUNNING_NO_PRESCALING`"]
    #[inline(always)]
    pub fn is_running_no_prescaling(&self) -> bool {
        *self == CS1_A::RUNNING_NO_PRESCALING
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_8`"]
    #[inline(always)]
    pub fn is_running_clk_8(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_8
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_64`"]
    #[inline(always)]
    pub fn is_running_clk_64(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_64
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_256`"]
    #[inline(always)]
    pub fn is_running_clk_256(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_256
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_1024`"]
    #[inline(always)]
    pub fn is_running_clk_1024(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_1024
    }
    #[doc = "Checks if the value of the field is `RUNNING_EXTCLK_TN_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_running_extclk_tn_falling_edge(&self) -> bool {
        *self == CS1_A::RUNNING_EXTCLK_TN_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RUNNING_EXTCLK_TN_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_running_extclk_tn_rising_edge(&self) -> bool {
        *self == CS1_A::RUNNING_EXTCLK_TN_RISING_EDGE
    }
}
#[doc = "Field `CS1` writer - Clock Select1 bits"]
pub type CS1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR1B_SPEC, u8, CS1_A, 3, O>;
impl<'a, const O: u8> CS1_W<'a, O> {
    #[doc = "No Clock Source (Stopped)"]
    #[inline(always)]
    pub fn no_clock_source_stopped(self) -> &'a mut W {
        self.variant(CS1_A::NO_CLOCK_SOURCE_STOPPED)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn running_no_prescaling(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_NO_PRESCALING)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn running_clk_8(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_8)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn running_clk_64(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_64)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn running_clk_256(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_256)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn running_clk_1024(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_1024)
    }
    #[doc = "Running, ExtClk Tn Falling Edge"]
    #[inline(always)]
    pub fn running_extclk_tn_falling_edge(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_EXTCLK_TN_FALLING_EDGE)
    }
    #[doc = "Running, ExtClk Tn Rising Edge"]
    #[inline(always)]
    pub fn running_extclk_tn_rising_edge(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_EXTCLK_TN_RISING_EDGE)
    }
}
#[doc = "Field `WGM1` reader - Waveform Generation Mode Bits"]
pub type WGM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WGM1` writer - Waveform Generation Mode Bits"]
pub type WGM1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR1B_SPEC, u8, u8, 2, O>;
#[doc = "Field `ICES1` reader - Input Capture 1 Edge Select"]
pub type ICES1_R = crate::BitReader<bool>;
#[doc = "Field `ICES1` writer - Input Capture 1 Edge Select"]
pub type ICES1_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1B_SPEC, bool, O>;
#[doc = "Field `ICNC1` reader - Input Capture 1 Noise Canceler"]
pub type ICNC1_R = crate::BitReader<bool>;
#[doc = "Field `ICNC1` writer - Input Capture 1 Noise Canceler"]
pub type ICNC1_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1B_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Clock Select1 bits"]
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode Bits"]
    #[inline(always)]
    pub fn wgm1(&self) -> WGM1_R {
        WGM1_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 6 - Input Capture 1 Edge Select"]
    #[inline(always)]
    pub fn ices1(&self) -> ICES1_R {
        ICES1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Capture 1 Noise Canceler"]
    #[inline(always)]
    pub fn icnc1(&self) -> ICNC1_R {
        ICNC1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select1 bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs1(&mut self) -> CS1_W<0> {
        CS1_W::new(self)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode Bits"]
    #[inline(always)]
    #[must_use]
    pub fn wgm1(&mut self) -> WGM1_W<3> {
        WGM1_W::new(self)
    }
    #[doc = "Bit 6 - Input Capture 1 Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ices1(&mut self) -> ICES1_W<6> {
        ICES1_W::new(self)
    }
    #[doc = "Bit 7 - Input Capture 1 Noise Canceler"]
    #[inline(always)]
    #[must_use]
    pub fn icnc1(&mut self) -> ICNC1_W<7> {
        ICNC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter1 Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr1b](index.html) module"]
pub struct TCCR1B_SPEC;
impl crate::RegisterSpec for TCCR1B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr1b::R](R) reader structure"]
impl crate::Readable for TCCR1B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr1b::W](W) writer structure"]
impl crate::Writable for TCCR1B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1B to value 0"]
impl crate::Resettable for TCCR1B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
