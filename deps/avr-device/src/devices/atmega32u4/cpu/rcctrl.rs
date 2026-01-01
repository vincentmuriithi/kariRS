#[doc = "Register `RCCTRL` reader"]
pub struct R(crate::R<RCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCCTRL` writer"]
pub struct W(crate::W<RCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCCTRL_SPEC>;
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
impl From<crate::W<RCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCFREQ` reader - No Description."]
pub type RCFREQ_R = crate::BitReader<bool>;
#[doc = "Field `RCFREQ` writer - No Description."]
pub type RCFREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn rcfreq(&self) -> RCFREQ_R {
        RCFREQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rcfreq(&mut self) -> RCFREQ_W<0> {
        RCFREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcctrl](index.html) module"]
pub struct RCCTRL_SPEC;
impl crate::RegisterSpec for RCCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rcctrl::R](R) reader structure"]
impl crate::Readable for RCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcctrl::W](W) writer structure"]
impl crate::Writable for RCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCCTRL to value 0"]
impl crate::Resettable for RCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
