#[doc = "Register `AMISCR` reader"]
pub struct R(crate::R<AMISCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMISCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMISCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMISCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMISCR` writer"]
pub struct W(crate::W<AMISCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMISCR_SPEC>;
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
impl From<crate::W<AMISCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMISCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISRCEN` reader - Current Source Enable"]
pub type ISRCEN_R = crate::BitReader<bool>;
#[doc = "Field `ISRCEN` writer - Current Source Enable"]
pub type ISRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMISCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Current Source Enable"]
    #[inline(always)]
    pub fn isrcen(&self) -> ISRCEN_R {
        ISRCEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Current Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn isrcen(&mut self) -> ISRCEN_W<0> {
        ISRCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Miscellaneous Control Register (Shared with AD_CONVERTER IO_MODULE)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amiscr](index.html) module"]
pub struct AMISCR_SPEC;
impl crate::RegisterSpec for AMISCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [amiscr::R](R) reader structure"]
impl crate::Readable for AMISCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amiscr::W](W) writer structure"]
impl crate::Writable for AMISCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMISCR to value 0"]
impl crate::Resettable for AMISCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
