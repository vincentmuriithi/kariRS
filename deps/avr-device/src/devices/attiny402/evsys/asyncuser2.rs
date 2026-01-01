#[doc = "Register `ASYNCUSER2` reader"]
pub struct R(crate::R<ASYNCUSER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCUSER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCUSER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCUSER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCUSER2` writer"]
pub struct W(crate::W<ASYNCUSER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCUSER2_SPEC>;
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
impl From<crate::W<ASYNCUSER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCUSER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASYNCUSER2` reader - Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0"]
pub type ASYNCUSER2_R = crate::FieldReader<u8, ASYNCUSER2_A>;
#[doc = "Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASYNCUSER2_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Synchronous Event Channel 0"]
    SYNCCH0 = 1,
    #[doc = "3: Asynchronous Event Channel 0"]
    ASYNCCH0 = 3,
    #[doc = "4: Asynchronous Event Channel 1"]
    ASYNCCH1 = 4,
}
impl From<ASYNCUSER2_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCUSER2_A) -> Self {
        variant as _
    }
}
impl ASYNCUSER2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASYNCUSER2_A> {
        match self.bits {
            0 => Some(ASYNCUSER2_A::OFF),
            1 => Some(ASYNCUSER2_A::SYNCCH0),
            3 => Some(ASYNCUSER2_A::ASYNCCH0),
            4 => Some(ASYNCUSER2_A::ASYNCCH1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCUSER2_A::OFF
    }
    #[doc = "Checks if the value of the field is `SYNCCH0`"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == ASYNCUSER2_A::SYNCCH0
    }
    #[doc = "Checks if the value of the field is `ASYNCCH0`"]
    #[inline(always)]
    pub fn is_asyncch0(&self) -> bool {
        *self == ASYNCUSER2_A::ASYNCCH0
    }
    #[doc = "Checks if the value of the field is `ASYNCCH1`"]
    #[inline(always)]
    pub fn is_asyncch1(&self) -> bool {
        *self == ASYNCUSER2_A::ASYNCCH1
    }
}
#[doc = "Field `ASYNCUSER2` writer - Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0"]
pub type ASYNCUSER2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, ASYNCUSER2_SPEC, u8, ASYNCUSER2_A, 8, O>;
impl<'a, const O: u8> ASYNCUSER2_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ASYNCUSER2_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut W {
        self.variant(ASYNCUSER2_A::SYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn asyncch0(self) -> &'a mut W {
        self.variant(ASYNCUSER2_A::ASYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn asyncch1(self) -> &'a mut W {
        self.variant(ASYNCUSER2_A::ASYNCCH1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0"]
    #[inline(always)]
    pub fn asyncuser2(&self) -> ASYNCUSER2_R {
        ASYNCUSER2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0"]
    #[inline(always)]
    #[must_use]
    pub fn asyncuser2(&mut self) -> ASYNCUSER2_W<0> {
        ASYNCUSER2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncuser2](index.html) module"]
pub struct ASYNCUSER2_SPEC;
impl crate::RegisterSpec for ASYNCUSER2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [asyncuser2::R](R) reader structure"]
impl crate::Readable for ASYNCUSER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncuser2::W](W) writer structure"]
impl crate::Writable for ASYNCUSER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCUSER2 to value 0"]
impl crate::Resettable for ASYNCUSER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
