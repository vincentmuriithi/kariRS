#[doc = "Register `ASYNCUSER3` reader"]
pub struct R(crate::R<ASYNCUSER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCUSER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCUSER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCUSER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCUSER3` writer"]
pub struct W(crate::W<ASYNCUSER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCUSER3_SPEC>;
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
impl From<crate::W<ASYNCUSER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCUSER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASYNCUSER3` reader - Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0"]
pub type ASYNCUSER3_R = crate::FieldReader<u8, ASYNCUSER3_A>;
#[doc = "Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASYNCUSER3_A {
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
impl From<ASYNCUSER3_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCUSER3_A) -> Self {
        variant as _
    }
}
impl ASYNCUSER3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASYNCUSER3_A> {
        match self.bits {
            0 => Some(ASYNCUSER3_A::OFF),
            1 => Some(ASYNCUSER3_A::SYNCCH0),
            2 => Some(ASYNCUSER3_A::SYNCCH1),
            3 => Some(ASYNCUSER3_A::ASYNCCH0),
            4 => Some(ASYNCUSER3_A::ASYNCCH1),
            5 => Some(ASYNCUSER3_A::ASYNCCH2),
            6 => Some(ASYNCUSER3_A::ASYNCCH3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCUSER3_A::OFF
    }
    #[doc = "Checks if the value of the field is `SYNCCH0`"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == ASYNCUSER3_A::SYNCCH0
    }
    #[doc = "Checks if the value of the field is `SYNCCH1`"]
    #[inline(always)]
    pub fn is_syncch1(&self) -> bool {
        *self == ASYNCUSER3_A::SYNCCH1
    }
    #[doc = "Checks if the value of the field is `ASYNCCH0`"]
    #[inline(always)]
    pub fn is_asyncch0(&self) -> bool {
        *self == ASYNCUSER3_A::ASYNCCH0
    }
    #[doc = "Checks if the value of the field is `ASYNCCH1`"]
    #[inline(always)]
    pub fn is_asyncch1(&self) -> bool {
        *self == ASYNCUSER3_A::ASYNCCH1
    }
    #[doc = "Checks if the value of the field is `ASYNCCH2`"]
    #[inline(always)]
    pub fn is_asyncch2(&self) -> bool {
        *self == ASYNCUSER3_A::ASYNCCH2
    }
    #[doc = "Checks if the value of the field is `ASYNCCH3`"]
    #[inline(always)]
    pub fn is_asyncch3(&self) -> bool {
        *self == ASYNCUSER3_A::ASYNCCH3
    }
}
#[doc = "Field `ASYNCUSER3` writer - Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0"]
pub type ASYNCUSER3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, ASYNCUSER3_SPEC, u8, ASYNCUSER3_A, 8, O>;
impl<'a, const O: u8> ASYNCUSER3_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ASYNCUSER3_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut W {
        self.variant(ASYNCUSER3_A::SYNCCH0)
    }
    #[doc = "Synchronous Event Channel 1"]
    #[inline(always)]
    pub fn syncch1(self) -> &'a mut W {
        self.variant(ASYNCUSER3_A::SYNCCH1)
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn asyncch0(self) -> &'a mut W {
        self.variant(ASYNCUSER3_A::ASYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn asyncch1(self) -> &'a mut W {
        self.variant(ASYNCUSER3_A::ASYNCCH1)
    }
    #[doc = "Asynchronous Event Channel 2"]
    #[inline(always)]
    pub fn asyncch2(self) -> &'a mut W {
        self.variant(ASYNCUSER3_A::ASYNCCH2)
    }
    #[doc = "Asynchronous Event Channel 3"]
    #[inline(always)]
    pub fn asyncch3(self) -> &'a mut W {
        self.variant(ASYNCUSER3_A::ASYNCCH3)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0"]
    #[inline(always)]
    pub fn asyncuser3(&self) -> ASYNCUSER3_R {
        ASYNCUSER3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0"]
    #[inline(always)]
    #[must_use]
    pub fn asyncuser3(&mut self) -> ASYNCUSER3_W<0> {
        ASYNCUSER3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncuser3](index.html) module"]
pub struct ASYNCUSER3_SPEC;
impl crate::RegisterSpec for ASYNCUSER3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [asyncuser3::R](R) reader structure"]
impl crate::Readable for ASYNCUSER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncuser3::W](W) writer structure"]
impl crate::Writable for ASYNCUSER3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCUSER3 to value 0"]
impl crate::Resettable for ASYNCUSER3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
