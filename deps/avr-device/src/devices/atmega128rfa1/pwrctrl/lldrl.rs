#[doc = "Register `LLDRL` reader"]
pub struct R(crate::R<LLDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LLDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LLDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LLDRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LLDRL` writer"]
pub struct W(crate::W<LLDRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LLDRL_SPEC>;
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
impl From<crate::W<LLDRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LLDRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLDRL` reader - Low-Byte Data Register Bits"]
pub type LLDRL_R = crate::FieldReader<u8, LLDRL_A>;
#[doc = "Low-Byte Data Register Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LLDRL_A {
    #[doc = "0: Calibration limit for fast process corner/high output voltage"]
    CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE = 0,
    #[doc = "8: Calibration limit for slow process corner/low output voltage"]
    CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE = 8,
}
impl From<LLDRL_A> for u8 {
    #[inline(always)]
    fn from(variant: LLDRL_A) -> Self {
        variant as _
    }
}
impl LLDRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LLDRL_A> {
        match self.bits {
            0 => Some(LLDRL_A::CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE),
            8 => Some(LLDRL_A::CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE`"]
    #[inline(always)]
    pub fn is_calibration_limit_for_fast_process_corner_high_output_voltage(&self) -> bool {
        *self == LLDRL_A::CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE
    }
    #[doc = "Checks if the value of the field is `CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE`"]
    #[inline(always)]
    pub fn is_calibration_limit_for_slow_process_corner_low_output_voltage(&self) -> bool {
        *self == LLDRL_A::CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE
    }
}
#[doc = "Field `LLDRL` writer - Low-Byte Data Register Bits"]
pub type LLDRL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LLDRL_SPEC, u8, LLDRL_A, 4, O>;
impl<'a, const O: u8> LLDRL_W<'a, O> {
    #[doc = "Calibration limit for fast process corner/high output voltage"]
    #[inline(always)]
    pub fn calibration_limit_for_fast_process_corner_high_output_voltage(self) -> &'a mut W {
        self.variant(LLDRL_A::CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE)
    }
    #[doc = "Calibration limit for slow process corner/low output voltage"]
    #[inline(always)]
    pub fn calibration_limit_for_slow_process_corner_low_output_voltage(self) -> &'a mut W {
        self.variant(LLDRL_A::CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LLDRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Low-Byte Data Register Bits"]
    #[inline(always)]
    pub fn lldrl(&self) -> LLDRL_R {
        LLDRL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Low-Byte Data Register Bits"]
    #[inline(always)]
    #[must_use]
    pub fn lldrl(&mut self) -> LLDRL_W<0> {
        LLDRL_W::new(self)
    }
    #[doc = "Bits 4:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<4> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Leakage Voltage Regulator Data Register (Low-Byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lldrl](index.html) module"]
pub struct LLDRL_SPEC;
impl crate::RegisterSpec for LLDRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lldrl::R](R) reader structure"]
impl crate::Readable for LLDRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lldrl::W](W) writer structure"]
impl crate::Writable for LLDRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LLDRL to value 0"]
impl crate::Resettable for LLDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
