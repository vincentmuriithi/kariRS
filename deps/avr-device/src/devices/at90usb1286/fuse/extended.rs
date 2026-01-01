#[doc = "Register `EXTENDED` reader"]
pub struct R(crate::R<EXTENDED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTENDED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTENDED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTENDED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTENDED` writer"]
pub struct W(crate::W<EXTENDED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTENDED_SPEC>;
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
impl From<crate::W<EXTENDED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTENDED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BODLEVEL` reader - Brown-out Detector trigger level"]
pub type BODLEVEL_R = crate::FieldReader<u8, BODLEVEL_A>;
#[doc = "Brown-out Detector trigger level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BODLEVEL_A {
    #[doc = "0: Brown-out detection at VCC=4.3 V"]
    _4V3 = 0,
    #[doc = "1: Brown-out detection at VCC=3.5 V"]
    _3V5 = 1,
    #[doc = "2: Brown-out detection at VCC=3.4 V"]
    _3V4 = 2,
    #[doc = "3: Brown-out detection at VCC=2.6 V"]
    _2V6 = 3,
    #[doc = "4: Brown-out detection at VCC=2.4 V"]
    _2V4 = 4,
    #[doc = "5: Brown-out detection at VCC=2.2 V"]
    _2V2 = 5,
    #[doc = "6: Brown-out detection at VCC=2.0 V"]
    _2V0 = 6,
    #[doc = "7: Brown-out detection disabled; \\[BODLEVEL=111\\]"]
    DISABLED = 7,
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
            1 => BODLEVEL_A::_3V5,
            2 => BODLEVEL_A::_3V4,
            3 => BODLEVEL_A::_2V6,
            4 => BODLEVEL_A::_2V4,
            5 => BODLEVEL_A::_2V2,
            6 => BODLEVEL_A::_2V0,
            7 => BODLEVEL_A::DISABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4V3`"]
    #[inline(always)]
    pub fn is_4v3(&self) -> bool {
        *self == BODLEVEL_A::_4V3
    }
    #[doc = "Checks if the value of the field is `_3V5`"]
    #[inline(always)]
    pub fn is_3v5(&self) -> bool {
        *self == BODLEVEL_A::_3V5
    }
    #[doc = "Checks if the value of the field is `_3V4`"]
    #[inline(always)]
    pub fn is_3v4(&self) -> bool {
        *self == BODLEVEL_A::_3V4
    }
    #[doc = "Checks if the value of the field is `_2V6`"]
    #[inline(always)]
    pub fn is_2v6(&self) -> bool {
        *self == BODLEVEL_A::_2V6
    }
    #[doc = "Checks if the value of the field is `_2V4`"]
    #[inline(always)]
    pub fn is_2v4(&self) -> bool {
        *self == BODLEVEL_A::_2V4
    }
    #[doc = "Checks if the value of the field is `_2V2`"]
    #[inline(always)]
    pub fn is_2v2(&self) -> bool {
        *self == BODLEVEL_A::_2V2
    }
    #[doc = "Checks if the value of the field is `_2V0`"]
    #[inline(always)]
    pub fn is_2v0(&self) -> bool {
        *self == BODLEVEL_A::_2V0
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BODLEVEL_A::DISABLED
    }
}
#[doc = "Field `BODLEVEL` writer - Brown-out Detector trigger level"]
pub type BODLEVEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, EXTENDED_SPEC, u8, BODLEVEL_A, 3, O>;
impl<'a, const O: u8> BODLEVEL_W<'a, O> {
    #[doc = "Brown-out detection at VCC=4.3 V"]
    #[inline(always)]
    pub fn _4v3(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_4V3)
    }
    #[doc = "Brown-out detection at VCC=3.5 V"]
    #[inline(always)]
    pub fn _3v5(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_3V5)
    }
    #[doc = "Brown-out detection at VCC=3.4 V"]
    #[inline(always)]
    pub fn _3v4(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_3V4)
    }
    #[doc = "Brown-out detection at VCC=2.6 V"]
    #[inline(always)]
    pub fn _2v6(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_2V6)
    }
    #[doc = "Brown-out detection at VCC=2.4 V"]
    #[inline(always)]
    pub fn _2v4(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_2V4)
    }
    #[doc = "Brown-out detection at VCC=2.2 V"]
    #[inline(always)]
    pub fn _2v2(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_2V2)
    }
    #[doc = "Brown-out detection at VCC=2.0 V"]
    #[inline(always)]
    pub fn _2v0(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_2V0)
    }
    #[doc = "Brown-out detection disabled; \\[BODLEVEL=111\\]"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BODLEVEL_A::DISABLED)
    }
}
#[doc = "Field `HWBE` reader - Hardware Boot Enable"]
pub type HWBE_R = crate::BitReader<bool>;
#[doc = "Field `HWBE` writer - Hardware Boot Enable"]
pub type HWBE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EXTENDED_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Brown-out Detector trigger level"]
    #[inline(always)]
    pub fn bodlevel(&self) -> BODLEVEL_R {
        BODLEVEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Hardware Boot Enable"]
    #[inline(always)]
    pub fn hwbe(&self) -> HWBE_R {
        HWBE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Brown-out Detector trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn bodlevel(&mut self) -> BODLEVEL_W<0> {
        BODLEVEL_W::new(self)
    }
    #[doc = "Bit 3 - Hardware Boot Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwbe(&mut self) -> HWBE_W<3> {
        HWBE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extended](index.html) module"]
pub struct EXTENDED_SPEC;
impl crate::RegisterSpec for EXTENDED_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [extended::R](R) reader structure"]
impl crate::Readable for EXTENDED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extended::W](W) writer structure"]
impl crate::Writable for EXTENDED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTENDED to value 0"]
impl crate::Resettable for EXTENDED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
