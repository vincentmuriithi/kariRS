#[doc = "Register `UECONX` reader"]
pub struct R(crate::R<UECONX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UECONX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UECONX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UECONX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UECONX` writer"]
pub struct W(crate::W<UECONX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UECONX_SPEC>;
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
impl From<crate::W<UECONX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UECONX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPEN` reader - No Description."]
pub type EPEN_R = crate::BitReader<bool>;
#[doc = "Field `EPEN` writer - No Description."]
pub type EPEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, UECONX_SPEC, bool, O>;
#[doc = "Field `RSTDT` reader - No Description."]
pub type RSTDT_R = crate::BitReader<bool>;
#[doc = "Field `RSTDT` writer - No Description."]
pub type RSTDT_W<'a, const O: u8> = crate::BitWriter<'a, u8, UECONX_SPEC, bool, O>;
#[doc = "Field `STALLRQC` reader - No Description."]
pub type STALLRQC_R = crate::BitReader<bool>;
#[doc = "Field `STALLRQC` writer - No Description."]
pub type STALLRQC_W<'a, const O: u8> = crate::BitWriter<'a, u8, UECONX_SPEC, bool, O>;
#[doc = "Field `STALLRQ` reader - No Description."]
pub type STALLRQ_R = crate::BitReader<bool>;
#[doc = "Field `STALLRQ` writer - No Description."]
pub type STALLRQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, UECONX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn epen(&self) -> EPEN_R {
        EPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn rstdt(&self) -> RSTDT_R {
        RSTDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    pub fn stallrqc(&self) -> STALLRQC_R {
        STALLRQC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn stallrq(&self) -> STALLRQ_R {
        STALLRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn epen(&mut self) -> EPEN_W<0> {
        EPEN_W::new(self)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rstdt(&mut self) -> RSTDT_W<3> {
        RSTDT_W::new(self)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn stallrqc(&mut self) -> STALLRQC_W<4> {
        STALLRQC_W::new(self)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn stallrq(&mut self) -> STALLRQ_W<5> {
        STALLRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ueconx](index.html) module"]
pub struct UECONX_SPEC;
impl crate::RegisterSpec for UECONX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ueconx::R](R) reader structure"]
impl crate::Readable for UECONX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ueconx::W](W) writer structure"]
impl crate::Writable for UECONX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UECONX to value 0"]
impl crate::Resettable for UECONX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
