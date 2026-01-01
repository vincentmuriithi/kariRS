#[doc = "Register `ASYNCUSER9` reader"]
pub struct R(crate::R<ASYNCUSER9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCUSER9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCUSER9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCUSER9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCUSER9` writer"]
pub struct W(crate::W<ASYNCUSER9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCUSER9_SPEC>;
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
impl From<crate::W<ASYNCUSER9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCUSER9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASYNCUSER9` reader - Asynchronous User Ch 9 Input Selection - Event Out 1"]
pub type ASYNCUSER9_R = crate::FieldReader<u8, ASYNCUSER9_A>;
#[doc = "Asynchronous User Ch 9 Input Selection - Event Out 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASYNCUSER9_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Synchronous Event Channel 0"]
    SYNCCH0 = 1,
    #[doc = "2: Synchronous Event Channel 1"]
    SYNCCH1 = 2,
    #[doc = "3: Asynchronous Event Channel 0"]
    ASYNCCH0 = 3,
    #[doc = "4: Asynchronous Event Channel 1"]
    ASYNCCH1 = 4,
    #[doc = "5: Asynchronous Event Channel 2"]
    ASYNCCH2 = 5,
    #[doc = "6: Asynchronous Event Channel 3"]
    ASYNCCH3 = 6,
}
impl From<ASYNCUSER9_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCUSER9_A) -> Self {
        variant as _
    }
}
impl ASYNCUSER9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASYNCUSER9_A> {
        match self.bits {
            0 => Some(ASYNCUSER9_A::OFF),
            1 => Some(ASYNCUSER9_A::SYNCCH0),
            2 => Some(ASYNCUSER9_A::SYNCCH1),
            3 => Some(ASYNCUSER9_A::ASYNCCH0),
            4 => Some(ASYNCUSER9_A::ASYNCCH1),
            5 => Some(ASYNCUSER9_A::ASYNCCH2),
            6 => Some(ASYNCUSER9_A::ASYNCCH3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCUSER9_A::OFF
    }
    #[doc = "Checks if the value of the field is `SYNCCH0`"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == ASYNCUSER9_A::SYNCCH0
    }
    #[doc = "Checks if the value of the field is `SYNCCH1`"]
    #[inline(always)]
    pub fn is_syncch1(&self) -> bool {
        *self == ASYNCUSER9_A::SYNCCH1
    }
    #[doc = "Checks if the value of the field is `ASYNCCH0`"]
    #[inline(always)]
    pub fn is_asyncch0(&self) -> bool {
        *self == ASYNCUSER9_A::ASYNCCH0
    }
    #[doc = "Checks if the value of the field is `ASYNCCH1`"]
    #[inline(always)]
    pub fn is_asyncch1(&self) -> bool {
        *self == ASYNCUSER9_A::ASYNCCH1
    }
    #[doc = "Checks if the value of the field is `ASYNCCH2`"]
    #[inline(always)]
    pub fn is_asyncch2(&self) -> bool {
        *self == ASYNCUSER9_A::ASYNCCH2
    }
    #[doc = "Checks if the value of the field is `ASYNCCH3`"]
    #[inline(always)]
    pub fn is_asyncch3(&self) -> bool {
        *self == ASYNCUSER9_A::ASYNCCH3
    }
}
#[doc = "Field `ASYNCUSER9` writer - Asynchronous User Ch 9 Input Selection - Event Out 1"]
pub type ASYNCUSER9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, ASYNCUSER9_SPEC, u8, ASYNCUSER9_A, 8, O>;
impl<'a, const O: u8> ASYNCUSER9_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ASYNCUSER9_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut W {
        self.variant(ASYNCUSER9_A::SYNCCH0)
    }
    #[doc = "Synchronous Event Channel 1"]
    #[inline(always)]
    pub fn syncch1(self) -> &'a mut W {
        self.variant(ASYNCUSER9_A::SYNCCH1)
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn asyncch0(self) -> &'a mut W {
        self.variant(ASYNCUSER9_A::ASYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn asyncch1(self) -> &'a mut W {
        self.variant(ASYNCUSER9_A::ASYNCCH1)
    }
    #[doc = "Asynchronous Event Channel 2"]
    #[inline(always)]
    pub fn asyncch2(self) -> &'a mut W {
        self.variant(ASYNCUSER9_A::ASYNCCH2)
    }
    #[doc = "Asynchronous Event Channel 3"]
    #[inline(always)]
    pub fn asyncch3(self) -> &'a mut W {
        self.variant(ASYNCUSER9_A::ASYNCCH3)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous User Ch 9 Input Selection - Event Out 1"]
    #[inline(always)]
    pub fn asyncuser9(&self) -> ASYNCUSER9_R {
        ASYNCUSER9_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous User Ch 9 Input Selection - Event Out 1"]
    #[inline(always)]
    #[must_use]
    pub fn asyncuser9(&mut self) -> ASYNCUSER9_W<0> {
        ASYNCUSER9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous User Ch 9 Input Selection - Event Out 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncuser9](index.html) module"]
pub struct ASYNCUSER9_SPEC;
impl crate::RegisterSpec for ASYNCUSER9_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [asyncuser9::R](R) reader structure"]
impl crate::Readable for ASYNCUSER9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncuser9::W](W) writer structure"]
impl crate::Writable for ASYNCUSER9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCUSER9 to value 0"]
impl crate::Resettable for ASYNCUSER9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
