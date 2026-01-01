#[doc = "Register `BODCR` reader"]
pub struct R(crate::R<BODCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BODCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BODCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BODCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BODCR` writer"]
pub struct W(crate::W<BODCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BODCR_SPEC>;
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
impl From<crate::W<BODCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BODCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BODSE` reader - BOD Power-Down Sleep Enable"]
pub type BODSE_R = crate::BitReader<bool>;
#[doc = "Field `BODSE` writer - BOD Power-Down Sleep Enable"]
pub type BODSE_W<'a, const O: u8> = crate::BitWriter<'a, u8, BODCR_SPEC, bool, O>;
#[doc = "Field `BODS` reader - BOD Power-Down in Power-Down Sleep"]
pub type BODS_R = crate::BitReader<bool>;
#[doc = "Field `BODS` writer - BOD Power-Down in Power-Down Sleep"]
pub type BODS_W<'a, const O: u8> = crate::BitWriter<'a, u8, BODCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BOD Power-Down Sleep Enable"]
    #[inline(always)]
    pub fn bodse(&self) -> BODSE_R {
        BODSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD Power-Down in Power-Down Sleep"]
    #[inline(always)]
    pub fn bods(&self) -> BODS_R {
        BODS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOD Power-Down Sleep Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bodse(&mut self) -> BODSE_W<0> {
        BODSE_W::new(self)
    }
    #[doc = "Bit 1 - BOD Power-Down in Power-Down Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn bods(&mut self) -> BODS_W<1> {
        BODS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodcr](index.html) module"]
pub struct BODCR_SPEC;
impl crate::RegisterSpec for BODCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bodcr::R](R) reader structure"]
impl crate::Readable for BODCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bodcr::W](W) writer structure"]
impl crate::Writable for BODCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BODCR to value 0"]
impl crate::Resettable for BODCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
