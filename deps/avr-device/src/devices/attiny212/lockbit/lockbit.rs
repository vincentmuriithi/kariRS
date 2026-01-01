#[doc = "Register `LOCKBIT` reader"]
pub struct R(crate::R<LOCKBIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKBIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKBIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKBIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKBIT` writer"]
pub struct W(crate::W<LOCKBIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCKBIT_SPEC>;
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
impl From<crate::W<LOCKBIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCKBIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LB` reader - Lock Bits"]
pub type LB_R = crate::FieldReader<u8, LB_A>;
#[doc = "Lock Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LB_A {
    #[doc = "58: Read and write lock"]
    RWLOCK = 58,
    #[doc = "197: No locks"]
    NOLOCK = 197,
}
impl From<LB_A> for u8 {
    #[inline(always)]
    fn from(variant: LB_A) -> Self {
        variant as _
    }
}
impl LB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LB_A> {
        match self.bits {
            58 => Some(LB_A::RWLOCK),
            197 => Some(LB_A::NOLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RWLOCK`"]
    #[inline(always)]
    pub fn is_rwlock(&self) -> bool {
        *self == LB_A::RWLOCK
    }
    #[doc = "Checks if the value of the field is `NOLOCK`"]
    #[inline(always)]
    pub fn is_nolock(&self) -> bool {
        *self == LB_A::NOLOCK
    }
}
#[doc = "Field `LB` writer - Lock Bits"]
pub type LB_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LOCKBIT_SPEC, u8, LB_A, 8, O>;
impl<'a, const O: u8> LB_W<'a, O> {
    #[doc = "Read and write lock"]
    #[inline(always)]
    pub fn rwlock(self) -> &'a mut W {
        self.variant(LB_A::RWLOCK)
    }
    #[doc = "No locks"]
    #[inline(always)]
    pub fn nolock(self) -> &'a mut W {
        self.variant(LB_A::NOLOCK)
    }
}
impl R {
    #[doc = "Bits 0:7 - Lock Bits"]
    #[inline(always)]
    pub fn lb(&self) -> LB_R {
        LB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lock Bits"]
    #[inline(always)]
    #[must_use]
    pub fn lb(&mut self) -> LB_W<0> {
        LB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockbit](index.html) module"]
pub struct LOCKBIT_SPEC;
impl crate::RegisterSpec for LOCKBIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lockbit::R](R) reader structure"]
impl crate::Readable for LOCKBIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockbit::W](W) writer structure"]
impl crate::Writable for LOCKBIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCKBIT to value 0"]
impl crate::Resettable for LOCKBIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
