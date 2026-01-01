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
pub type MUX_R = crate::FieldReader<u8, MUX_A>;
#[doc = "Analog Channel and Gain Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_A {
    #[doc = "0: Single-ended Input ADC0"]
    ADC0 = 0,
    #[doc = "1: Single-ended Input ADC1"]
    ADC1 = 1,
    #[doc = "2: Single-ended Input ADC2"]
    ADC2 = 2,
    #[doc = "3: Single-ended Input ADC3"]
    ADC3 = 3,
    #[doc = "4: Single-ended Input ADC4"]
    ADC4 = 4,
    #[doc = "5: Single-ended Input ADC5"]
    ADC5 = 5,
    #[doc = "6: Single-ended Input ADC6"]
    ADC6 = 6,
    #[doc = "7: Single-ended Input ADC7"]
    ADC7 = 7,
    #[doc = "32: 0V (GND)"]
    ADC_GND = 32,
    #[doc = "33: Internal Reference (VBG)"]
    ADC_VBG = 33,
    #[doc = "34: Temperature sensor"]
    TEMPSENS = 34,
}
impl From<MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_A) -> Self {
        variant as _
    }
}
impl MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUX_A> {
        match self.bits {
            0 => Some(MUX_A::ADC0),
            1 => Some(MUX_A::ADC1),
            2 => Some(MUX_A::ADC2),
            3 => Some(MUX_A::ADC3),
            4 => Some(MUX_A::ADC4),
            5 => Some(MUX_A::ADC5),
            6 => Some(MUX_A::ADC6),
            7 => Some(MUX_A::ADC7),
            32 => Some(MUX_A::ADC_GND),
            33 => Some(MUX_A::ADC_VBG),
            34 => Some(MUX_A::TEMPSENS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == MUX_A::ADC0
    }
    #[doc = "Checks if the value of the field is `ADC1`"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == MUX_A::ADC1
    }
    #[doc = "Checks if the value of the field is `ADC2`"]
    #[inline(always)]
    pub fn is_adc2(&self) -> bool {
        *self == MUX_A::ADC2
    }
    #[doc = "Checks if the value of the field is `ADC3`"]
    #[inline(always)]
    pub fn is_adc3(&self) -> bool {
        *self == MUX_A::ADC3
    }
    #[doc = "Checks if the value of the field is `ADC4`"]
    #[inline(always)]
    pub fn is_adc4(&self) -> bool {
        *self == MUX_A::ADC4
    }
    #[doc = "Checks if the value of the field is `ADC5`"]
    #[inline(always)]
    pub fn is_adc5(&self) -> bool {
        *self == MUX_A::ADC5
    }
    #[doc = "Checks if the value of the field is `ADC6`"]
    #[inline(always)]
    pub fn is_adc6(&self) -> bool {
        *self == MUX_A::ADC6
    }
    #[doc = "Checks if the value of the field is `ADC7`"]
    #[inline(always)]
    pub fn is_adc7(&self) -> bool {
        *self == MUX_A::ADC7
    }
    #[doc = "Checks if the value of the field is `ADC_GND`"]
    #[inline(always)]
    pub fn is_adc_gnd(&self) -> bool {
        *self == MUX_A::ADC_GND
    }
    #[doc = "Checks if the value of the field is `ADC_VBG`"]
    #[inline(always)]
    pub fn is_adc_vbg(&self) -> bool {
        *self == MUX_A::ADC_VBG
    }
    #[doc = "Checks if the value of the field is `TEMPSENS`"]
    #[inline(always)]
    pub fn is_tempsens(&self) -> bool {
        *self == MUX_A::TEMPSENS
    }
}
#[doc = "Field `MUX` writer - Analog Channel and Gain Selection Bits"]
pub type MUX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ADMUX_SPEC, u8, MUX_A, 6, O>;
impl<'a, const O: u8> MUX_W<'a, O> {
    #[doc = "Single-ended Input ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(MUX_A::ADC0)
    }
    #[doc = "Single-ended Input ADC1"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut W {
        self.variant(MUX_A::ADC1)
    }
    #[doc = "Single-ended Input ADC2"]
    #[inline(always)]
    pub fn adc2(self) -> &'a mut W {
        self.variant(MUX_A::ADC2)
    }
    #[doc = "Single-ended Input ADC3"]
    #[inline(always)]
    pub fn adc3(self) -> &'a mut W {
        self.variant(MUX_A::ADC3)
    }
    #[doc = "Single-ended Input ADC4"]
    #[inline(always)]
    pub fn adc4(self) -> &'a mut W {
        self.variant(MUX_A::ADC4)
    }
    #[doc = "Single-ended Input ADC5"]
    #[inline(always)]
    pub fn adc5(self) -> &'a mut W {
        self.variant(MUX_A::ADC5)
    }
    #[doc = "Single-ended Input ADC6"]
    #[inline(always)]
    pub fn adc6(self) -> &'a mut W {
        self.variant(MUX_A::ADC6)
    }
    #[doc = "Single-ended Input ADC7"]
    #[inline(always)]
    pub fn adc7(self) -> &'a mut W {
        self.variant(MUX_A::ADC7)
    }
    #[doc = "0V (GND)"]
    #[inline(always)]
    pub fn adc_gnd(self) -> &'a mut W {
        self.variant(MUX_A::ADC_GND)
    }
    #[doc = "Internal Reference (VBG)"]
    #[inline(always)]
    pub fn adc_vbg(self) -> &'a mut W {
        self.variant(MUX_A::ADC_VBG)
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn tempsens(self) -> &'a mut W {
        self.variant(MUX_A::TEMPSENS)
    }
}
#[doc = "Field `REFS` reader - Reference Selection Bits"]
pub type REFS_R = crate::FieldReader<u8, REFS_A>;
#[doc = "Reference Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFS_A {
    #[doc = "0: VCC used as analog reference, disconnected from PA0 (AREF)"]
    VCC = 0,
    #[doc = "1: External voltage reference at PA0 (AREF) pin, internal reference turned off"]
    AREF = 1,
    #[doc = "2: Internal 1.1V voltage reference"]
    INTERNAL = 2,
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
    pub fn variant(&self) -> Option<REFS_A> {
        match self.bits {
            0 => Some(REFS_A::VCC),
            1 => Some(REFS_A::AREF),
            2 => Some(REFS_A::INTERNAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VCC`"]
    #[inline(always)]
    pub fn is_vcc(&self) -> bool {
        *self == REFS_A::VCC
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == REFS_A::AREF
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == REFS_A::INTERNAL
    }
}
#[doc = "Field `REFS` writer - Reference Selection Bits"]
pub type REFS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADMUX_SPEC, u8, REFS_A, 2, O>;
impl<'a, const O: u8> REFS_W<'a, O> {
    #[doc = "VCC used as analog reference, disconnected from PA0 (AREF)"]
    #[inline(always)]
    pub fn vcc(self) -> &'a mut W {
        self.variant(REFS_A::VCC)
    }
    #[doc = "External voltage reference at PA0 (AREF) pin, internal reference turned off"]
    #[inline(always)]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFS_A::AREF)
    }
    #[doc = "Internal 1.1V voltage reference"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(REFS_A::INTERNAL)
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
