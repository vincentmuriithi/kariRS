#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPNUM` reader - Accumulation Samples"]
pub type SAMPNUM_R = crate::FieldReader<u8, SAMPNUM_A>;
#[doc = "Accumulation Samples\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAMPNUM_A {
    #[doc = "0: 1 ADC sample"]
    ACC1 = 0,
    #[doc = "1: Accumulate 2 samples"]
    ACC2 = 1,
    #[doc = "2: Accumulate 4 samples"]
    ACC4 = 2,
    #[doc = "3: Accumulate 8 samples"]
    ACC8 = 3,
    #[doc = "4: Accumulate 16 samples"]
    ACC16 = 4,
    #[doc = "5: Accumulate 32 samples"]
    ACC32 = 5,
    #[doc = "6: Accumulate 64 samples"]
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
            0 => Some(SAMPNUM_A::ACC1),
            1 => Some(SAMPNUM_A::ACC2),
            2 => Some(SAMPNUM_A::ACC4),
            3 => Some(SAMPNUM_A::ACC8),
            4 => Some(SAMPNUM_A::ACC16),
            5 => Some(SAMPNUM_A::ACC32),
            6 => Some(SAMPNUM_A::ACC64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ACC1`"]
    #[inline(always)]
    pub fn is_acc1(&self) -> bool {
        *self == SAMPNUM_A::ACC1
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
#[doc = "Field `SAMPNUM` writer - Accumulation Samples"]
pub type SAMPNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLB_SPEC, u8, SAMPNUM_A, 3, O>;
impl<'a, const O: u8> SAMPNUM_W<'a, O> {
    #[doc = "1 ADC sample"]
    #[inline(always)]
    pub fn acc1(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC1)
    }
    #[doc = "Accumulate 2 samples"]
    #[inline(always)]
    pub fn acc2(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC2)
    }
    #[doc = "Accumulate 4 samples"]
    #[inline(always)]
    pub fn acc4(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC4)
    }
    #[doc = "Accumulate 8 samples"]
    #[inline(always)]
    pub fn acc8(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC8)
    }
    #[doc = "Accumulate 16 samples"]
    #[inline(always)]
    pub fn acc16(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC16)
    }
    #[doc = "Accumulate 32 samples"]
    #[inline(always)]
    pub fn acc32(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC32)
    }
    #[doc = "Accumulate 64 samples"]
    #[inline(always)]
    pub fn acc64(self) -> &'a mut W {
        self.variant(SAMPNUM_A::ACC64)
    }
}
impl R {
    #[doc = "Bits 0:2 - Accumulation Samples"]
    #[inline(always)]
    pub fn sampnum(&self) -> SAMPNUM_R {
        SAMPNUM_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Accumulation Samples"]
    #[inline(always)]
    #[must_use]
    pub fn sampnum(&mut self) -> SAMPNUM_W<0> {
        SAMPNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
