#[doc = "Register `ETIFR` reader"]
pub struct R(crate::R<ETIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETIFR` writer"]
pub struct W(crate::W<ETIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETIFR_SPEC>;
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
impl From<crate::W<ETIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCF1C` reader - Timer/Counter 1, Output Compare C Match Flag"]
pub type OCF1C_R = crate::BitReader<bool>;
#[doc = "Field `OCF1C` writer - Timer/Counter 1, Output Compare C Match Flag"]
pub type OCF1C_W<'a, const O: u8> = crate::BitWriter<'a, u8, ETIFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter 1, Output Compare C Match Flag"]
    #[inline(always)]
    pub fn ocf1c(&self) -> OCF1C_R {
        OCF1C_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter 1, Output Compare C Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf1c(&mut self) -> OCF1C_W<0> {
        OCF1C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Timer/Counter Interrupt Flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etifr](index.html) module"]
pub struct ETIFR_SPEC;
impl crate::RegisterSpec for ETIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [etifr::R](R) reader structure"]
impl crate::Readable for ETIFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etifr::W](W) writer structure"]
impl crate::Writable for ETIFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETIFR to value 0"]
impl crate::Resettable for ETIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
