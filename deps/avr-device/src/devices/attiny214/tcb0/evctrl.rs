#[doc = "Register `EVCTRL` reader"]
pub struct R(crate::R<EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL` writer"]
pub struct W(crate::W<EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRL_SPEC>;
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
impl From<crate::W<EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTEI` reader - Event Input Enable"]
pub type CAPTEI_R = crate::BitReader<bool>;
#[doc = "Field `CAPTEI` writer - Event Input Enable"]
pub type CAPTEI_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, O>;
#[doc = "Field `EDGE` reader - Event Edge"]
pub type EDGE_R = crate::BitReader<bool>;
#[doc = "Field `EDGE` writer - Event Edge"]
pub type EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, O>;
#[doc = "Field `FILTER` reader - Input Capture Noise Cancellation Filter"]
pub type FILTER_R = crate::BitReader<bool>;
#[doc = "Field `FILTER` writer - Input Capture Noise Cancellation Filter"]
pub type FILTER_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Event Input Enable"]
    #[inline(always)]
    pub fn captei(&self) -> CAPTEI_R {
        CAPTEI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Event Edge"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Input Capture Noise Cancellation Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn captei(&mut self) -> CAPTEI_W<0> {
        CAPTEI_W::new(self)
    }
    #[doc = "Bit 4 - Event Edge"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<4> {
        EDGE_W::new(self)
    }
    #[doc = "Bit 6 - Input Capture Noise Cancellation Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<6> {
        FILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [evctrl::R](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrl::W](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
