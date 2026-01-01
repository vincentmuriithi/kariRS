#[doc = "Register `LLDRH` reader"]
pub struct R(crate::R<LLDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LLDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LLDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LLDRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LLDRH` writer"]
pub struct W(crate::W<LLDRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LLDRH_SPEC>;
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
impl From<crate::W<LLDRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LLDRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLDRH` reader - High-Byte Data Register Bits"]
pub type LLDRH_R = crate::FieldReader<u8, LLDRH_A>;
#[doc = "High-Byte Data Register Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LLDRH_A {
    #[doc = "0: Calibration limit for fast process corner/high output voltage"]
    CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE = 0,
    #[doc = "16: Calibration limit for slow process corner/low output voltage"]
    CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE = 16,
}
impl From<LLDRH_A> for u8 {
    #[inline(always)]
    fn from(variant: LLDRH_A) -> Self {
        variant as _
    }
}
impl LLDRH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LLDRH_A> {
        match self.bits {
            0 => Some(LLDRH_A::CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE),
            16 => Some(LLDRH_A::CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE`"]
    #[inline(always)]
    pub fn is_calibration_limit_for_fast_process_corner_high_output_voltage(&self) -> bool {
        *self == LLDRH_A::CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE
    }
    #[doc = "Checks if the value of the field is `CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE`"]
    #[inline(always)]
    pub fn is_calibration_limit_for_slow_process_corner_low_output_voltage(&self) -> bool {
        *self == LLDRH_A::CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE
    }
}
#[doc = "Field `LLDRH` writer - High-Byte Data Register Bits"]
pub type LLDRH_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LLDRH_SPEC, u8, LLDRH_A, 5, O>;
impl<'a, const O: u8> LLDRH_W<'a, O> {
    #[doc = "Calibration limit for fast process corner/high output voltage"]
    #[inline(always)]
    pub fn calibration_limit_for_fast_process_corner_high_output_voltage(self) -> &'a mut W {
        self.variant(LLDRH_A::CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE)
    }
    #[doc = "Calibration limit for slow process corner/low output voltage"]
    #[inline(always)]
    pub fn calibration_limit_for_slow_process_corner_low_output_voltage(self) -> &'a mut W {
        self.variant(LLDRH_A::CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LLDRH_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:4 - High-Byte Data Register Bits"]
    #[inline(always)]
    pub fn lldrh(&self) -> LLDRH_R {
        LLDRH_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:4 - High-Byte Data Register Bits"]
    #[inline(always)]
    #[must_use]
    pub fn lldrh(&mut self) -> LLDRH_W<0> {
        LLDRH_W::new(self)
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<5> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Leakage Voltage Regulator Data Register (High-Byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lldrh](index.html) module"]
pub struct LLDRH_SPEC;
impl crate::RegisterSpec for LLDRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lldrh::R](R) reader structure"]
impl crate::Readable for LLDRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lldrh::W](W) writer structure"]
impl crate::Writable for LLDRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LLDRH to value 0"]
impl crate::Resettable for LLDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
