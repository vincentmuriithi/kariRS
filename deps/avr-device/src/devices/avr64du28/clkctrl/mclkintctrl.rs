#[doc = "Register `MCLKINTCTRL` reader"]
pub struct R(crate::R<MCLKINTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKINTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKINTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKINTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKINTCTRL` writer"]
pub struct W(crate::W<MCLKINTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKINTCTRL_SPEC>;
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
impl From<crate::W<MCLKINTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKINTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFD` reader - Clock Failure Detect Interrupt Enable"]
pub type CFD_R = crate::BitReader<bool>;
#[doc = "Field `CFD` writer - Clock Failure Detect Interrupt Enable"]
pub type CFD_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCLKINTCTRL_SPEC, bool, O>;
#[doc = "Field `INTTYPE` reader - Interrupt type"]
pub type INTTYPE_R = crate::BitReader<INTTYPE_A>;
#[doc = "Interrupt type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTTYPE_A {
    #[doc = "0: Regular Interrupt"]
    INT = 0,
    #[doc = "1: NMI"]
    NMI = 1,
}
impl From<INTTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: INTTYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl INTTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTTYPE_A {
        match self.bits {
            false => INTTYPE_A::INT,
            true => INTTYPE_A::NMI,
        }
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == INTTYPE_A::INT
    }
    #[doc = "Checks if the value of the field is `NMI`"]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        *self == INTTYPE_A::NMI
    }
}
#[doc = "Field `INTTYPE` writer - Interrupt type"]
pub type INTTYPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCLKINTCTRL_SPEC, INTTYPE_A, O>;
impl<'a, const O: u8> INTTYPE_W<'a, O> {
    #[doc = "Regular Interrupt"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(INTTYPE_A::INT)
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut W {
        self.variant(INTTYPE_A::NMI)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Failure Detect Interrupt Enable"]
    #[inline(always)]
    pub fn cfd(&self) -> CFD_R {
        CFD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt type"]
    #[inline(always)]
    pub fn inttype(&self) -> INTTYPE_R {
        INTTYPE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfd(&mut self) -> CFD_W<0> {
        CFD_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt type"]
    #[inline(always)]
    #[must_use]
    pub fn inttype(&mut self) -> INTTYPE_W<7> {
        INTTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkintctrl](index.html) module"]
pub struct MCLKINTCTRL_SPEC;
impl crate::RegisterSpec for MCLKINTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mclkintctrl::R](R) reader structure"]
impl crate::Readable for MCLKINTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkintctrl::W](W) writer structure"]
impl crate::Writable for MCLKINTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKINTCTRL to value 0"]
impl crate::Resettable for MCLKINTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
