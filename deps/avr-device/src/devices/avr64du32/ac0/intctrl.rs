#[doc = "Register `INTCTRL` reader"]
pub struct R(crate::R<INTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCTRL` writer"]
pub struct W(crate::W<INTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCTRL_SPEC>;
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
impl From<crate::W<INTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP` reader - Interrupt Enable"]
pub type CMP_R = crate::BitReader<bool>;
#[doc = "Field `CMP` writer - Interrupt Enable"]
pub type CMP_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `INTMODE` reader - Interrupt Mode"]
pub type INTMODE_R = crate::FieldReader<u8, INTMODE_A>;
#[doc = "Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTMODE_A {
    #[doc = "0: Positive and negative inputs crosses"]
    BOTHEDGE = 0,
    #[doc = "2: Positive input goes below negative input"]
    NEGEDGE = 2,
    #[doc = "3: Positive input goes above negative input"]
    POSEDGE = 3,
}
impl From<INTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE_A) -> Self {
        variant as _
    }
}
impl INTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INTMODE_A> {
        match self.bits {
            0 => Some(INTMODE_A::BOTHEDGE),
            2 => Some(INTMODE_A::NEGEDGE),
            3 => Some(INTMODE_A::POSEDGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE`"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == INTMODE_A::BOTHEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == INTMODE_A::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == INTMODE_A::POSEDGE
    }
}
#[doc = "Field `INTMODE` writer - Interrupt Mode"]
pub type INTMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, INTCTRL_SPEC, u8, INTMODE_A, 2, O>;
impl<'a, const O: u8> INTMODE_W<'a, O> {
    #[doc = "Positive and negative inputs crosses"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut W {
        self.variant(INTMODE_A::BOTHEDGE)
    }
    #[doc = "Positive input goes below negative input"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(INTMODE_A::NEGEDGE)
    }
    #[doc = "Positive input goes above negative input"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(INTMODE_A::POSEDGE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Interrupt Mode"]
    #[inline(always)]
    pub fn intmode(&self) -> INTMODE_R {
        INTMODE_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<0> {
        CMP_W::new(self)
    }
    #[doc = "Bits 4:5 - Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn intmode(&mut self) -> INTMODE_W<4> {
        INTMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intctrl](index.html) module"]
pub struct INTCTRL_SPEC;
impl crate::RegisterSpec for INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intctrl::R](R) reader structure"]
impl crate::Readable for INTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intctrl::W](W) writer structure"]
impl crate::Writable for INTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::Resettable for INTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
