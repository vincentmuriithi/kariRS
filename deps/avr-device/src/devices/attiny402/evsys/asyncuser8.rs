#[doc = "Register `ASYNCUSER8` reader"]
pub struct R(crate::R<ASYNCUSER8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCUSER8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCUSER8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCUSER8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCUSER8` writer"]
pub struct W(crate::W<ASYNCUSER8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCUSER8_SPEC>;
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
impl From<crate::W<ASYNCUSER8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCUSER8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASYNCUSER8` reader - Asynchronous User Ch 8 Input Selection - Event Out 0"]
pub type ASYNCUSER8_R = crate::FieldReader<u8, ASYNCUSER8_A>;
#[doc = "Asynchronous User Ch 8 Input Selection - Event Out 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASYNCUSER8_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Synchronous Event Channel 0"]
    SYNCCH0 = 1,
    #[doc = "3: Asynchronous Event Channel 0"]
    ASYNCCH0 = 3,
    #[doc = "4: Asynchronous Event Channel 1"]
    ASYNCCH1 = 4,
}
impl From<ASYNCUSER8_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCUSER8_A) -> Self {
        variant as _
    }
}
impl ASYNCUSER8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASYNCUSER8_A> {
        match self.bits {
            0 => Some(ASYNCUSER8_A::OFF),
            1 => Some(ASYNCUSER8_A::SYNCCH0),
            3 => Some(ASYNCUSER8_A::ASYNCCH0),
            4 => Some(ASYNCUSER8_A::ASYNCCH1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCUSER8_A::OFF
    }
    #[doc = "Checks if the value of the field is `SYNCCH0`"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == ASYNCUSER8_A::SYNCCH0
    }
    #[doc = "Checks if the value of the field is `ASYNCCH0`"]
    #[inline(always)]
    pub fn is_asyncch0(&self) -> bool {
        *self == ASYNCUSER8_A::ASYNCCH0
    }
    #[doc = "Checks if the value of the field is `ASYNCCH1`"]
    #[inline(always)]
    pub fn is_asyncch1(&self) -> bool {
        *self == ASYNCUSER8_A::ASYNCCH1
    }
}
#[doc = "Field `ASYNCUSER8` writer - Asynchronous User Ch 8 Input Selection - Event Out 0"]
pub type ASYNCUSER8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, ASYNCUSER8_SPEC, u8, ASYNCUSER8_A, 8, O>;
impl<'a, const O: u8> ASYNCUSER8_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ASYNCUSER8_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut W {
        self.variant(ASYNCUSER8_A::SYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn asyncch0(self) -> &'a mut W {
        self.variant(ASYNCUSER8_A::ASYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn asyncch1(self) -> &'a mut W {
        self.variant(ASYNCUSER8_A::ASYNCCH1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous User Ch 8 Input Selection - Event Out 0"]
    #[inline(always)]
    pub fn asyncuser8(&self) -> ASYNCUSER8_R {
        ASYNCUSER8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous User Ch 8 Input Selection - Event Out 0"]
    #[inline(always)]
    #[must_use]
    pub fn asyncuser8(&mut self) -> ASYNCUSER8_W<0> {
        ASYNCUSER8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous User Ch 8 Input Selection - Event Out 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncuser8](index.html) module"]
pub struct ASYNCUSER8_SPEC;
impl crate::RegisterSpec for ASYNCUSER8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [asyncuser8::R](R) reader structure"]
impl crate::Readable for ASYNCUSER8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncuser8::W](W) writer structure"]
impl crate::Writable for ASYNCUSER8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCUSER8 to value 0"]
impl crate::Resettable for ASYNCUSER8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
