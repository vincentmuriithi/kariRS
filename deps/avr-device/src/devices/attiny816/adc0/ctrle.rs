#[doc = "Register `CTRLE` reader"]
pub struct R(crate::R<CTRLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLE` writer"]
pub struct W(crate::W<CTRLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLE_SPEC>;
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
impl From<crate::W<CTRLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINCM` reader - Window Comparator Mode"]
pub type WINCM_R = crate::FieldReader<u8, WINCM_A>;
#[doc = "Window Comparator Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WINCM_A {
    #[doc = "0: No Window Comparison"]
    NONE = 0,
    #[doc = "1: Below Window"]
    BELOW = 1,
    #[doc = "2: Above Window"]
    ABOVE = 2,
    #[doc = "3: Inside Window"]
    INSIDE = 3,
    #[doc = "4: Outside Window"]
    OUTSIDE = 4,
}
impl From<WINCM_A> for u8 {
    #[inline(always)]
    fn from(variant: WINCM_A) -> Self {
        variant as _
    }
}
impl WINCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WINCM_A> {
        match self.bits {
            0 => Some(WINCM_A::NONE),
            1 => Some(WINCM_A::BELOW),
            2 => Some(WINCM_A::ABOVE),
            3 => Some(WINCM_A::INSIDE),
            4 => Some(WINCM_A::OUTSIDE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WINCM_A::NONE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == WINCM_A::BELOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == WINCM_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `INSIDE`"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == WINCM_A::INSIDE
    }
    #[doc = "Checks if the value of the field is `OUTSIDE`"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == WINCM_A::OUTSIDE
    }
}
#[doc = "Field `WINCM` writer - Window Comparator Mode"]
pub type WINCM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLE_SPEC, u8, WINCM_A, 3, O>;
impl<'a, const O: u8> WINCM_W<'a, O> {
    #[doc = "No Window Comparison"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WINCM_A::NONE)
    }
    #[doc = "Below Window"]
    #[inline(always)]
    pub fn below(self) -> &'a mut W {
        self.variant(WINCM_A::BELOW)
    }
    #[doc = "Above Window"]
    #[inline(always)]
    pub fn above(self) -> &'a mut W {
        self.variant(WINCM_A::ABOVE)
    }
    #[doc = "Inside Window"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut W {
        self.variant(WINCM_A::INSIDE)
    }
    #[doc = "Outside Window"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut W {
        self.variant(WINCM_A::OUTSIDE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Comparator Mode"]
    #[inline(always)]
    pub fn wincm(&self) -> WINCM_R {
        WINCM_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Comparator Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wincm(&mut self) -> WINCM_W<0> {
        WINCM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrle](index.html) module"]
pub struct CTRLE_SPEC;
impl crate::RegisterSpec for CTRLE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrle::R](R) reader structure"]
impl crate::Readable for CTRLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrle::W](W) writer structure"]
impl crate::Writable for CTRLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLE to value 0"]
impl crate::Resettable for CTRLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
