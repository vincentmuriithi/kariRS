#[doc = "Register `CTRLF` reader"]
pub struct R(crate::R<CTRLF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLF` writer"]
pub struct W(crate::W<CTRLF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLF_SPEC>;
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
impl From<crate::W<CTRLF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPNUM` reader - Sampling Number"]
pub type SAMPNUM_R = crate::FieldReader<u8, SAMPNUM_A>;
#[doc = "Sampling Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAMPNUM_A {
    #[doc = "0: No accumulation"]
    NONE = 0,
    #[doc = "1: 2 results accumulated"]
    ACC2 = 1,
    #[doc = "2: 4 results accumulated"]
    ACC4 = 2,
    #[doc = "3: 8 results accumulated"]
    ACC8 = 3,
    #[doc = "4: 16 results accumulated"]
    ACC16 = 4,
    #[doc = "5: 32 results accumulated"]
    ACC32 = 5,
    #[doc = "6: 64 results accumulated"]
    ACC64 = 6,
}
impl From<SAMPNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPNUM_A) -> Self {
        variant as _
    }
}
impl SAMPNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAMPNUM_A> {
        match self.bits {
            0 => Some(SAMPNUM_A::NONE),
            1 => Some(SAMPNUM_A::ACC2),
            2 => Some(SAMPNUM_A::ACC4),
            3 => Some(SAMPNUM_A::ACC8),
            4 => Some(SAMPNUM_A::ACC16),
            5 => Some(SAMPNUM_A::ACC32),
            6 => Some(SAMPNUM_A::ACC64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SAMPNUM_A::NONE
    }
    #[doc = "Checks if the value of the field is `ACC2`"]
    #[inline(always)]
    pub fn is_acc2(&self) -> bool {
        *self == SAMPNUM_A::ACC2
    }
    #[doc = "Checks if the value of the field is `ACC4`"]
    #[inline(always)]
    pub fn is_acc4(&self) -> bool {
        *self == SAMPNUM_A::ACC4
    }
    #[doc = "Checks if the value of the field is `ACC8`"]
    #[inline(always)]
    pub fn is_acc8(&self) -> bool {
        *self == SAMPNUM_A::ACC8
    }
    #[doc = "Checks if the value of the field is `ACC16`"]
    #[inline(always)]
    pub fn is_acc16(&self) -> bool {
        *self == SAMPNUM_A::ACC16
    }
    #[doc = "Checks if the value of the field is `ACC32`"]
    #[inline(always)]
    pub fn is_acc32(&self) -> bool {
        *self == SAMPNUM_A::ACC32
    }
    #[doc = "Checks if the value of the field is `ACC64`"]
    #[inline(always)]
    pub fn is_acc64(&self) -> bool {
        *self == SAMPNUM_A::ACC64
    }
}
#[doc = "Field `SAMPNUM` writer - Sampling Number"]
pub type SAMPNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLF_SPEC, u8, SAMPNUM_A, 3, O>;
impl<'a, const O: u8> SAMPNUM_W<'a, O> {
    #[doc = "No accumulation"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SAMPNUM_A::NONE)
    }
    #[doc = "2 results accumulated"]
    #[inline(always)]
    pub fn acc2(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC2)
    }
    #[doc = "4 results accumulated"]
    #[inline(always)]
    pub fn acc4(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC4)
    }
    #[doc = "8 results accumulated"]
    #[inline(always)]
    pub fn acc8(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC8)
    }
    #[doc = "16 results accumulated"]
    #[inline(always)]
    pub fn acc16(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC16)
    }
    #[doc = "32 results accumulated"]
    #[inline(always)]
    pub fn acc32(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC32)
    }
    #[doc = "64 results accumulated"]
    #[inline(always)]
    pub fn acc64(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC64)
    }
}
#[doc = "Field `LEFTADJ` reader - Left Adjust"]
pub type LEFTADJ_R = crate::BitReader<bool>;
#[doc = "Field `LEFTADJ` writer - Left Adjust"]
pub type LEFTADJ_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLF_SPEC, bool, O>;
#[doc = "Field `FREERUN` reader - Free Running"]
pub type FREERUN_R = crate::BitReader<bool>;
#[doc = "Field `FREERUN` writer - Free Running"]
pub type FREERUN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Sampling Number"]
    #[inline(always)]
    pub fn sampnum(&self) -> SAMPNUM_R {
        SAMPNUM_R::new(self.bits & 7)
    }
    #[doc = "Bit 4 - Left Adjust"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Free Running"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling Number"]
    #[inline(always)]
    #[must_use]
    pub fn sampnum(&mut self) -> SAMPNUM_W<0> {
        SAMPNUM_W::new(self)
    }
    #[doc = "Bit 4 - Left Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn leftadj(&mut self) -> LEFTADJ_W<4> {
        LEFTADJ_W::new(self)
    }
    #[doc = "Bit 5 - Free Running"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FREERUN_W<5> {
        FREERUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlf](index.html) module"]
pub struct CTRLF_SPEC;
impl crate::RegisterSpec for CTRLF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlf::R](R) reader structure"]
impl crate::Readable for CTRLF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlf::W](W) writer structure"]
impl crate::Writable for CTRLF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLF to value 0"]
impl crate::Resettable for CTRLF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
