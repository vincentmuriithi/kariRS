#[doc = "Register `MUXCTRLA` reader"]
pub struct R(crate::R<MUXCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUXCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUXCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUXCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUXCTRLA` writer"]
pub struct W(crate::W<MUXCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUXCTRLA_SPEC>;
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
impl From<crate::W<MUXCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUXCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUXNEG` reader - Negative Input MUX Selection"]
pub type MUXNEG_R = crate::FieldReader<u8, MUXNEG_A>;
#[doc = "Negative Input MUX Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXNEG_A {
    #[doc = "0: Negative Pin 0"]
    PIN0 = 0,
    #[doc = "1: Negative Pin 1"]
    PIN1 = 1,
    #[doc = "2: Voltage Reference"]
    VREF = 2,
    #[doc = "3: DAC output"]
    DAC = 3,
}
impl From<MUXNEG_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXNEG_A) -> Self {
        variant as _
    }
}
impl MUXNEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUXNEG_A {
        match self.bits {
            0 => MUXNEG_A::PIN0,
            1 => MUXNEG_A::PIN1,
            2 => MUXNEG_A::VREF,
            3 => MUXNEG_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXNEG_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXNEG_A::PIN1
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == MUXNEG_A::VREF
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == MUXNEG_A::DAC
    }
}
#[doc = "Field `MUXNEG` writer - Negative Input MUX Selection"]
pub type MUXNEG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, MUXCTRLA_SPEC, u8, MUXNEG_A, 2, O>;
impl<'a, const O: u8> MUXNEG_W<'a, O> {
    #[doc = "Negative Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN0)
    }
    #[doc = "Negative Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN1)
    }
    #[doc = "Voltage Reference"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(MUXNEG_A::VREF)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(MUXNEG_A::DAC)
    }
}
#[doc = "Field `MUXPOS` reader - Positive Input MUX Selection"]
pub type MUXPOS_R = crate::FieldReader<u8, MUXPOS_A>;
#[doc = "Positive Input MUX Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXPOS_A {
    #[doc = "0: Positive Pin 0"]
    PIN0 = 0,
    #[doc = "1: Positive Pin 1"]
    PIN1 = 1,
}
impl From<MUXPOS_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXPOS_A) -> Self {
        variant as _
    }
}
impl MUXPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUXPOS_A> {
        match self.bits {
            0 => Some(MUXPOS_A::PIN0),
            1 => Some(MUXPOS_A::PIN1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXPOS_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXPOS_A::PIN1
    }
}
#[doc = "Field `MUXPOS` writer - Positive Input MUX Selection"]
pub type MUXPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MUXCTRLA_SPEC, u8, MUXPOS_A, 2, O>;
impl<'a, const O: u8> MUXPOS_W<'a, O> {
    #[doc = "Positive Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN0)
    }
    #[doc = "Positive Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN1)
    }
}
#[doc = "Field `INVERT` reader - Invert AC Output"]
pub type INVERT_R = crate::BitReader<bool>;
#[doc = "Field `INVERT` writer - Invert AC Output"]
pub type INVERT_W<'a, const O: u8> = crate::BitWriter<'a, u8, MUXCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Negative Input MUX Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(self.bits & 3)
    }
    #[doc = "Bits 3:4 - Positive Input MUX Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 7 - Invert AC Output"]
    #[inline(always)]
    pub fn invert(&self) -> INVERT_R {
        INVERT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Negative Input MUX Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<0> {
        MUXNEG_W::new(self)
    }
    #[doc = "Bits 3:4 - Positive Input MUX Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<3> {
        MUXPOS_W::new(self)
    }
    #[doc = "Bit 7 - Invert AC Output"]
    #[inline(always)]
    #[must_use]
    pub fn invert(&mut self) -> INVERT_W<7> {
        INVERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mux Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [muxctrla](index.html) module"]
pub struct MUXCTRLA_SPEC;
impl crate::RegisterSpec for MUXCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [muxctrla::R](R) reader structure"]
impl crate::Readable for MUXCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [muxctrla::W](W) writer structure"]
impl crate::Writable for MUXCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXCTRLA to value 0"]
impl crate::Resettable for MUXCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
