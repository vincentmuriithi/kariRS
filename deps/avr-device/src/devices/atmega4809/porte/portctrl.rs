#[doc = "Register `PORTCTRL` reader"]
pub struct R(crate::R<PORTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTCTRL` writer"]
pub struct W(crate::W<PORTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTCTRL_SPEC>;
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
impl From<crate::W<PORTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRL` reader - Slew Rate Limit Enable"]
pub type SRL_R = crate::BitReader<bool>;
#[doc = "Field `SRL` writer - Slew Rate Limit Enable"]
pub type SRL_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Slew Rate Limit Enable"]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slew Rate Limit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srl(&mut self) -> SRL_W<0> {
        SRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portctrl](index.html) module"]
pub struct PORTCTRL_SPEC;
impl crate::RegisterSpec for PORTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [portctrl::R](R) reader structure"]
impl crate::Readable for PORTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portctrl::W](W) writer structure"]
impl crate::Writable for PORTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTCTRL to value 0"]
impl crate::Resettable for PORTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
