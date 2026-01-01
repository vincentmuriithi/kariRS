#[doc = "Register `CCLROUTEA` reader"]
pub struct R(crate::R<CCLROUTEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCLROUTEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCLROUTEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCLROUTEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCLROUTEA` writer"]
pub struct W(crate::W<CCLROUTEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCLROUTEA_SPEC>;
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
impl From<crate::W<CCLROUTEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCLROUTEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT0` reader - CCL Look-Up Table 0 Signals"]
pub type LUT0_R = crate::BitReader<LUT0_A>;
#[doc = "CCL Look-Up Table 0 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT0_A {
    #[doc = "0: In: PA0, PA1, PA2, Out: PA3"]
    DEFAULT = 0,
    #[doc = "1: In: PA0, PA1, PA2, Out: PA6"]
    ALT1 = 1,
}
impl From<LUT0_A> for bool {
    #[inline(always)]
    fn from(variant: LUT0_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUT0_A {
        match self.bits {
            false => LUT0_A::DEFAULT,
            true => LUT0_A::ALT1,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LUT0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == LUT0_A::ALT1
    }
}
#[doc = "Field `LUT0` writer - CCL Look-Up Table 0 Signals"]
pub type LUT0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CCLROUTEA_SPEC, LUT0_A, O>;
impl<'a, const O: u8> LUT0_W<'a, O> {
    #[doc = "In: PA0, PA1, PA2, Out: PA3"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(LUT0_A::DEFAULT)
    }
    #[doc = "In: PA0, PA1, PA2, Out: PA6"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(LUT0_A::ALT1)
    }
}
#[doc = "Field `LUT1` reader - CCL Look-Up Table 1 Signals"]
pub type LUT1_R = crate::BitReader<LUT1_A>;
#[doc = "CCL Look-Up Table 1 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT1_A {
    #[doc = "0: In: -, -, -, Out: PC3"]
    DEFAULT = 0,
}
impl From<LUT1_A> for bool {
    #[inline(always)]
    fn from(variant: LUT1_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LUT1_A> {
        match self.bits {
            false => Some(LUT1_A::DEFAULT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LUT1_A::DEFAULT
    }
}
#[doc = "Field `LUT1` writer - CCL Look-Up Table 1 Signals"]
pub type LUT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, CCLROUTEA_SPEC, LUT1_A, O>;
impl<'a, const O: u8> LUT1_W<'a, O> {
    #[doc = "In: -, -, -, Out: PC3"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(LUT1_A::DEFAULT)
    }
}
#[doc = "Field `LUT2` reader - CCL Look-Up Table 2 Signals"]
pub type LUT2_R = crate::BitReader<LUT2_A>;
#[doc = "CCL Look-Up Table 2 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT2_A {
    #[doc = "0: In: PD0, PD1, PD2, Out: PD3"]
    DEFAULT = 0,
    #[doc = "1: In: PD0, PD1, PD2, Out: PD6"]
    ALT1 = 1,
}
impl From<LUT2_A> for bool {
    #[inline(always)]
    fn from(variant: LUT2_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUT2_A {
        match self.bits {
            false => LUT2_A::DEFAULT,
            true => LUT2_A::ALT1,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LUT2_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == LUT2_A::ALT1
    }
}
#[doc = "Field `LUT2` writer - CCL Look-Up Table 2 Signals"]
pub type LUT2_W<'a, const O: u8> = crate::BitWriter<'a, u8, CCLROUTEA_SPEC, LUT2_A, O>;
impl<'a, const O: u8> LUT2_W<'a, O> {
    #[doc = "In: PD0, PD1, PD2, Out: PD3"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(LUT2_A::DEFAULT)
    }
    #[doc = "In: PD0, PD1, PD2, Out: PD6"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(LUT2_A::ALT1)
    }
}
#[doc = "Field `LUT3` reader - CCL Look-Up Table 3 Signals"]
pub type LUT3_R = crate::BitReader<LUT3_A>;
#[doc = "CCL Look-Up Table 3 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT3_A {
    #[doc = "0: In: PF0, PF1, -, Out: -"]
    DEFAULT = 0,
}
impl From<LUT3_A> for bool {
    #[inline(always)]
    fn from(variant: LUT3_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LUT3_A> {
        match self.bits {
            false => Some(LUT3_A::DEFAULT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LUT3_A::DEFAULT
    }
}
#[doc = "Field `LUT3` writer - CCL Look-Up Table 3 Signals"]
pub type LUT3_W<'a, const O: u8> = crate::BitWriter<'a, u8, CCLROUTEA_SPEC, LUT3_A, O>;
impl<'a, const O: u8> LUT3_W<'a, O> {
    #[doc = "In: PF0, PF1, -, Out: -"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(LUT3_A::DEFAULT)
    }
}
impl R {
    #[doc = "Bit 0 - CCL Look-Up Table 0 Signals"]
    #[inline(always)]
    pub fn lut0(&self) -> LUT0_R {
        LUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCL Look-Up Table 1 Signals"]
    #[inline(always)]
    pub fn lut1(&self) -> LUT1_R {
        LUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCL Look-Up Table 2 Signals"]
    #[inline(always)]
    pub fn lut2(&self) -> LUT2_R {
        LUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCL Look-Up Table 3 Signals"]
    #[inline(always)]
    pub fn lut3(&self) -> LUT3_R {
        LUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCL Look-Up Table 0 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn lut0(&mut self) -> LUT0_W<0> {
        LUT0_W::new(self)
    }
    #[doc = "Bit 1 - CCL Look-Up Table 1 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn lut1(&mut self) -> LUT1_W<1> {
        LUT1_W::new(self)
    }
    #[doc = "Bit 2 - CCL Look-Up Table 2 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn lut2(&mut self) -> LUT2_W<2> {
        LUT2_W::new(self)
    }
    #[doc = "Bit 3 - CCL Look-Up Table 3 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn lut3(&mut self) -> LUT3_W<3> {
        LUT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCL route A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cclroutea](index.html) module"]
pub struct CCLROUTEA_SPEC;
impl crate::RegisterSpec for CCLROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cclroutea::R](R) reader structure"]
impl crate::Readable for CCLROUTEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cclroutea::W](W) writer structure"]
impl crate::Writable for CCLROUTEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCLROUTEA to value 0"]
impl crate::Resettable for CCLROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
