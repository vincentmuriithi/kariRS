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
    #[doc = "4: Brown-out detection at VCC=4.3 V"]
    _4V3 = 4,
    #[doc = "5: Brown-out detection at VCC=2.7 V"]
    _2V7 = 5,
    #[doc = "6: Brown-out detection at VCC=1.8 V"]
    _1V8 = 6,
    #[doc = "7: Brown-out detection disabled"]
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
    pub fn variant(&self) -> Option<BODLEVEL_A> {
        match self.bits {
            4 => Some(BODLEVEL_A::_4V3),
            5 => Some(BODLEVEL_A::_2V7),
            6 => Some(BODLEVEL_A::_1V8),
            7 => Some(BODLEVEL_A::DISABLED),
            _ => None,
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
#[doc = "Field `BODLEVEL` writer - Brown-out Detector trigger level"]
pub type BODLEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, EXTENDED_SPEC, u8, BODLEVEL_A, 3, O>;
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
#[doc = "Field `CFD` reader - Clock Failure Detection"]
pub type CFD_R = crate::BitReader<CFD_A>;
#[doc = "Clock Failure Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFD_A {
    #[doc = "0: Disabled"]
    CFD_DISABLED = 0,
    #[doc = "1: Enabled"]
    CFD_ENABLED = 1,
}
impl From<CFD_A> for bool {
    #[inline(always)]
    fn from(variant: CFD_A) -> Self {
        variant as u8 != 0
    }
}
impl CFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFD_A {
        match self.bits {
            false => CFD_A::CFD_DISABLED,
            true => CFD_A::CFD_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CFD_DISABLED`"]
    #[inline(always)]
    pub fn is_cfd_disabled(&self) -> bool {
        *self == CFD_A::CFD_DISABLED
    }
    #[doc = "Checks if the value of the field is `CFD_ENABLED`"]
    #[inline(always)]
    pub fn is_cfd_enabled(&self) -> bool {
        *self == CFD_A::CFD_ENABLED
    }
}
#[doc = "Field `CFD` writer - Clock Failure Detection"]
pub type CFD_W<'a, const O: u8> = crate::BitWriter<'a, u8, EXTENDED_SPEC, CFD_A, O>;
impl<'a, const O: u8> CFD_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn cfd_disabled(self) -> &'a mut W {
        self.variant(CFD_A::CFD_DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cfd_enabled(self) -> &'a mut W {
        self.variant(CFD_A::CFD_ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:2 - Brown-out Detector trigger level"]
    #[inline(always)]
    pub fn bodlevel(&self) -> BODLEVEL_R {
        BODLEVEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Clock Failure Detection"]
    #[inline(always)]
    pub fn cfd(&self) -> CFD_R {
        CFD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Brown-out Detector trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn bodlevel(&mut self) -> BODLEVEL_W<0> {
        BODLEVEL_W::new(self)
    }
    #[doc = "Bit 3 - Clock Failure Detection"]
    #[inline(always)]
    #[must_use]
    pub fn cfd(&mut self) -> CFD_W<3> {
        CFD_W::new(self)
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
