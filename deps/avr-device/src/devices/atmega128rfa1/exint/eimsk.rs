#[doc = "Register `EIMSK` reader"]
pub struct R(crate::R<EIMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIMSK` writer"]
pub struct W(crate::W<EIMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIMSK_SPEC>;
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
impl From<crate::W<EIMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - External Interrupt Request Enable"]
pub type INT_R = crate::FieldReader<u8, INT_A>;
#[doc = "External Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INT_A {
    #[doc = "0: All external pin interrupts are disabled."]
    ALL_EXTERNAL_PIN_INTERRUPTS_ARE_DISABLED = 0,
    #[doc = "255: All external pin interrupts are enabled."]
    ALL_EXTERNAL_PIN_INTERRUPTS_ARE_ENABLED = 255,
}
impl From<INT_A> for u8 {
    #[inline(always)]
    fn from(variant: INT_A) -> Self {
        variant as _
    }
}
impl INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INT_A> {
        match self.bits {
            0 => Some(INT_A::ALL_EXTERNAL_PIN_INTERRUPTS_ARE_DISABLED),
            255 => Some(INT_A::ALL_EXTERNAL_PIN_INTERRUPTS_ARE_ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_EXTERNAL_PIN_INTERRUPTS_ARE_DISABLED`"]
    #[inline(always)]
    pub fn is_all_external_pin_interrupts_are_disabled(&self) -> bool {
        *self == INT_A::ALL_EXTERNAL_PIN_INTERRUPTS_ARE_DISABLED
    }
    #[doc = "Checks if the value of the field is `ALL_EXTERNAL_PIN_INTERRUPTS_ARE_ENABLED`"]
    #[inline(always)]
    pub fn is_all_external_pin_interrupts_are_enabled(&self) -> bool {
        *self == INT_A::ALL_EXTERNAL_PIN_INTERRUPTS_ARE_ENABLED
    }
}
#[doc = "Field `INT` writer - External Interrupt Request Enable"]
pub type INT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, EIMSK_SPEC, u8, INT_A, 8, O>;
impl<'a, const O: u8> INT_W<'a, O> {
    #[doc = "All external pin interrupts are disabled."]
    #[inline(always)]
    pub fn all_external_pin_interrupts_are_disabled(self) -> &'a mut W {
        self.variant(INT_A::ALL_EXTERNAL_PIN_INTERRUPTS_ARE_DISABLED)
    }
    #[doc = "All external pin interrupts are enabled."]
    #[inline(always)]
    pub fn all_external_pin_interrupts_are_enabled(self) -> &'a mut W {
        self.variant(INT_A::ALL_EXTERNAL_PIN_INTERRUPTS_ARE_ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:7 - External Interrupt Request Enable"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - External Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<0> {
        INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eimsk](index.html) module"]
pub struct EIMSK_SPEC;
impl crate::RegisterSpec for EIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eimsk::R](R) reader structure"]
impl crate::Readable for EIMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eimsk::W](W) writer structure"]
impl crate::Writable for EIMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIMSK to value 0"]
impl crate::Resettable for EIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
