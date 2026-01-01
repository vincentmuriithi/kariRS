#[doc = "Register `HIGH` reader"]
pub struct R(crate::R<HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIGH` writer"]
pub struct W(crate::W<HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIGH_SPEC>;
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
impl From<crate::W<HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTDISBL` reader - Reset Disabled (Enable PB5 as i/o pin)"]
pub type RSTDISBL_R = crate::BitReader<bool>;
#[doc = "Field `RSTDISBL` writer - Reset Disabled (Enable PB5 as i/o pin)"]
pub type RSTDISBL_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
#[doc = "Field `BODLEVEL` reader - Enable BOD and select level"]
pub type BODLEVEL_R = crate::FieldReader<u8, BODLEVEL_A>;
#[doc = "Enable BOD and select level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BODLEVEL_A {
    #[doc = "0: Brown-out detection at VCC=4.3 V"]
    _4V3 = 0,
    #[doc = "1: Brown-out detection at VCC=2.7 V"]
    _2V7 = 1,
    #[doc = "2: Brown-out detection at VCC=1.8 V"]
    _1V8 = 2,
    #[doc = "3: Brown-out detection disabled"]
    DISABLED = 3,
}
impl From<BODLEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BODLEVEL_A) -> Self {
        variant as _
    }
}
impl BODLEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODLEVEL_A {
        match self.bits {
            0 => BODLEVEL_A::_4V3,
            1 => BODLEVEL_A::_2V7,
            2 => BODLEVEL_A::_1V8,
            3 => BODLEVEL_A::DISABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4V3`"]
    #[inline(always)]
    pub fn is_4v3(&self) -> bool {
        *self == BODLEVEL_A::_4V3
    }
    #[doc = "Checks if the value of the field is `_2V7`"]
    #[inline(always)]
    pub fn is_2v7(&self) -> bool {
        *self == BODLEVEL_A::_2V7
    }
    #[doc = "Checks if the value of the field is `_1V8`"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == BODLEVEL_A::_1V8
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BODLEVEL_A::DISABLED
    }
}
#[doc = "Field `BODLEVEL` writer - Enable BOD and select level"]
pub type BODLEVEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, HIGH_SPEC, u8, BODLEVEL_A, 2, O>;
impl<'a, const O: u8> BODLEVEL_W<'a, O> {
    #[doc = "Brown-out detection at VCC=4.3 V"]
    #[inline(always)]
    pub fn _4v3(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_4V3)
    }
    #[doc = "Brown-out detection at VCC=2.7 V"]
    #[inline(always)]
    pub fn _2v7(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_2V7)
    }
    #[doc = "Brown-out detection at VCC=1.8 V"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_1V8)
    }
    #[doc = "Brown-out detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BODLEVEL_A::DISABLED)
    }
}
#[doc = "Field `DWEN` reader - Debug Wire enable"]
pub type DWEN_R = crate::BitReader<bool>;
#[doc = "Field `DWEN` writer - Debug Wire enable"]
pub type DWEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
#[doc = "Field `SELFPRGEN` reader - Self Programming enable"]
pub type SELFPRGEN_R = crate::BitReader<bool>;
#[doc = "Field `SELFPRGEN` writer - Self Programming enable"]
pub type SELFPRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Reset Disabled (Enable PB5 as i/o pin)"]
    #[inline(always)]
    pub fn rstdisbl(&self) -> RSTDISBL_R {
        RSTDISBL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enable BOD and select level"]
    #[inline(always)]
    pub fn bodlevel(&self) -> BODLEVEL_R {
        BODLEVEL_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Debug Wire enable"]
    #[inline(always)]
    pub fn dwen(&self) -> DWEN_R {
        DWEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Self Programming enable"]
    #[inline(always)]
    pub fn selfprgen(&self) -> SELFPRGEN_R {
        SELFPRGEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Disabled (Enable PB5 as i/o pin)"]
    #[inline(always)]
    #[must_use]
    pub fn rstdisbl(&mut self) -> RSTDISBL_W<0> {
        RSTDISBL_W::new(self)
    }
    #[doc = "Bits 1:2 - Enable BOD and select level"]
    #[inline(always)]
    #[must_use]
    pub fn bodlevel(&mut self) -> BODLEVEL_W<1> {
        BODLEVEL_W::new(self)
    }
    #[doc = "Bit 3 - Debug Wire enable"]
    #[inline(always)]
    #[must_use]
    pub fn dwen(&mut self) -> DWEN_W<3> {
        DWEN_W::new(self)
    }
    #[doc = "Bit 4 - Self Programming enable"]
    #[inline(always)]
    #[must_use]
    pub fn selfprgen(&mut self) -> SELFPRGEN_W<4> {
        SELFPRGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [high](index.html) module"]
pub struct HIGH_SPEC;
impl crate::RegisterSpec for HIGH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [high::R](R) reader structure"]
impl crate::Readable for HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [high::W](W) writer structure"]
impl crate::Writable for HIGH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIGH to value 0"]
impl crate::Resettable for HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
