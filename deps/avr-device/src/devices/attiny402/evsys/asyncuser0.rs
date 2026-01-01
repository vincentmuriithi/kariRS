#[doc = "Register `ASYNCUSER0` reader"]
pub struct R(crate::R<ASYNCUSER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCUSER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCUSER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCUSER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCUSER0` writer"]
pub struct W(crate::W<ASYNCUSER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCUSER0_SPEC>;
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
impl From<crate::W<ASYNCUSER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCUSER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASYNCUSER0` reader - Asynchronous User Ch 0 Input Selection - TCB0"]
pub type ASYNCUSER0_R = crate::FieldReader<u8, ASYNCUSER0_A>;
#[doc = "Asynchronous User Ch 0 Input Selection - TCB0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASYNCUSER0_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Synchronous Event Channel 0"]
    SYNCCH0 = 1,
    #[doc = "3: Asynchronous Event Channel 0"]
    ASYNCCH0 = 3,
    #[doc = "4: Asynchronous Event Channel 1"]
    ASYNCCH1 = 4,
}
impl From<ASYNCUSER0_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCUSER0_A) -> Self {
        variant as _
    }
}
impl ASYNCUSER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASYNCUSER0_A> {
        match self.bits {
            0 => Some(ASYNCUSER0_A::OFF),
            1 => Some(ASYNCUSER0_A::SYNCCH0),
            3 => Some(ASYNCUSER0_A::ASYNCCH0),
            4 => Some(ASYNCUSER0_A::ASYNCCH1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCUSER0_A::OFF
    }
    #[doc = "Checks if the value of the field is `SYNCCH0`"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == ASYNCUSER0_A::SYNCCH0
    }
    #[doc = "Checks if the value of the field is `ASYNCCH0`"]
    #[inline(always)]
    pub fn is_asyncch0(&self) -> bool {
        *self == ASYNCUSER0_A::ASYNCCH0
    }
    #[doc = "Checks if the value of the field is `ASYNCCH1`"]
    #[inline(always)]
    pub fn is_asyncch1(&self) -> bool {
        *self == ASYNCUSER0_A::ASYNCCH1
    }
}
#[doc = "Field `ASYNCUSER0` writer - Asynchronous User Ch 0 Input Selection - TCB0"]
pub type ASYNCUSER0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, ASYNCUSER0_SPEC, u8, ASYNCUSER0_A, 8, O>;
impl<'a, const O: u8> ASYNCUSER0_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ASYNCUSER0_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut W {
        self.variant(ASYNCUSER0_A::SYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn asyncch0(self) -> &'a mut W {
        self.variant(ASYNCUSER0_A::ASYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn asyncch1(self) -> &'a mut W {
        self.variant(ASYNCUSER0_A::ASYNCCH1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous User Ch 0 Input Selection - TCB0"]
    #[inline(always)]
    pub fn asyncuser0(&self) -> ASYNCUSER0_R {
        ASYNCUSER0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous User Ch 0 Input Selection - TCB0"]
    #[inline(always)]
    #[must_use]
    pub fn asyncuser0(&mut self) -> ASYNCUSER0_W<0> {
        ASYNCUSER0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous User Ch 0 Input Selection - TCB0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncuser0](index.html) module"]
pub struct ASYNCUSER0_SPEC;
impl crate::RegisterSpec for ASYNCUSER0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [asyncuser0::R](R) reader structure"]
impl crate::Readable for ASYNCUSER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncuser0::W](W) writer structure"]
impl crate::Writable for ASYNCUSER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCUSER0 to value 0"]
impl crate::Resettable for ASYNCUSER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
