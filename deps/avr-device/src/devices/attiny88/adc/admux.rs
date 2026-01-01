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
#[doc = "Field `MUX` reader - Analog Channel Selection Bits"]
pub type MUX_R = crate::FieldReader<u8, MUX_A>;
#[doc = "Analog Channel Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_A {
    #[doc = "0: ADC Single Ended Input pin 0"]
    ADC0 = 0,
    #[doc = "1: ADC Single Ended Input pin 1"]
    ADC1 = 1,
    #[doc = "2: ADC Single Ended Input pin 2"]
    ADC2 = 2,
    #[doc = "3: ADC Single Ended Input pin 3"]
    ADC3 = 3,
    #[doc = "4: ADC Single Ended Input pin 4"]
    ADC4 = 4,
    #[doc = "5: ADC Single Ended Input pin 5"]
    ADC5 = 5,
    #[doc = "6: ADC Single Ended Input pin 6"]
    ADC6 = 6,
    #[doc = "7: ADC Single Ended Input pin 7"]
    ADC7 = 7,
    #[doc = "8: Temperature sensor"]
    TEMPSENS = 8,
    #[doc = "14: Internal Reference (VBG)"]
    ADC_VBG = 14,
    #[doc = "15: 0V (GND)"]
    ADC_GND = 15,
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
            8 => Some(MUX_A::TEMPSENS),
            14 => Some(MUX_A::ADC_VBG),
            15 => Some(MUX_A::ADC_GND),
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
    #[doc = "Checks if the value of the field is `TEMPSENS`"]
    #[inline(always)]
    pub fn is_tempsens(&self) -> bool {
        *self == MUX_A::TEMPSENS
    }
    #[doc = "Checks if the value of the field is `ADC_VBG`"]
    #[inline(always)]
    pub fn is_adc_vbg(&self) -> bool {
        *self == MUX_A::ADC_VBG
    }
    #[doc = "Checks if the value of the field is `ADC_GND`"]
    #[inline(always)]
    pub fn is_adc_gnd(&self) -> bool {
        *self == MUX_A::ADC_GND
    }
}
#[doc = "Field `MUX` writer - Analog Channel Selection Bits"]
pub type MUX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ADMUX_SPEC, u8, MUX_A, 4, O>;
impl<'a, const O: u8> MUX_W<'a, O> {
    #[doc = "ADC Single Ended Input pin 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(MUX_A::ADC0)
    }
    #[doc = "ADC Single Ended Input pin 1"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut W {
        self.variant(MUX_A::ADC1)
    }
    #[doc = "ADC Single Ended Input pin 2"]
    #[inline(always)]
    pub fn adc2(self) -> &'a mut W {
        self.variant(MUX_A::ADC2)
    }
    #[doc = "ADC Single Ended Input pin 3"]
    #[inline(always)]
    pub fn adc3(self) -> &'a mut W {
        self.variant(MUX_A::ADC3)
    }
    #[doc = "ADC Single Ended Input pin 4"]
    #[inline(always)]
    pub fn adc4(self) -> &'a mut W {
        self.variant(MUX_A::ADC4)
    }
    #[doc = "ADC Single Ended Input pin 5"]
    #[inline(always)]
    pub fn adc5(self) -> &'a mut W {
        self.variant(MUX_A::ADC5)
    }
    #[doc = "ADC Single Ended Input pin 6"]
    #[inline(always)]
    pub fn adc6(self) -> &'a mut W {
        self.variant(MUX_A::ADC6)
    }
    #[doc = "ADC Single Ended Input pin 7"]
    #[inline(always)]
    pub fn adc7(self) -> &'a mut W {
        self.variant(MUX_A::ADC7)
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn tempsens(self) -> &'a mut W {
        self.variant(MUX_A::TEMPSENS)
    }
    #[doc = "Internal Reference (VBG)"]
    #[inline(always)]
    pub fn adc_vbg(self) -> &'a mut W {
        self.variant(MUX_A::ADC_VBG)
    }
    #[doc = "0V (GND)"]
    #[inline(always)]
    pub fn adc_gnd(self) -> &'a mut W {
        self.variant(MUX_A::ADC_GND)
    }
}
#[doc = "Field `ADLAR` reader - Left Adjust Result"]
pub type ADLAR_R = crate::BitReader<bool>;
#[doc = "Field `ADLAR` writer - Left Adjust Result"]
pub type ADLAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADMUX_SPEC, bool, O>;
#[doc = "Field `REFS0` reader - Reference Selection Bit 0"]
pub type REFS0_R = crate::BitReader<REFS0_A>;
#[doc = "Reference Selection Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFS0_A {
    #[doc = "0: Internal 1.1V Voltage Reference"]
    INTERNAL = 0,
    #[doc = "1: AVcc Reference"]
    AVCC = 1,
}
impl From<REFS0_A> for bool {
    #[inline(always)]
    fn from(variant: REFS0_A) -> Self {
        variant as u8 != 0
    }
}
impl REFS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFS0_A {
        match self.bits {
            false => REFS0_A::INTERNAL,
            true => REFS0_A::AVCC,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == REFS0_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `AVCC`"]
    #[inline(always)]
    pub fn is_avcc(&self) -> bool {
        *self == REFS0_A::AVCC
    }
}
#[doc = "Field `REFS0` writer - Reference Selection Bit 0"]
pub type REFS0_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADMUX_SPEC, REFS0_A, O>;
impl<'a, const O: u8> REFS0_W<'a, O> {
    #[doc = "Internal 1.1V Voltage Reference"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(REFS0_A::INTERNAL)
    }
    #[doc = "AVcc Reference"]
    #[inline(always)]
    pub fn avcc(self) -> &'a mut W {
        self.variant(REFS0_A::AVCC)
    }
}
impl R {
    #[doc = "Bits 0:3 - Analog Channel Selection Bits"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 5 - Left Adjust Result"]
    #[inline(always)]
    pub fn adlar(&self) -> ADLAR_R {
        ADLAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reference Selection Bit 0"]
    #[inline(always)]
    pub fn refs0(&self) -> REFS0_R {
        REFS0_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Analog Channel Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<0> {
        MUX_W::new(self)
    }
    #[doc = "Bit 5 - Left Adjust Result"]
    #[inline(always)]
    #[must_use]
    pub fn adlar(&mut self) -> ADLAR_W<5> {
        ADLAR_W::new(self)
    }
    #[doc = "Bit 6 - Reference Selection Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn refs0(&mut self) -> REFS0_W<6> {
        REFS0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The ADC multiplexer Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [admux](index.html) module"]
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
