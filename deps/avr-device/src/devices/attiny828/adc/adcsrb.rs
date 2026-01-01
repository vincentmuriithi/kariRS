#[doc = "Register `ADCSRB` reader"]
pub struct R(crate::R<ADCSRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCSRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCSRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCSRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCSRB` writer"]
pub struct W(crate::W<ADCSRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCSRB_SPEC>;
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
impl From<crate::W<ADCSRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCSRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADTS` reader - ADC Auto Trigger Sources"]
pub type ADTS_R = crate::FieldReader<u8, ADTS_A>;
#[doc = "ADC Auto Trigger Sources\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADTS_A {
    #[doc = "0: Free Running Mode"]
    FREE_RUNNING = 0,
    #[doc = "1: Analog Comparator"]
    ANALOG_COMPARATOR = 1,
    #[doc = "2: External Interrupt Request 0"]
    EXTERNAL_INTERRUPT = 2,
    #[doc = "3: Timer/Counter0 Compare Match A"]
    TIMER0_COMPARE_MATCH_A = 3,
    #[doc = "4: Timer/Counter0 Overflow"]
    TIMER0_OVERFLOW = 4,
    #[doc = "5: Timer/Counter1 Compare Match B"]
    TIMER1_COMPARE_MATCH_B = 5,
    #[doc = "6: Timer/Counter1 Overflow"]
    TIMER1_OVERFLOW = 6,
    #[doc = "7: Timer/Counter1 Capture Event"]
    TIMER1_CAPTURE_EVENT = 7,
}
impl From<ADTS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADTS_A) -> Self {
        variant as _
    }
}
impl ADTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTS_A {
        match self.bits {
            0 => ADTS_A::FREE_RUNNING,
            1 => ADTS_A::ANALOG_COMPARATOR,
            2 => ADTS_A::EXTERNAL_INTERRUPT,
            3 => ADTS_A::TIMER0_COMPARE_MATCH_A,
            4 => ADTS_A::TIMER0_OVERFLOW,
            5 => ADTS_A::TIMER1_COMPARE_MATCH_B,
            6 => ADTS_A::TIMER1_OVERFLOW,
            7 => ADTS_A::TIMER1_CAPTURE_EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREE_RUNNING`"]
    #[inline(always)]
    pub fn is_free_running(&self) -> bool {
        *self == ADTS_A::FREE_RUNNING
    }
    #[doc = "Checks if the value of the field is `ANALOG_COMPARATOR`"]
    #[inline(always)]
    pub fn is_analog_comparator(&self) -> bool {
        *self == ADTS_A::ANALOG_COMPARATOR
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_INTERRUPT`"]
    #[inline(always)]
    pub fn is_external_interrupt(&self) -> bool {
        *self == ADTS_A::EXTERNAL_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `TIMER0_COMPARE_MATCH_A`"]
    #[inline(always)]
    pub fn is_timer0_compare_match_a(&self) -> bool {
        *self == ADTS_A::TIMER0_COMPARE_MATCH_A
    }
    #[doc = "Checks if the value of the field is `TIMER0_OVERFLOW`"]
    #[inline(always)]
    pub fn is_timer0_overflow(&self) -> bool {
        *self == ADTS_A::TIMER0_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `TIMER1_COMPARE_MATCH_B`"]
    #[inline(always)]
    pub fn is_timer1_compare_match_b(&self) -> bool {
        *self == ADTS_A::TIMER1_COMPARE_MATCH_B
    }
    #[doc = "Checks if the value of the field is `TIMER1_OVERFLOW`"]
    #[inline(always)]
    pub fn is_timer1_overflow(&self) -> bool {
        *self == ADTS_A::TIMER1_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `TIMER1_CAPTURE_EVENT`"]
    #[inline(always)]
    pub fn is_timer1_capture_event(&self) -> bool {
        *self == ADTS_A::TIMER1_CAPTURE_EVENT
    }
}
#[doc = "Field `ADTS` writer - ADC Auto Trigger Sources"]
pub type ADTS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ADCSRB_SPEC, u8, ADTS_A, 3, O>;
impl<'a, const O: u8> ADTS_W<'a, O> {
    #[doc = "Free Running Mode"]
    #[inline(always)]
    pub fn free_running(self) -> &'a mut W {
        self.variant(ADTS_A::FREE_RUNNING)
    }
    #[doc = "Analog Comparator"]
    #[inline(always)]
    pub fn analog_comparator(self) -> &'a mut W {
        self.variant(ADTS_A::ANALOG_COMPARATOR)
    }
    #[doc = "External Interrupt Request 0"]
    #[inline(always)]
    pub fn external_interrupt(self) -> &'a mut W {
        self.variant(ADTS_A::EXTERNAL_INTERRUPT)
    }
    #[doc = "Timer/Counter0 Compare Match A"]
    #[inline(always)]
    pub fn timer0_compare_match_a(self) -> &'a mut W {
        self.variant(ADTS_A::TIMER0_COMPARE_MATCH_A)
    }
    #[doc = "Timer/Counter0 Overflow"]
    #[inline(always)]
    pub fn timer0_overflow(self) -> &'a mut W {
        self.variant(ADTS_A::TIMER0_OVERFLOW)
    }
    #[doc = "Timer/Counter1 Compare Match B"]
    #[inline(always)]
    pub fn timer1_compare_match_b(self) -> &'a mut W {
        self.variant(ADTS_A::TIMER1_COMPARE_MATCH_B)
    }
    #[doc = "Timer/Counter1 Overflow"]
    #[inline(always)]
    pub fn timer1_overflow(self) -> &'a mut W {
        self.variant(ADTS_A::TIMER1_OVERFLOW)
    }
    #[doc = "Timer/Counter1 Capture Event"]
    #[inline(always)]
    pub fn timer1_capture_event(self) -> &'a mut W {
        self.variant(ADTS_A::TIMER1_CAPTURE_EVENT)
    }
}
#[doc = "Field `ADLAR` reader - No Description."]
pub type ADLAR_R = crate::BitReader<bool>;
#[doc = "Field `ADLAR` writer - No Description."]
pub type ADLAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCSRB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - ADC Auto Trigger Sources"]
    #[inline(always)]
    pub fn adts(&self) -> ADTS_R {
        ADTS_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn adlar(&self) -> ADLAR_R {
        ADLAR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC Auto Trigger Sources"]
    #[inline(always)]
    #[must_use]
    pub fn adts(&mut self) -> ADTS_W<0> {
        ADTS_W::new(self)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adlar(&mut self) -> ADLAR_W<3> {
        ADLAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control and Status Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcsrb](index.html) module"]
pub struct ADCSRB_SPEC;
impl crate::RegisterSpec for ADCSRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcsrb::R](R) reader structure"]
impl crate::Readable for ADCSRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcsrb::W](W) writer structure"]
impl crate::Writable for ADCSRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCSRB to value 0"]
impl crate::Resettable for ADCSRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
