#[doc = "Register `PITINTCTRL` reader"]
pub struct R(crate::R<PITINTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PITINTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PITINTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PITINTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PITINTCTRL` writer"]
pub struct W(crate::W<PITINTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PITINTCTRL_SPEC>;
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
impl From<crate::W<PITINTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PITINTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PI` reader - Periodic Interrupt"]
pub type PI_R = crate::BitReader<bool>;
#[doc = "Field `PI` writer - Periodic Interrupt"]
pub type PI_W<'a, const O: u8> = crate::BitWriter<'a, u8, PITINTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Periodic Interrupt"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PI_W<0> {
        PI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIT Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pitintctrl](index.html) module"]
pub struct PITINTCTRL_SPEC;
impl crate::RegisterSpec for PITINTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pitintctrl::R](R) reader structure"]
impl crate::Readable for PITINTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pitintctrl::W](W) writer structure"]
impl crate::Writable for PITINTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PITINTCTRL to value 0"]
impl crate::Resettable for PITINTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
