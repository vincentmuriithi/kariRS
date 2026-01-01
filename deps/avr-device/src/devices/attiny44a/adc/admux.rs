#[doc = "Register `ADMUX` reader"]
pub struct R(crate::R<ADMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADMUX` writer"]
pub struct W(crate::W<ADMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADMUX_SPEC>;
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
impl From<crate::W<ADMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX` reader - Analog Channel and Gain Selection Bits"]
pub type MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUX` writer - Analog Channel and Gain Selection Bits"]
pub type MUX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ADMUX_SPEC, u8, u8, 6, O>;
#[doc = "Field `REFS` reader - Reference Selection Bits"]
pub type REFS_R = crate::FieldReader<u8, REFS_A>;
#[doc = "Reference Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFS_A {
    #[doc = "0: VCC used as analog reference, disconnected from PA0 (AREF)"]
    VCC_USED_AS_ANALOG_REFERENCE_DISCONNECTED_FROM_PA0_AREF = 0,
    #[doc = "1: External voltage reference at PA0 (AREF) pin, internal reference turned off"]
    EXTERNAL_VOLTAGE_REFERENCE_AT_PA0_AREF_PIN_INTERNAL_REFERENCE_TURNED_OFF = 1,
    #[doc = "2: Internal 1.1V voltage reference"]
    INTERNAL_1_1V_VOLTAGE_REFERENCE = 2,
}
impl From<REFS_A> for u8 {
    #[inline(always)]
    fn from(variant: REFS_A) -> Self {
        variant as _
    }
}
impl REFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFS_A {
        match self.bits {
            0 => REFS_A::VCC_USED_AS_ANALOG_REFERENCE_DISCONNECTED_FROM_PA0_AREF,
            1 => REFS_A::EXTERNAL_VOLTAGE_REFERENCE_AT_PA0_AREF_PIN_INTERNAL_REFERENCE_TURNED_OFF,
            2 => REFS_A::INTERNAL_1_1V_VOLTAGE_REFERENCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VCC_USED_AS_ANALOG_REFERENCE_DISCONNECTED_FROM_PA0_AREF`"]
    #[inline(always)]
    pub fn is_vcc_used_as_analog_reference_disconnected_from_pa0_aref(&self) -> bool {
        *self == REFS_A::VCC_USED_AS_ANALOG_REFERENCE_DISCONNECTED_FROM_PA0_AREF
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_VOLTAGE_REFERENCE_AT_PA0_AREF_PIN_INTERNAL_REFERENCE_TURNED_OFF`"]
    #[inline(always)]
    pub fn is_external_voltage_reference_at_pa0_aref_pin_internal_reference_turned_off(
        &self,
    ) -> bool {
        *self == REFS_A::EXTERNAL_VOLTAGE_REFERENCE_AT_PA0_AREF_PIN_INTERNAL_REFERENCE_TURNED_OFF
    }
    #[doc = "Checks if the value of the field is `INTERNAL_1_1V_VOLTAGE_REFERENCE`"]
    #[inline(always)]
    pub fn is_internal_1_1v_voltage_reference(&self) -> bool {
        *self == REFS_A::INTERNAL_1_1V_VOLTAGE_REFERENCE
    }
}
#[doc = "Field `REFS` writer - Reference Selection Bits"]
pub type REFS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADMUX_SPEC, u8, REFS_A, 2, O>;
impl<'a, const O: u8> REFS_W<'a, O> {
    #[doc = "VCC used as analog reference, disconnected from PA0 (AREF)"]
    #[inline(always)]
    pub fn vcc_used_as_analog_reference_disconnected_from_pa0_aref(self) -> &'a mut W {
        self.variant(REFS_A::VCC_USED_AS_ANALOG_REFERENCE_DISCONNECTED_FROM_PA0_AREF)
    }
    #[doc = "External voltage reference at PA0 (AREF) pin, internal reference turned off"]
    #[inline(always)]
    pub fn external_voltage_reference_at_pa0_aref_pin_internal_reference_turned_off(
        self,
    ) -> &'a mut W {
        self.variant(
            REFS_A::EXTERNAL_VOLTAGE_REFERENCE_AT_PA0_AREF_PIN_INTERNAL_REFERENCE_TURNED_OFF,
        )
    }
    #[doc = "Internal 1.1V voltage reference"]
    #[inline(always)]
    pub fn internal_1_1v_voltage_reference(self) -> &'a mut W {
        self.variant(REFS_A::INTERNAL_1_1V_VOLTAGE_REFERENCE)
    }
}
impl R {
    #[doc = "Bits 0:5 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - Reference Selection Bits"]
    #[inline(always)]
    pub fn refs(&self) -> REFS_R {
        REFS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<0> {
        MUX_W::new(self)
    }
    #[doc = "Bits 6:7 - Reference Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn refs(&mut self) -> REFS_W<6> {
        REFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Multiplexer Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [admux](index.html) module"]
pub struct ADMUX_SPEC;
impl crate::RegisterSpec for ADMUX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [admux::R](R) reader structure"]
impl crate::Readable for ADMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [admux::W](W) writer structure"]
impl crate::Writable for ADMUX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADMUX to value 0"]
impl crate::Resettable for ADMUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
