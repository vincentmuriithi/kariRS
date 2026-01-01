#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLC_SPEC>;
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
impl From<crate::W<CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UROWWP` reader - User Row Write Protect"]
pub type UROWWP_R = crate::BitReader<bool>;
#[doc = "Field `UROWWP` writer - User Row Write Protect"]
pub type UROWWP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, bool, O>;
#[doc = "Field `BOOTROWWP` reader - Boot Row Write Protect"]
pub type BOOTROWWP_R = crate::BitReader<bool>;
#[doc = "Field `BOOTROWWP` writer - Boot Row Write Protect"]
pub type BOOTROWWP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - User Row Write Protect"]
    #[inline(always)]
    pub fn urowwp(&self) -> UROWWP_R {
        UROWWP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boot Row Write Protect"]
    #[inline(always)]
    pub fn bootrowwp(&self) -> BOOTROWWP_R {
        BOOTROWWP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User Row Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn urowwp(&mut self) -> UROWWP_W<0> {
        UROWWP_W::new(self)
    }
    #[doc = "Bit 1 - Boot Row Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn bootrowwp(&mut self) -> BOOTROWWP_W<1> {
        BOOTROWWP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](index.html) module"]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlc::R](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
