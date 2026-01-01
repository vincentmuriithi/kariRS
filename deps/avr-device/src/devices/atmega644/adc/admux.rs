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
    #[doc = "8: ADC Differential Inputs Postive pin 0 Negative pin 0 10x Gain"]
    ADC0_ADC0_10X = 8,
    #[doc = "9: ADC Differential Inputs Postive pin 1 Negative pin 0 10x Gain"]
    ADC1_ADC0_10X = 9,
    #[doc = "10: ADC Differential Inputs Postive pin 0 Negative pin 0 200x Gain"]
    ADC0_ADC0_200X = 10,
    #[doc = "11: ADC Differential Inputs Postive pin 1 Negative pin 0 200x Gain"]
    ADC1_ADC0_200X = 11,
    #[doc = "12: ADC Differential Inputs Postive pin 2 Negative pin 2 10x Gain"]
    ADC2_ADC2_10X = 12,
    #[doc = "13: ADC Differential Inputs Postive pin 3 Negative pin 2 10x Gain"]
    ADC3_ADC2_10X = 13,
    #[doc = "14: ADC Differential Inputs Postive pin 2 Negative pin 2 200x Gain"]
    ADC2_ADC2_200X = 14,
    #[doc = "15: ADC Differential Inputs Postive pin 3 Negative pin 2 200x Gain"]
    ADC3_ADC2_200X = 15,
    #[doc = "16: ADC Differential Inputs Postive pin 0 Negative pin 1 1x Gain"]
    ADC0_ADC1_1X = 16,
    #[doc = "17: ADC Differential Inputs Postive pin 1 Negative pin 1 1x Gain"]
    ADC1_ADC1_1X = 17,
    #[doc = "18: ADC Differential Inputs Postive pin 2 Negative pin 1 1x Gain"]
    ADC2_ADC1_1X = 18,
    #[doc = "19: ADC Differential Inputs Postive pin 3 Negative pin 1 1x Gain"]
    ADC3_ADC1_1X = 19,
    #[doc = "20: ADC Differential Inputs Postive pin 4 Negative pin 1 1x Gain"]
    ADC4_ADC1_1X = 20,
    #[doc = "21: ADC Differential Inputs Postive pin 5 Negative pin 1 1x Gain"]
    ADC5_ADC1_1X = 21,
    #[doc = "22: ADC Differential Inputs Postive pin 6 Negative pin 1 1x Gain"]
    ADC6_ADC1_1X = 22,
    #[doc = "23: ADC Differential Inputs Postive pin 7 Negative pin 1 1x Gain"]
    ADC7_ADC1_1X = 23,
    #[doc = "24: ADC Differential Inputs Postive pin 0 Negative pin 2 1x Gain"]
    ADC0_ADC2_1X = 24,
    #[doc = "25: ADC Differential Inputs Postive pin 1 Negative pin 2 1x Gain"]
    ADC1_ADC2_1X = 25,
    #[doc = "26: ADC Differential Inputs Postive pin 2 Negative pin 2 1x Gain"]
    ADC2_ADC2_1X = 26,
    #[doc = "27: ADC Differential Inputs Postive pin 3 Negative pin 2 1x Gain"]
    ADC3_ADC2_1X = 27,
    #[doc = "28: ADC Differential Inputs Postive pin 4 Negative pin 2 1x Gain"]
    ADC4_ADC2_1X = 28,
    #[doc = "29: ADC Differential Inputs Postive pin 5 Negative pin 2 1x Gain"]
    ADC5_ADC2_1X = 29,
    #[doc = "30: Internal Reference (VBG)"]
    ADC_VBG = 30,
    #[doc = "31: 0V (GND)"]
    ADC_GND = 31,
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
    pub fn variant(&self) -> MUX_A {
        match self.bits {
            0 => MUX_A::ADC0,
            1 => MUX_A::ADC1,
            2 => MUX_A::ADC2,
            3 => MUX_A::ADC3,
            4 => MUX_A::ADC4,
            5 => MUX_A::ADC5,
            6 => MUX_A::ADC6,
            7 => MUX_A::ADC7,
            8 => MUX_A::ADC0_ADC0_10X,
            9 => MUX_A::ADC1_ADC0_10X,
            10 => MUX_A::ADC0_ADC0_200X,
            11 => MUX_A::ADC1_ADC0_200X,
            12 => MUX_A::ADC2_ADC2_10X,
            13 => MUX_A::ADC3_ADC2_10X,
            14 => MUX_A::ADC2_ADC2_200X,
            15 => MUX_A::ADC3_ADC2_200X,
            16 => MUX_A::ADC0_ADC1_1X,
            17 => MUX_A::ADC1_ADC1_1X,
            18 => MUX_A::ADC2_ADC1_1X,
            19 => MUX_A::ADC3_ADC1_1X,
            20 => MUX_A::ADC4_ADC1_1X,
            21 => MUX_A::ADC5_ADC1_1X,
            22 => MUX_A::ADC6_ADC1_1X,
            23 => MUX_A::ADC7_ADC1_1X,
            24 => MUX_A::ADC0_ADC2_1X,
            25 => MUX_A::ADC1_ADC2_1X,
            26 => MUX_A::ADC2_ADC2_1X,
            27 => MUX_A::ADC3_ADC2_1X,
            28 => MUX_A::ADC4_ADC2_1X,
            29 => MUX_A::ADC5_ADC2_1X,
            30 => MUX_A::ADC_VBG,
            31 => MUX_A::ADC_GND,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `ADC0_ADC0_10X`"]
    #[inline(always)]
    pub fn is_adc0_adc0_10x(&self) -> bool {
        *self == MUX_A::ADC0_ADC0_10X
    }
    #[doc = "Checks if the value of the field is `ADC1_ADC0_10X`"]
    #[inline(always)]
    pub fn is_adc1_adc0_10x(&self) -> bool {
        *self == MUX_A::ADC1_ADC0_10X
    }
    #[doc = "Checks if the value of the field is `ADC0_ADC0_200X`"]
    #[inline(always)]
    pub fn is_adc0_adc0_200x(&self) -> bool {
        *self == MUX_A::ADC0_ADC0_200X
    }
    #[doc = "Checks if the value of the field is `ADC1_ADC0_200X`"]
    #[inline(always)]
    pub fn is_adc1_adc0_200x(&self) -> bool {
        *self == MUX_A::ADC1_ADC0_200X
    }
    #[doc = "Checks if the value of the field is `ADC2_ADC2_10X`"]
    #[inline(always)]
    pub fn is_adc2_adc2_10x(&self) -> bool {
        *self == MUX_A::ADC2_ADC2_10X
    }
    #[doc = "Checks if the value of the field is `ADC3_ADC2_10X`"]
    #[inline(always)]
    pub fn is_adc3_adc2_10x(&self) -> bool {
        *self == MUX_A::ADC3_ADC2_10X
    }
    #[doc = "Checks if the value of the field is `ADC2_ADC2_200X`"]
    #[inline(always)]
    pub fn is_adc2_adc2_200x(&self) -> bool {
        *self == MUX_A::ADC2_ADC2_200X
    }
    #[doc = "Checks if the value of the field is `ADC3_ADC2_200X`"]
    #[inline(always)]
    pub fn is_adc3_adc2_200x(&self) -> bool {
        *self == MUX_A::ADC3_ADC2_200X
    }
    #[doc = "Checks if the value of the field is `ADC0_ADC1_1X`"]
    #[inline(always)]
    pub fn is_adc0_adc1_1x(&self) -> bool {
        *self == MUX_A::ADC0_ADC1_1X
    }
    #[doc = "Checks if the value of the field is `ADC1_ADC1_1X`"]
    #[inline(always)]
    pub fn is_adc1_adc1_1x(&self) -> bool {
        *self == MUX_A::ADC1_ADC1_1X
    }
    #[doc = "Checks if the value of the field is `ADC2_ADC1_1X`"]
    #[inline(always)]
    pub fn is_adc2_adc1_1x(&self) -> bool {
        *self == MUX_A::ADC2_ADC1_1X
    }
    #[doc = "Checks if the value of the field is `ADC3_ADC1_1X`"]
    #[inline(always)]
    pub fn is_adc3_adc1_1x(&self) -> bool {
        *self == MUX_A::ADC3_ADC1_1X
    }
    #[doc = "Checks if the value of the field is `ADC4_ADC1_1X`"]
    #[inline(always)]
    pub fn is_adc4_adc1_1x(&self) -> bool {
        *self == MUX_A::ADC4_ADC1_1X
    }
    #[doc = "Checks if the value of the field is `ADC5_ADC1_1X`"]
    #[inline(always)]
    pub fn is_adc5_adc1_1x(&self) -> bool {
        *self == MUX_A::ADC5_ADC1_1X
    }
    #[doc = "Checks if the value of the field is `ADC6_ADC1_1X`"]
    #[inline(always)]
    pub fn is_adc6_adc1_1x(&self) -> bool {
        *self == MUX_A::ADC6_ADC1_1X
    }
    #[doc = "Checks if the value of the field is `ADC7_ADC1_1X`"]
    #[inline(always)]
    pub fn is_adc7_adc1_1x(&self) -> bool {
        *self == MUX_A::ADC7_ADC1_1X
    }
    #[doc = "Checks if the value of the field is `ADC0_ADC2_1X`"]
    #[inline(always)]
    pub fn is_adc0_adc2_1x(&self) -> bool {
        *self == MUX_A::ADC0_ADC2_1X
    }
    #[doc = "Checks if the value of the field is `ADC1_ADC2_1X`"]
    #[inline(always)]
    pub fn is_adc1_adc2_1x(&self) -> bool {
        *self == MUX_A::ADC1_ADC2_1X
    }
    #[doc = "Checks if the value of the field is `ADC2_ADC2_1X`"]
    #[inline(always)]
    pub fn is_adc2_adc2_1x(&self) -> bool {
        *self == MUX_A::ADC2_ADC2_1X
    }
    #[doc = "Checks if the value of the field is `ADC3_ADC2_1X`"]
    #[inline(always)]
    pub fn is_adc3_adc2_1x(&self) -> bool {
        *self == MUX_A::ADC3_ADC2_1X
    }
    #[doc = "Checks if the value of the field is `ADC4_ADC2_1X`"]
    #[inline(always)]
    pub fn is_adc4_adc2_1x(&self) -> bool {
        *self == MUX_A::ADC4_ADC2_1X
    }
    #[doc = "Checks if the value of the field is `ADC5_ADC2_1X`"]
    #[inline(always)]
    pub fn is_adc5_adc2_1x(&self) -> bool {
        *self == MUX_A::ADC5_ADC2_1X
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
#[doc = "Field `MUX` writer - Analog Channel and Gain Selection Bits"]
pub type MUX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ADMUX_SPEC, u8, MUX_A, 5, O>;
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
    #[doc = "ADC Differential Inputs Postive pin 0 Negative pin 0 10x Gain"]
    #[inline(always)]
    pub fn adc0_adc0_10x(self) -> &'a mut W {
        self.variant(MUX_A::ADC0_ADC0_10X)
    }
    #[doc = "ADC Differential Inputs Postive pin 1 Negative pin 0 10x Gain"]
    #[inline(always)]
    pub fn adc1_adc0_10x(self) -> &'a mut W {
        self.variant(MUX_A::ADC1_ADC0_10X)
    }
    #[doc = "ADC Differential Inputs Postive pin 0 Negative pin 0 200x Gain"]
    #[inline(always)]
    pub fn adc0_adc0_200x(self) -> &'a mut W {
        self.variant(MUX_A::ADC0_ADC0_200X)
    }
    #[doc = "ADC Differential Inputs Postive pin 1 Negative pin 0 200x Gain"]
    #[inline(always)]
    pub fn adc1_adc0_200x(self) -> &'a mut W {
        self.variant(MUX_A::ADC1_ADC0_200X)
    }
    #[doc = "ADC Differential Inputs Postive pin 2 Negative pin 2 10x Gain"]
    #[inline(always)]
    pub fn adc2_adc2_10x(self) -> &'a mut W {
        self.variant(MUX_A::ADC2_ADC2_10X)
    }
    #[doc = "ADC Differential Inputs Postive pin 3 Negative pin 2 10x Gain"]
    #[inline(always)]
    pub fn adc3_adc2_10x(self) -> &'a mut W {
        self.variant(MUX_A::ADC3_ADC2_10X)
    }
    #[doc = "ADC Differential Inputs Postive pin 2 Negative pin 2 200x Gain"]
    #[inline(always)]
    pub fn adc2_adc2_200x(self) -> &'a mut W {
        self.variant(MUX_A::ADC2_ADC2_200X)
    }
    #[doc = "ADC Differential Inputs Postive pin 3 Negative pin 2 200x Gain"]
    #[inline(always)]
    pub fn adc3_adc2_200x(self) -> &'a mut W {
        self.variant(MUX_A::ADC3_ADC2_200X)
    }
    #[doc = "ADC Differential Inputs Postive pin 0 Negative pin 1 1x Gain"]
    #[inline(always)]
    pub fn adc0_adc1_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC0_ADC1_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 1 Negative pin 1 1x Gain"]
    #[inline(always)]
    pub fn adc1_adc1_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC1_ADC1_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 2 Negative pin 1 1x Gain"]
    #[inline(always)]
    pub fn adc2_adc1_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC2_ADC1_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 3 Negative pin 1 1x Gain"]
    #[inline(always)]
    pub fn adc3_adc1_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC3_ADC1_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 4 Negative pin 1 1x Gain"]
    #[inline(always)]
    pub fn adc4_adc1_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC4_ADC1_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 5 Negative pin 1 1x Gain"]
    #[inline(always)]
    pub fn adc5_adc1_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC5_ADC1_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 6 Negative pin 1 1x Gain"]
    #[inline(always)]
    pub fn adc6_adc1_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC6_ADC1_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 7 Negative pin 1 1x Gain"]
    #[inline(always)]
    pub fn adc7_adc1_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC7_ADC1_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 0 Negative pin 2 1x Gain"]
    #[inline(always)]
    pub fn adc0_adc2_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC0_ADC2_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 1 Negative pin 2 1x Gain"]
    #[inline(always)]
    pub fn adc1_adc2_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC1_ADC2_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 2 Negative pin 2 1x Gain"]
    #[inline(always)]
    pub fn adc2_adc2_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC2_ADC2_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 3 Negative pin 2 1x Gain"]
    #[inline(always)]
    pub fn adc3_adc2_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC3_ADC2_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 4 Negative pin 2 1x Gain"]
    #[inline(always)]
    pub fn adc4_adc2_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC4_ADC2_1X)
    }
    #[doc = "ADC Differential Inputs Postive pin 5 Negative pin 2 1x Gain"]
    #[inline(always)]
    pub fn adc5_adc2_1x(self) -> &'a mut W {
        self.variant(MUX_A::ADC5_ADC2_1X)
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
#[doc = "Field `REFS` reader - Reference Selection Bits"]
pub type REFS_R = crate::FieldReader<u8, REFS_A>;
#[doc = "Reference Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFS_A {
    #[doc = "0: Aref Internal Vref turned off"]
    AREF = 0,
    #[doc = "1: AVcc with external capacitor at AREF pin"]
    AVCC = 1,
    #[doc = "3: Internal 1.1V Voltage Reference with external capacitor at AREF pin"]
    INTERNAL = 3,
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
            0 => Some(REFS_A::AREF),
            1 => Some(REFS_A::AVCC),
            3 => Some(REFS_A::INTERNAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == REFS_A::AREF
    }
    #[doc = "Checks if the value of the field is `AVCC`"]
    #[inline(always)]
    pub fn is_avcc(&self) -> bool {
        *self == REFS_A::AVCC
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
    #[doc = "Aref Internal Vref turned off"]
    #[inline(always)]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFS_A::AREF)
    }
    #[doc = "AVcc with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn avcc(self) -> &'a mut W {
        self.variant(REFS_A::AVCC)
    }
    #[doc = "Internal 1.1V Voltage Reference with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(REFS_A::INTERNAL)
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Left Adjust Result"]
    #[inline(always)]
    pub fn adlar(&self) -> ADLAR_R {
        ADLAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reference Selection Bits"]
    #[inline(always)]
    pub fn refs(&self) -> REFS_R {
        REFS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog Channel and Gain Selection Bits"]
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
