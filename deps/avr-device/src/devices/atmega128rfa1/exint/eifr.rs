#[doc = "Register `EIFR` reader"]
pub struct R(crate::R<EIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIFR` writer"]
pub struct W(crate::W<EIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIFR_SPEC>;
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
impl From<crate::W<EIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTF` reader - External Interrupt Flag"]
pub type INTF_R = crate::FieldReader<u8, INTF_A>;
#[doc = "External Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTF_A {
    #[doc = "0: No edge or logic change on INT7:0 occurred."]
    NO_EDGE_OR_LOGIC_CHANGE_ON_INT7_0_OCCURRED = 0,
    #[doc = "1: A edge or logic change on INT0 occurred and triggered an interrupt request."]
    A_EDGE_OR_LOGIC_CHANGE_ON_INT0_OCCURRED_AND_TRIGGERED_AN_INTERRUPT_REQUEST = 1,
    #[doc = "128: A edge or logic change on INT7 occurred and triggered an interrupt request."]
    A_EDGE_OR_LOGIC_CHANGE_ON_INT7_OCCURRED_AND_TRIGGERED_AN_INTERRUPT_REQUEST = 128,
}
impl From<INTF_A> for u8 {
    #[inline(always)]
    fn from(variant: INTF_A) -> Self {
        variant as _
    }
}
impl INTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INTF_A> {
        match self.bits {
            0 => Some(INTF_A::NO_EDGE_OR_LOGIC_CHANGE_ON_INT7_0_OCCURRED),
            1 => Some(
                INTF_A::A_EDGE_OR_LOGIC_CHANGE_ON_INT0_OCCURRED_AND_TRIGGERED_AN_INTERRUPT_REQUEST,
            ),
            128 => Some(
                INTF_A::A_EDGE_OR_LOGIC_CHANGE_ON_INT7_OCCURRED_AND_TRIGGERED_AN_INTERRUPT_REQUEST,
            ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_OR_LOGIC_CHANGE_ON_INT7_0_OCCURRED`"]
    #[inline(always)]
    pub fn is_no_edge_or_logic_change_on_int7_0_occurred(&self) -> bool {
        *self == INTF_A::NO_EDGE_OR_LOGIC_CHANGE_ON_INT7_0_OCCURRED
    }
    #[doc = "Checks if the value of the field is `A_EDGE_OR_LOGIC_CHANGE_ON_INT0_OCCURRED_AND_TRIGGERED_AN_INTERRUPT_REQUEST`"]
    #[inline(always)]
    pub fn is_a_edge_or_logic_change_on_int0_occurred_and_triggered_an_interrupt_request(
        &self,
    ) -> bool {
        *self == INTF_A::A_EDGE_OR_LOGIC_CHANGE_ON_INT0_OCCURRED_AND_TRIGGERED_AN_INTERRUPT_REQUEST
    }
    #[doc = "Checks if the value of the field is `A_EDGE_OR_LOGIC_CHANGE_ON_INT7_OCCURRED_AND_TRIGGERED_AN_INTERRUPT_REQUEST`"]
    #[inline(always)]
    pub fn is_a_edge_or_logic_change_on_int7_occurred_and_triggered_an_interrupt_request(
        &self,
    ) -> bool {
        *self == INTF_A::A_EDGE_OR_LOGIC_CHANGE_ON_INT7_OCCURRED_AND_TRIGGERED_AN_INTERRUPT_REQUEST
    }
}
#[doc = "Field `INTF` writer - External Interrupt Flag"]
pub type INTF_W<'a, const O: u8> = crate::FieldWriter<'a, u8, EIFR_SPEC, u8, INTF_A, 8, O>;
impl<'a, const O: u8> INTF_W<'a, O> {
    #[doc = "No edge or logic change on INT7:0 occurred."]
    #[inline(always)]
    pub fn no_edge_or_logic_change_on_int7_0_occurred(self) -> &'a mut W {
        self.variant(INTF_A::NO_EDGE_OR_LOGIC_CHANGE_ON_INT7_0_OCCURRED)
    }
    #[doc = "A edge or logic change on INT0 occurred and triggered an interrupt request."]
    #[inline(always)]
    pub fn a_edge_or_logic_change_on_int0_occurred_and_triggered_an_interrupt_request(
        self,
    ) -> &'a mut W {
        self.variant(
            INTF_A::A_EDGE_OR_LOGIC_CHANGE_ON_INT0_OCCURRED_AND_TRIGGERED_AN_INTERRUPT_REQUEST,
        )
    }
    #[doc = "A edge or logic change on INT7 occurred and triggered an interrupt request."]
    #[inline(always)]
    pub fn a_edge_or_logic_change_on_int7_occurred_and_triggered_an_interrupt_request(
        self,
    ) -> &'a mut W {
        self.variant(
            INTF_A::A_EDGE_OR_LOGIC_CHANGE_ON_INT7_OCCURRED_AND_TRIGGERED_AN_INTERRUPT_REQUEST,
        )
    }
}
impl R {
    #[doc = "Bits 0:7 - External Interrupt Flag"]
    #[inline(always)]
    pub fn intf(&self) -> INTF_R {
        INTF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - External Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn intf(&mut self) -> INTF_W<0> {
        INTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eifr](index.html) module"]
pub struct EIFR_SPEC;
impl crate::RegisterSpec for EIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eifr::R](R) reader structure"]
impl crate::Readable for EIFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eifr::W](W) writer structure"]
impl crate::Writable for EIFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIFR to value 0"]
impl crate::Resettable for EIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
