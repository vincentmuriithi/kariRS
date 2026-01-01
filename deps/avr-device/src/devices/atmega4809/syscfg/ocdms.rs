#[doc = "Register `OCDMS` reader"]
pub struct R(crate::R<OCDMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCDMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCDMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCDMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCDMS` writer"]
pub struct W(crate::W<OCDMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCDMS_SPEC>;
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
impl From<crate::W<OCDMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCDMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCDMR` reader - OCD Message Read"]
pub type OCDMR_R = crate::BitReader<bool>;
#[doc = "Field `OCDMR` writer - OCD Message Read"]
pub type OCDMR_W<'a, const O: u8> = crate::BitWriter<'a, u8, OCDMS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OCD Message Read"]
    #[inline(always)]
    pub fn ocdmr(&self) -> OCDMR_R {
        OCDMR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OCD Message Read"]
    #[inline(always)]
    #[must_use]
    pub fn ocdmr(&mut self) -> OCDMR_W<0> {
        OCDMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCD Message Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocdms](index.html) module"]
pub struct OCDMS_SPEC;
impl crate::RegisterSpec for OCDMS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ocdms::R](R) reader structure"]
impl crate::Readable for OCDMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocdms::W](W) writer structure"]
impl crate::Writable for OCDMS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCDMS to value 0"]
impl crate::Resettable for OCDMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
