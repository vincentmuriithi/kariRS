#[doc = "Register `DBGCTRL` reader"]
pub struct R(crate::R<DBGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGCTRL` writer"]
pub struct W(crate::W<DBGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGCTRL_SPEC>;
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
impl From<crate::W<DBGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGRUN` reader - Debug Run"]
pub type DBGRUN_R = crate::BitReader<DBGRUN_A>;
#[doc = "Debug Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGRUN_A {
    #[doc = "0: The peripheral is halted in Break Debug mode and ignores events"]
    HALT = 0,
    #[doc = "1: The peripheral will continue to run in Break Debug mode when the CPU is halted"]
    RUN = 1,
}
impl From<DBGRUN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGRUN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGRUN_A {
        match self.bits {
            false => DBGRUN_A::HALT,
            true => DBGRUN_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == DBGRUN_A::HALT
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == DBGRUN_A::RUN
    }
}
#[doc = "Field `DBGRUN` writer - Debug Run"]
pub type DBGRUN_W<'a, const O: u8> = crate::BitWriter<'a, u8, DBGCTRL_SPEC, DBGRUN_A, O>;
impl<'a, const O: u8> DBGRUN_W<'a, O> {
    #[doc = "The peripheral is halted in Break Debug mode and ignores events"]
    #[inline(always)]
    pub fn halt(self) -> &'a mut W {
        self.variant(DBGRUN_A::HALT)
    }
    #[doc = "The peripheral will continue to run in Break Debug mode when the CPU is halted"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(DBGRUN_A::RUN)
    }
}
impl R {
    #[doc = "Bit 0 - Debug Run"]
    #[inline(always)]
    pub fn dbgrun(&self) -> DBGRUN_R {
        DBGRUN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Run"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrun(&mut self) -> DBGRUN_W<0> {
        DBGRUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgctrl](index.html) module"]
pub struct DBGCTRL_SPEC;
impl crate::RegisterSpec for DBGCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dbgctrl::R](R) reader structure"]
impl crate::Readable for DBGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgctrl::W](W) writer structure"]
impl crate::Writable for DBGCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGCTRL to value 0"]
impl crate::Resettable for DBGCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
