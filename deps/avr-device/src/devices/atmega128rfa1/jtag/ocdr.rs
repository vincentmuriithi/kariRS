#[doc = "Register `OCDR` reader"]
pub struct R(crate::R<OCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCDR` writer"]
pub struct W(crate::W<OCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCDR_SPEC>;
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
impl From<crate::W<OCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCDR` reader - On-Chip Debug Register Data"]
pub type OCDR_R = crate::FieldReader<u8, OCDR_A>;
#[doc = "On-Chip Debug Register Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OCDR_A {
    #[doc = "0: Refer to the debugger documentation for further information on how to use this register."]
    REFER_TO_THE_DEBUGGER_DOCUMENTATION_FOR_FURTHER_INFORMATION_ON_HOW_TO_USE_THIS_REGISTER = 0,
}
impl From<OCDR_A> for u8 {
    #[inline(always)]
    fn from(variant: OCDR_A) -> Self {
        variant as _
    }
}
impl OCDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OCDR_A> {
        match self . bits { 0 => Some ( OCDR_A :: REFER_TO_THE_DEBUGGER_DOCUMENTATION_FOR_FURTHER_INFORMATION_ON_HOW_TO_USE_THIS_REGISTER ) , _ => None , }
    }
    #[doc = "Checks if the value of the field is `REFER_TO_THE_DEBUGGER_DOCUMENTATION_FOR_FURTHER_INFORMATION_ON_HOW_TO_USE_THIS_REGISTER`"]
    #[inline(always)]
    pub fn is_refer_to_the_debugger_documentation_for_further_information_on_how_to_use_this_register(
        &self,
    ) -> bool {
        * self == OCDR_A :: REFER_TO_THE_DEBUGGER_DOCUMENTATION_FOR_FURTHER_INFORMATION_ON_HOW_TO_USE_THIS_REGISTER
    }
}
#[doc = "Field `OCDR` writer - On-Chip Debug Register Data"]
pub type OCDR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, OCDR_SPEC, u8, OCDR_A, 8, O>;
impl<'a, const O: u8> OCDR_W<'a, O> {
    #[doc = "Refer to the debugger documentation for further information on how to use this register."]
    #[inline(always)]
    pub fn refer_to_the_debugger_documentation_for_further_information_on_how_to_use_this_register(
        self,
    ) -> &'a mut W {
        self . variant ( OCDR_A :: REFER_TO_THE_DEBUGGER_DOCUMENTATION_FOR_FURTHER_INFORMATION_ON_HOW_TO_USE_THIS_REGISTER )
    }
}
impl R {
    #[doc = "Bits 0:7 - On-Chip Debug Register Data"]
    #[inline(always)]
    pub fn ocdr(&self) -> OCDR_R {
        OCDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - On-Chip Debug Register Data"]
    #[inline(always)]
    #[must_use]
    pub fn ocdr(&mut self) -> OCDR_W<0> {
        OCDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "On-Chip Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocdr](index.html) module"]
pub struct OCDR_SPEC;
impl crate::RegisterSpec for OCDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ocdr::R](R) reader structure"]
impl crate::Readable for OCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocdr::W](W) writer structure"]
impl crate::Writable for OCDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCDR to value 0"]
impl crate::Resettable for OCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
