#[doc = "Register `MUXCTRL` reader"]
pub struct R(crate::R<MUXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUXCTRL` writer"]
pub struct W(crate::W<MUXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUXCTRL_SPEC>;
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
impl From<crate::W<MUXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUXCTRL_SPEC>) -> Self {
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
    AINN0 = 0,
    #[doc = "1: Negative Pin 1"]
    AINN1 = 1,
    #[doc = "2: Negative Pin 2"]
    AINN2 = 2,
    #[doc = "4: DAC Reference"]
    DACREF = 4,
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
    pub fn variant(&self) -> Option<MUXNEG_A> {
        match self.bits {
            0 => Some(MUXNEG_A::AINN0),
            1 => Some(MUXNEG_A::AINN1),
            2 => Some(MUXNEG_A::AINN2),
            4 => Some(MUXNEG_A::DACREF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AINN0`"]
    #[inline(always)]
    pub fn is_ainn0(&self) -> bool {
        *self == MUXNEG_A::AINN0
    }
    #[doc = "Checks if the value of the field is `AINN1`"]
    #[inline(always)]
    pub fn is_ainn1(&self) -> bool {
        *self == MUXNEG_A::AINN1
    }
    #[doc = "Checks if the value of the field is `AINN2`"]
    #[inline(always)]
    pub fn is_ainn2(&self) -> bool {
        *self == MUXNEG_A::AINN2
    }
    #[doc = "Checks if the value of the field is `DACREF`"]
    #[inline(always)]
    pub fn is_dacref(&self) -> bool {
        *self == MUXNEG_A::DACREF
    }
}
#[doc = "Field `MUXNEG` writer - Negative Input MUX Selection"]
pub type MUXNEG_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MUXCTRL_SPEC, u8, MUXNEG_A, 3, O>;
impl<'a, const O: u8> MUXNEG_W<'a, O> {
    #[doc = "Negative Pin 0"]
    #[inline(always)]
    pub fn ainn0(self) -> &'a mut W {
        self.variant(MUXNEG_A::AINN0)
    }
    #[doc = "Negative Pin 1"]
    #[inline(always)]
    pub fn ainn1(self) -> &'a mut W {
        self.variant(MUXNEG_A::AINN1)
    }
    #[doc = "Negative Pin 2"]
    #[inline(always)]
    pub fn ainn2(self) -> &'a mut W {
        self.variant(MUXNEG_A::AINN2)
    }
    #[doc = "DAC Reference"]
    #[inline(always)]
    pub fn dacref(self) -> &'a mut W {
        self.variant(MUXNEG_A::DACREF)
    }
}
#[doc = "Field `MUXPOS` reader - Positive Input MUX Selection"]
pub type MUXPOS_R = crate::FieldReader<u8, MUXPOS_A>;
#[doc = "Positive Input MUX Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXPOS_A {
    #[doc = "0: Positive Pin 0"]
    AINP0 = 0,
    #[doc = "3: Positive Pin 3"]
    AINP3 = 3,
    #[doc = "4: Positive Pin 4"]
    AINP4 = 4,
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
            0 => Some(MUXPOS_A::AINP0),
            3 => Some(MUXPOS_A::AINP3),
            4 => Some(MUXPOS_A::AINP4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AINP0`"]
    #[inline(always)]
    pub fn is_ainp0(&self) -> bool {
        *self == MUXPOS_A::AINP0
    }
    #[doc = "Checks if the value of the field is `AINP3`"]
    #[inline(always)]
    pub fn is_ainp3(&self) -> bool {
        *self == MUXPOS_A::AINP3
    }
    #[doc = "Checks if the value of the field is `AINP4`"]
    #[inline(always)]
    pub fn is_ainp4(&self) -> bool {
        *self == MUXPOS_A::AINP4
    }
}
#[doc = "Field `MUXPOS` writer - Positive Input MUX Selection"]
pub type MUXPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MUXCTRL_SPEC, u8, MUXPOS_A, 3, O>;
impl<'a, const O: u8> MUXPOS_W<'a, O> {
    #[doc = "Positive Pin 0"]
    #[inline(always)]
    pub fn ainp0(self) -> &'a mut W {
        self.variant(MUXPOS_A::AINP0)
    }
    #[doc = "Positive Pin 3"]
    #[inline(always)]
    pub fn ainp3(self) -> &'a mut W {
        self.variant(MUXPOS_A::AINP3)
    }
    #[doc = "Positive Pin 4"]
    #[inline(always)]
    pub fn ainp4(self) -> &'a mut W {
        self.variant(MUXPOS_A::AINP4)
    }
}
#[doc = "Field `INITVAL` reader - AC Output Initial Value"]
pub type INITVAL_R = crate::BitReader<INITVAL_A>;
#[doc = "AC Output Initial Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITVAL_A {
    #[doc = "0: Output initialized to 0"]
    LOW = 0,
    #[doc = "1: Output initialized to 1"]
    HIGH = 1,
}
impl From<INITVAL_A> for bool {
    #[inline(always)]
    fn from(variant: INITVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl INITVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITVAL_A {
        match self.bits {
            false => INITVAL_A::LOW,
            true => INITVAL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == INITVAL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == INITVAL_A::HIGH
    }
}
#[doc = "Field `INITVAL` writer - AC Output Initial Value"]
pub type INITVAL_W<'a, const O: u8> = crate::BitWriter<'a, u8, MUXCTRL_SPEC, INITVAL_A, O>;
impl<'a, const O: u8> INITVAL_W<'a, O> {
    #[doc = "Output initialized to 0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(INITVAL_A::LOW)
    }
    #[doc = "Output initialized to 1"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(INITVAL_A::HIGH)
    }
}
#[doc = "Field `INVERT` reader - Invert AC Output"]
pub type INVERT_R = crate::BitReader<bool>;
#[doc = "Field `INVERT` writer - Invert AC Output"]
pub type INVERT_W<'a, const O: u8> = crate::BitWriter<'a, u8, MUXCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Negative Input MUX Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:5 - Positive Input MUX Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new((self.bits >> 3) & 7)
    }
    #[doc = "Bit 6 - AC Output Initial Value"]
    #[inline(always)]
    pub fn initval(&self) -> INITVAL_R {
        INITVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Invert AC Output"]
    #[inline(always)]
    pub fn invert(&self) -> INVERT_R {
        INVERT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Negative Input MUX Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<0> {
        MUXNEG_W::new(self)
    }
    #[doc = "Bits 3:5 - Positive Input MUX Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<3> {
        MUXPOS_W::new(self)
    }
    #[doc = "Bit 6 - AC Output Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn initval(&mut self) -> INITVAL_W<6> {
        INITVAL_W::new(self)
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
#[doc = "Mux Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [muxctrl](index.html) module"]
pub struct MUXCTRL_SPEC;
impl crate::RegisterSpec for MUXCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [muxctrl::R](R) reader structure"]
impl crate::Readable for MUXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [muxctrl::W](W) writer structure"]
impl crate::Writable for MUXCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXCTRL to value 0"]
impl crate::Resettable for MUXCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
