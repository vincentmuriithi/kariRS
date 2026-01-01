#[doc = "Register `ANT_DIV` reader"]
pub struct R(crate::R<ANT_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANT_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANT_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANT_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANT_DIV` writer"]
pub struct W(crate::W<ANT_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANT_DIV_SPEC>;
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
impl From<crate::W<ANT_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANT_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANT_CTRL` reader - Static Antenna Diversity Switch Control"]
pub type ANT_CTRL_R = crate::FieldReader<u8, ANT_CTRL_A>;
#[doc = "Static Antenna Diversity Switch Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANT_CTRL_A {
    #[doc = "1: Antenna 1: DIG1=H, DIG2=L"]
    ANT_1 = 1,
    #[doc = "2: Antenna 0: DIG1=L, DIG2=H"]
    ANT_0 = 2,
    #[doc = "3: Default value for ANT_EXT_SW_EN=0; Mandatory setting for applications not using Antenna Diversity"]
    ANT_RESET = 3,
}
impl From<ANT_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: ANT_CTRL_A) -> Self {
        variant as _
    }
}
impl ANT_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANT_CTRL_A {
        match self.bits {
            1 => ANT_CTRL_A::ANT_1,
            2 => ANT_CTRL_A::ANT_0,
            3 => ANT_CTRL_A::ANT_RESET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ANT_1`"]
    #[inline(always)]
    pub fn is_ant_1(&self) -> bool {
        *self == ANT_CTRL_A::ANT_1
    }
    #[doc = "Checks if the value of the field is `ANT_0`"]
    #[inline(always)]
    pub fn is_ant_0(&self) -> bool {
        *self == ANT_CTRL_A::ANT_0
    }
    #[doc = "Checks if the value of the field is `ANT_RESET`"]
    #[inline(always)]
    pub fn is_ant_reset(&self) -> bool {
        *self == ANT_CTRL_A::ANT_RESET
    }
}
#[doc = "Field `ANT_CTRL` writer - Static Antenna Diversity Switch Control"]
pub type ANT_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, ANT_DIV_SPEC, u8, ANT_CTRL_A, 2, O>;
impl<'a, const O: u8> ANT_CTRL_W<'a, O> {
    #[doc = "Antenna 1: DIG1=H, DIG2=L"]
    #[inline(always)]
    pub fn ant_1(self) -> &'a mut W {
        self.variant(ANT_CTRL_A::ANT_1)
    }
    #[doc = "Antenna 0: DIG1=L, DIG2=H"]
    #[inline(always)]
    pub fn ant_0(self) -> &'a mut W {
        self.variant(ANT_CTRL_A::ANT_0)
    }
    #[doc = "Default value for ANT_EXT_SW_EN=0; Mandatory setting for applications not using Antenna Diversity"]
    #[inline(always)]
    pub fn ant_reset(self) -> &'a mut W {
        self.variant(ANT_CTRL_A::ANT_RESET)
    }
}
#[doc = "Field `ANT_EXT_SW_EN` reader - Enable External Antenna Switch Control"]
pub type ANT_EXT_SW_EN_R = crate::BitReader<ANT_EXT_SW_EN_A>;
#[doc = "Enable External Antenna Switch Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANT_EXT_SW_EN_A {
    #[doc = "0: Antenna Diversity RF switch control disabled"]
    ANT_DIV_EXT_SW_DIS = 0,
    #[doc = "1: Antenna Diversity RF switch control enabled"]
    ANT_DIV_EXT_SW_EN = 1,
}
impl From<ANT_EXT_SW_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ANT_EXT_SW_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ANT_EXT_SW_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANT_EXT_SW_EN_A {
        match self.bits {
            false => ANT_EXT_SW_EN_A::ANT_DIV_EXT_SW_DIS,
            true => ANT_EXT_SW_EN_A::ANT_DIV_EXT_SW_EN,
        }
    }
    #[doc = "Checks if the value of the field is `ANT_DIV_EXT_SW_DIS`"]
    #[inline(always)]
    pub fn is_ant_div_ext_sw_dis(&self) -> bool {
        *self == ANT_EXT_SW_EN_A::ANT_DIV_EXT_SW_DIS
    }
    #[doc = "Checks if the value of the field is `ANT_DIV_EXT_SW_EN`"]
    #[inline(always)]
    pub fn is_ant_div_ext_sw_en(&self) -> bool {
        *self == ANT_EXT_SW_EN_A::ANT_DIV_EXT_SW_EN
    }
}
#[doc = "Field `ANT_EXT_SW_EN` writer - Enable External Antenna Switch Control"]
pub type ANT_EXT_SW_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, ANT_DIV_SPEC, ANT_EXT_SW_EN_A, O>;
impl<'a, const O: u8> ANT_EXT_SW_EN_W<'a, O> {
    #[doc = "Antenna Diversity RF switch control disabled"]
    #[inline(always)]
    pub fn ant_div_ext_sw_dis(self) -> &'a mut W {
        self.variant(ANT_EXT_SW_EN_A::ANT_DIV_EXT_SW_DIS)
    }
    #[doc = "Antenna Diversity RF switch control enabled"]
    #[inline(always)]
    pub fn ant_div_ext_sw_en(self) -> &'a mut W {
        self.variant(ANT_EXT_SW_EN_A::ANT_DIV_EXT_SW_EN)
    }
}
#[doc = "Field `ANT_DIV_EN` reader - Enable Antenna Diversity"]
pub type ANT_DIV_EN_R = crate::BitReader<ANT_DIV_EN_A>;
#[doc = "Enable Antenna Diversity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANT_DIV_EN_A {
    #[doc = "0: Antenna Diversity algorithm disabled"]
    ANTENNA_DIVERSITY_ALGORITHM_DISABLED = 0,
    #[doc = "1: Antenna Diversity algorithm enabled"]
    ANTENNA_DIVERSITY_ALGORITHM_ENABLED = 1,
}
impl From<ANT_DIV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ANT_DIV_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ANT_DIV_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANT_DIV_EN_A {
        match self.bits {
            false => ANT_DIV_EN_A::ANTENNA_DIVERSITY_ALGORITHM_DISABLED,
            true => ANT_DIV_EN_A::ANTENNA_DIVERSITY_ALGORITHM_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ANTENNA_DIVERSITY_ALGORITHM_DISABLED`"]
    #[inline(always)]
    pub fn is_antenna_diversity_algorithm_disabled(&self) -> bool {
        *self == ANT_DIV_EN_A::ANTENNA_DIVERSITY_ALGORITHM_DISABLED
    }
    #[doc = "Checks if the value of the field is `ANTENNA_DIVERSITY_ALGORITHM_ENABLED`"]
    #[inline(always)]
    pub fn is_antenna_diversity_algorithm_enabled(&self) -> bool {
        *self == ANT_DIV_EN_A::ANTENNA_DIVERSITY_ALGORITHM_ENABLED
    }
}
#[doc = "Field `ANT_DIV_EN` writer - Enable Antenna Diversity"]
pub type ANT_DIV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ANT_DIV_SPEC, ANT_DIV_EN_A, O>;
impl<'a, const O: u8> ANT_DIV_EN_W<'a, O> {
    #[doc = "Antenna Diversity algorithm disabled"]
    #[inline(always)]
    pub fn antenna_diversity_algorithm_disabled(self) -> &'a mut W {
        self.variant(ANT_DIV_EN_A::ANTENNA_DIVERSITY_ALGORITHM_DISABLED)
    }
    #[doc = "Antenna Diversity algorithm enabled"]
    #[inline(always)]
    pub fn antenna_diversity_algorithm_enabled(self) -> &'a mut W {
        self.variant(ANT_DIV_EN_A::ANTENNA_DIVERSITY_ALGORITHM_ENABLED)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ANT_DIV_SPEC, u8, u8, 3, O>;
#[doc = "Field `ANT_SEL` reader - Antenna Diversity Antenna Status"]
pub type ANT_SEL_R = crate::BitReader<ANT_SEL_A>;
#[doc = "Antenna Diversity Antenna Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANT_SEL_A {
    #[doc = "0: Antenna 0"]
    ANTENNA_0 = 0,
    #[doc = "1: Antenna 1"]
    ANTENNA_1 = 1,
}
impl From<ANT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ANT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ANT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANT_SEL_A {
        match self.bits {
            false => ANT_SEL_A::ANTENNA_0,
            true => ANT_SEL_A::ANTENNA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ANTENNA_0`"]
    #[inline(always)]
    pub fn is_antenna_0(&self) -> bool {
        *self == ANT_SEL_A::ANTENNA_0
    }
    #[doc = "Checks if the value of the field is `ANTENNA_1`"]
    #[inline(always)]
    pub fn is_antenna_1(&self) -> bool {
        *self == ANT_SEL_A::ANTENNA_1
    }
}
#[doc = "Field `ANT_SEL` writer - Antenna Diversity Antenna Status"]
pub type ANT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, ANT_DIV_SPEC, ANT_SEL_A, O>;
impl<'a, const O: u8> ANT_SEL_W<'a, O> {
    #[doc = "Antenna 0"]
    #[inline(always)]
    pub fn antenna_0(self) -> &'a mut W {
        self.variant(ANT_SEL_A::ANTENNA_0)
    }
    #[doc = "Antenna 1"]
    #[inline(always)]
    pub fn antenna_1(self) -> &'a mut W {
        self.variant(ANT_SEL_A::ANTENNA_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Static Antenna Diversity Switch Control"]
    #[inline(always)]
    pub fn ant_ctrl(&self) -> ANT_CTRL_R {
        ANT_CTRL_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Enable External Antenna Switch Control"]
    #[inline(always)]
    pub fn ant_ext_sw_en(&self) -> ANT_EXT_SW_EN_R {
        ANT_EXT_SW_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Antenna Diversity"]
    #[inline(always)]
    pub fn ant_div_en(&self) -> ANT_DIV_EN_R {
        ANT_DIV_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Antenna Diversity Antenna Status"]
    #[inline(always)]
    pub fn ant_sel(&self) -> ANT_SEL_R {
        ANT_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Static Antenna Diversity Switch Control"]
    #[inline(always)]
    #[must_use]
    pub fn ant_ctrl(&mut self) -> ANT_CTRL_W<0> {
        ANT_CTRL_W::new(self)
    }
    #[doc = "Bit 2 - Enable External Antenna Switch Control"]
    #[inline(always)]
    #[must_use]
    pub fn ant_ext_sw_en(&mut self) -> ANT_EXT_SW_EN_W<2> {
        ANT_EXT_SW_EN_W::new(self)
    }
    #[doc = "Bit 3 - Enable Antenna Diversity"]
    #[inline(always)]
    #[must_use]
    pub fn ant_div_en(&mut self) -> ANT_DIV_EN_W<3> {
        ANT_DIV_EN_W::new(self)
    }
    #[doc = "Bits 4:6 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<4> {
        RES_W::new(self)
    }
    #[doc = "Bit 7 - Antenna Diversity Antenna Status"]
    #[inline(always)]
    #[must_use]
    pub fn ant_sel(&mut self) -> ANT_SEL_W<7> {
        ANT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Antenna Diversity Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ant_div](index.html) module"]
pub struct ANT_DIV_SPEC;
impl crate::RegisterSpec for ANT_DIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ant_div::R](R) reader structure"]
impl crate::Readable for ANT_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ant_div::W](W) writer structure"]
impl crate::Writable for ANT_DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANT_DIV to value 0"]
impl crate::Resettable for ANT_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
