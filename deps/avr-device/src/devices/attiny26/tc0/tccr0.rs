#[doc = "Register `TCCR0` reader"]
pub struct R(crate::R<TCCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR0` writer"]
pub struct W(crate::W<TCCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR0_SPEC>;
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
impl From<crate::W<TCCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS0` reader - Clock Select0 bits"]
pub type CS0_R = crate::FieldReader<u8, CS0_A>;
#[doc = "Clock Select0 bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS0_A {
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
impl From<CS0_A> for u8 {
    #[inline(always)]
    fn from(variant: CS0_A) -> Self {
        variant as _
    }
}
impl CS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS0_A {
        match self.bits {
            0 => CS0_A::NO_CLOCK_SOURCE_STOPPED,
            1 => CS0_A::RUNNING_NO_PRESCALING,
            2 => CS0_A::RUNNING_CLK_8,
            3 => CS0_A::RUNNING_CLK_64,
            4 => CS0_A::RUNNING_CLK_256,
            5 => CS0_A::RUNNING_CLK_1024,
            6 => CS0_A::RUNNING_EXTCLK_TN_FALLING_EDGE,
            7 => CS0_A::RUNNING_EXTCLK_TN_RISING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK_SOURCE_STOPPED`"]
    #[inline(always)]
    pub fn is_no_clock_source_stopped(&self) -> bool {
        *self == CS0_A::NO_CLOCK_SOURCE_STOPPED
    }
    #[doc = "Checks if the value of the field is `RUNNING_NO_PRESCALING`"]
    #[inline(always)]
    pub fn is_running_no_prescaling(&self) -> bool {
        *self == CS0_A::RUNNING_NO_PRESCALING
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_8`"]
    #[inline(always)]
    pub fn is_running_clk_8(&self) -> bool {
        *self == CS0_A::RUNNING_CLK_8
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_64`"]
    #[inline(always)]
    pub fn is_running_clk_64(&self) -> bool {
        *self == CS0_A::RUNNING_CLK_64
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_256`"]
    #[inline(always)]
    pub fn is_running_clk_256(&self) -> bool {
        *self == CS0_A::RUNNING_CLK_256
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_1024`"]
    #[inline(always)]
    pub fn is_running_clk_1024(&self) -> bool {
        *self == CS0_A::RUNNING_CLK_1024
    }
    #[doc = "Checks if the value of the field is `RUNNING_EXTCLK_TN_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_running_extclk_tn_falling_edge(&self) -> bool {
        *self == CS0_A::RUNNING_EXTCLK_TN_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RUNNING_EXTCLK_TN_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_running_extclk_tn_rising_edge(&self) -> bool {
        *self == CS0_A::RUNNING_EXTCLK_TN_RISING_EDGE
    }
}
#[doc = "Field `CS0` writer - Clock Select0 bits"]
pub type CS0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR0_SPEC, u8, CS0_A, 3, O>;
impl<'a, const O: u8> CS0_W<'a, O> {
    #[doc = "No Clock Source (Stopped)"]
    #[inline(always)]
    pub fn no_clock_source_stopped(self) -> &'a mut W {
        self.variant(CS0_A::NO_CLOCK_SOURCE_STOPPED)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn running_no_prescaling(self) -> &'a mut W {
        self.variant(CS0_A::RUNNING_NO_PRESCALING)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn running_clk_8(self) -> &'a mut W {
        self.variant(CS0_A::RUNNING_CLK_8)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn running_clk_64(self) -> &'a mut W {
        self.variant(CS0_A::RUNNING_CLK_64)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn running_clk_256(self) -> &'a mut W {
        self.variant(CS0_A::RUNNING_CLK_256)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn running_clk_1024(self) -> &'a mut W {
        self.variant(CS0_A::RUNNING_CLK_1024)
    }
    #[doc = "Running, ExtClk Tn Falling Edge"]
    #[inline(always)]
    pub fn running_extclk_tn_falling_edge(self) -> &'a mut W {
        self.variant(CS0_A::RUNNING_EXTCLK_TN_FALLING_EDGE)
    }
    #[doc = "Running, ExtClk Tn Rising Edge"]
    #[inline(always)]
    pub fn running_extclk_tn_rising_edge(self) -> &'a mut W {
        self.variant(CS0_A::RUNNING_EXTCLK_TN_RISING_EDGE)
    }
}
#[doc = "Field `PSR0` reader - Prescaler Reset Timer/Counter0"]
pub type PSR0_R = crate::BitReader<bool>;
#[doc = "Field `PSR0` writer - Prescaler Reset Timer/Counter0"]
pub type PSR0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Clock Select0 bits"]
    #[inline(always)]
    pub fn cs0(&self) -> CS0_R {
        CS0_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Prescaler Reset Timer/Counter0"]
    #[inline(always)]
    pub fn psr0(&self) -> PSR0_R {
        PSR0_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select0 bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs0(&mut self) -> CS0_W<0> {
        CS0_W::new(self)
    }
    #[doc = "Bit 3 - Prescaler Reset Timer/Counter0"]
    #[inline(always)]
    #[must_use]
    pub fn psr0(&mut self) -> PSR0_W<3> {
        PSR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr0](index.html) module"]
pub struct TCCR0_SPEC;
impl crate::RegisterSpec for TCCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr0::R](R) reader structure"]
impl crate::Readable for TCCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr0::W](W) writer structure"]
impl crate::Writable for TCCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR0 to value 0"]
impl crate::Resettable for TCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
