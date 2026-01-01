#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC0REFSEL` reader - DAC0/AC0 reference select"]
pub type DAC0REFSEL_R = crate::FieldReader<u8, DAC0REFSEL_A>;
#[doc = "DAC0/AC0 reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC0REFSEL_A {
    #[doc = "0: Voltage reference at 0.55V"]
    _0V55 = 0,
    #[doc = "1: Voltage reference at 1.1V"]
    _1V1 = 1,
    #[doc = "2: Voltage reference at 2.5V"]
    _2V5 = 2,
    #[doc = "3: Voltage reference at 4.34V"]
    _4V34 = 3,
    #[doc = "4: Voltage reference at 1.5V"]
    _1V5 = 4,
}
impl From<DAC0REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC0REFSEL_A) -> Self {
        variant as _
    }
}
impl DAC0REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAC0REFSEL_A> {
        match self.bits {
            0 => Some(DAC0REFSEL_A::_0V55),
            1 => Some(DAC0REFSEL_A::_1V1),
            2 => Some(DAC0REFSEL_A::_2V5),
            3 => Some(DAC0REFSEL_A::_4V34),
            4 => Some(DAC0REFSEL_A::_1V5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0V55`"]
    #[inline(always)]
    pub fn is_0v55(&self) -> bool {
        *self == DAC0REFSEL_A::_0V55
    }
    #[doc = "Checks if the value of the field is `_1V1`"]
    #[inline(always)]
    pub fn is_1v1(&self) -> bool {
        *self == DAC0REFSEL_A::_1V1
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == DAC0REFSEL_A::_2V5
    }
    #[doc = "Checks if the value of the field is `_4V34`"]
    #[inline(always)]
    pub fn is_4v34(&self) -> bool {
        *self == DAC0REFSEL_A::_4V34
    }
    #[doc = "Checks if the value of the field is `_1V5`"]
    #[inline(always)]
    pub fn is_1v5(&self) -> bool {
        *self == DAC0REFSEL_A::_1V5
    }
}
#[doc = "Field `DAC0REFSEL` writer - DAC0/AC0 reference select"]
pub type DAC0REFSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CTRLA_SPEC, u8, DAC0REFSEL_A, 3, O>;
impl<'a, const O: u8> DAC0REFSEL_W<'a, O> {
    #[doc = "Voltage reference at 0.55V"]
    #[inline(always)]
    pub fn _0v55(self) -> &'a mut W {
        self.variant(DAC0REFSEL_A::_0V55)
    }
    #[doc = "Voltage reference at 1.1V"]
    #[inline(always)]
    pub fn _1v1(self) -> &'a mut W {
        self.variant(DAC0REFSEL_A::_1V1)
    }
    #[doc = "Voltage reference at 2.5V"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(DAC0REFSEL_A::_2V5)
    }
    #[doc = "Voltage reference at 4.34V"]
    #[inline(always)]
    pub fn _4v34(self) -> &'a mut W {
        self.variant(DAC0REFSEL_A::_4V34)
    }
    #[doc = "Voltage reference at 1.5V"]
    #[inline(always)]
    pub fn _1v5(self) -> &'a mut W {
        self.variant(DAC0REFSEL_A::_1V5)
    }
}
#[doc = "Field `ADC0REFSEL` reader - ADC0 reference select"]
pub type ADC0REFSEL_R = crate::FieldReader<u8, ADC0REFSEL_A>;
#[doc = "ADC0 reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC0REFSEL_A {
    #[doc = "0: Voltage reference at 0.55V"]
    _0V55 = 0,
    #[doc = "1: Voltage reference at 1.1V"]
    _1V1 = 1,
    #[doc = "2: Voltage reference at 2.5V"]
    _2V5 = 2,
    #[doc = "3: Voltage reference at 4.34V"]
    _4V34 = 3,
    #[doc = "4: Voltage reference at 1.5V"]
    _1V5 = 4,
}
impl From<ADC0REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0REFSEL_A) -> Self {
        variant as _
    }
}
impl ADC0REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC0REFSEL_A> {
        match self.bits {
            0 => Some(ADC0REFSEL_A::_0V55),
            1 => Some(ADC0REFSEL_A::_1V1),
            2 => Some(ADC0REFSEL_A::_2V5),
            3 => Some(ADC0REFSEL_A::_4V34),
            4 => Some(ADC0REFSEL_A::_1V5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0V55`"]
    #[inline(always)]
    pub fn is_0v55(&self) -> bool {
        *self == ADC0REFSEL_A::_0V55
    }
    #[doc = "Checks if the value of the field is `_1V1`"]
    #[inline(always)]
    pub fn is_1v1(&self) -> bool {
        *self == ADC0REFSEL_A::_1V1
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == ADC0REFSEL_A::_2V5
    }
    #[doc = "Checks if the value of the field is `_4V34`"]
    #[inline(always)]
    pub fn is_4v34(&self) -> bool {
        *self == ADC0REFSEL_A::_4V34
    }
    #[doc = "Checks if the value of the field is `_1V5`"]
    #[inline(always)]
    pub fn is_1v5(&self) -> bool {
        *self == ADC0REFSEL_A::_1V5
    }
}
#[doc = "Field `ADC0REFSEL` writer - ADC0 reference select"]
pub type ADC0REFSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CTRLA_SPEC, u8, ADC0REFSEL_A, 3, O>;
impl<'a, const O: u8> ADC0REFSEL_W<'a, O> {
    #[doc = "Voltage reference at 0.55V"]
    #[inline(always)]
    pub fn _0v55(self) -> &'a mut W {
        self.variant(ADC0REFSEL_A::_0V55)
    }
    #[doc = "Voltage reference at 1.1V"]
    #[inline(always)]
    pub fn _1v1(self) -> &'a mut W {
        self.variant(ADC0REFSEL_A::_1V1)
    }
    #[doc = "Voltage reference at 2.5V"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(ADC0REFSEL_A::_2V5)
    }
    #[doc = "Voltage reference at 4.34V"]
    #[inline(always)]
    pub fn _4v34(self) -> &'a mut W {
        self.variant(ADC0REFSEL_A::_4V34)
    }
    #[doc = "Voltage reference at 1.5V"]
    #[inline(always)]
    pub fn _1v5(self) -> &'a mut W {
        self.variant(ADC0REFSEL_A::_1V5)
    }
}
impl R {
    #[doc = "Bits 0:2 - DAC0/AC0 reference select"]
    #[inline(always)]
    pub fn dac0refsel(&self) -> DAC0REFSEL_R {
        DAC0REFSEL_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - ADC0 reference select"]
    #[inline(always)]
    pub fn adc0refsel(&self) -> ADC0REFSEL_R {
        ADC0REFSEL_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC0/AC0 reference select"]
    #[inline(always)]
    #[must_use]
    pub fn dac0refsel(&mut self) -> DAC0REFSEL_W<0> {
        DAC0REFSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - ADC0 reference select"]
    #[inline(always)]
    #[must_use]
    pub fn adc0refsel(&mut self) -> ADC0REFSEL_W<4> {
        ADC0REFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
