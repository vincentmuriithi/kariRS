#[doc = "Register `OSCCAL` reader"]
pub struct R(crate::R<OSCCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCCAL` writer"]
pub struct W(crate::W<OSCCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCCAL_SPEC>;
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
impl From<crate::W<OSCCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL` reader - Oscillator Calibration Tuning Value"]
pub type CAL_R = crate::FieldReader<u8, CAL_A>;
#[doc = "Oscillator Calibration Tuning Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAL_A {
    #[doc = "0: Calibration value for lowest oscillator frequency"]
    CALIBRATION_VALUE_FOR_LOWEST_OSCILLATOR_FREQUENCY = 0,
    #[doc = "127: End value of low frequency range calibration"]
    END_VALUE_OF_LOW_FREQUENCY_RANGE_CALIBRATION = 127,
    #[doc = "128: Start value of high frequency range calibration"]
    START_VALUE_OF_HIGH_FREQUENCY_RANGE_CALIBRATION = 128,
    #[doc = "255: Calibration value for highest oscillator frequency"]
    CALIBRATION_VALUE_FOR_HIGHEST_OSCILLATOR_FREQUENCY = 255,
}
impl From<CAL_A> for u8 {
    #[inline(always)]
    fn from(variant: CAL_A) -> Self {
        variant as _
    }
}
impl CAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAL_A> {
        match self.bits {
            0 => Some(CAL_A::CALIBRATION_VALUE_FOR_LOWEST_OSCILLATOR_FREQUENCY),
            127 => Some(CAL_A::END_VALUE_OF_LOW_FREQUENCY_RANGE_CALIBRATION),
            128 => Some(CAL_A::START_VALUE_OF_HIGH_FREQUENCY_RANGE_CALIBRATION),
            255 => Some(CAL_A::CALIBRATION_VALUE_FOR_HIGHEST_OSCILLATOR_FREQUENCY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CALIBRATION_VALUE_FOR_LOWEST_OSCILLATOR_FREQUENCY`"]
    #[inline(always)]
    pub fn is_calibration_value_for_lowest_oscillator_frequency(&self) -> bool {
        *self == CAL_A::CALIBRATION_VALUE_FOR_LOWEST_OSCILLATOR_FREQUENCY
    }
    #[doc = "Checks if the value of the field is `END_VALUE_OF_LOW_FREQUENCY_RANGE_CALIBRATION`"]
    #[inline(always)]
    pub fn is_end_value_of_low_frequency_range_calibration(&self) -> bool {
        *self == CAL_A::END_VALUE_OF_LOW_FREQUENCY_RANGE_CALIBRATION
    }
    #[doc = "Checks if the value of the field is `START_VALUE_OF_HIGH_FREQUENCY_RANGE_CALIBRATION`"]
    #[inline(always)]
    pub fn is_start_value_of_high_frequency_range_calibration(&self) -> bool {
        *self == CAL_A::START_VALUE_OF_HIGH_FREQUENCY_RANGE_CALIBRATION
    }
    #[doc = "Checks if the value of the field is `CALIBRATION_VALUE_FOR_HIGHEST_OSCILLATOR_FREQUENCY`"]
    #[inline(always)]
    pub fn is_calibration_value_for_highest_oscillator_frequency(&self) -> bool {
        *self == CAL_A::CALIBRATION_VALUE_FOR_HIGHEST_OSCILLATOR_FREQUENCY
    }
}
#[doc = "Field `CAL` writer - Oscillator Calibration Tuning Value"]
pub type CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, OSCCAL_SPEC, u8, CAL_A, 8, O>;
impl<'a, const O: u8> CAL_W<'a, O> {
    #[doc = "Calibration value for lowest oscillator frequency"]
    #[inline(always)]
    pub fn calibration_value_for_lowest_oscillator_frequency(self) -> &'a mut W {
        self.variant(CAL_A::CALIBRATION_VALUE_FOR_LOWEST_OSCILLATOR_FREQUENCY)
    }
    #[doc = "End value of low frequency range calibration"]
    #[inline(always)]
    pub fn end_value_of_low_frequency_range_calibration(self) -> &'a mut W {
        self.variant(CAL_A::END_VALUE_OF_LOW_FREQUENCY_RANGE_CALIBRATION)
    }
    #[doc = "Start value of high frequency range calibration"]
    #[inline(always)]
    pub fn start_value_of_high_frequency_range_calibration(self) -> &'a mut W {
        self.variant(CAL_A::START_VALUE_OF_HIGH_FREQUENCY_RANGE_CALIBRATION)
    }
    #[doc = "Calibration value for highest oscillator frequency"]
    #[inline(always)]
    pub fn calibration_value_for_highest_oscillator_frequency(self) -> &'a mut W {
        self.variant(CAL_A::CALIBRATION_VALUE_FOR_HIGHEST_OSCILLATOR_FREQUENCY)
    }
}
#[doc = "Field `OSCCAL` reader - Oscillator Calibration"]
pub type OSCCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSCCAL` writer - Oscillator Calibration"]
pub type OSCCAL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, OSCCAL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Oscillator Calibration Tuning Value"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(self.bits)
    }
    #[doc = "Bits 0:7 - Oscillator Calibration"]
    #[inline(always)]
    pub fn osccal(&self) -> OSCCAL_R {
        OSCCAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Oscillator Calibration Tuning Value"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<0> {
        CAL_W::new(self)
    }
    #[doc = "Bits 0:7 - Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn osccal(&mut self) -> OSCCAL_W<0> {
        OSCCAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Calibration Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccal](index.html) module"]
pub struct OSCCAL_SPEC;
impl crate::RegisterSpec for OSCCAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osccal::R](R) reader structure"]
impl crate::Readable for OSCCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osccal::W](W) writer structure"]
impl crate::Writable for OSCCAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCCAL to value 0"]
impl crate::Resettable for OSCCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
