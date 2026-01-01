#[doc = "Register `UDINT` reader"]
pub struct R(crate::R<UDINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDINT` writer"]
pub struct W(crate::W<UDINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDINT_SPEC>;
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
impl From<crate::W<UDINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPI` reader - No Description."]
pub type SUSPI_R = crate::BitReader<bool>;
#[doc = "Field `SUSPI` writer - No Description."]
pub type SUSPI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDINT_SPEC, bool, O>;
#[doc = "Field `SOFI` reader - No Description."]
pub type SOFI_R = crate::BitReader<bool>;
#[doc = "Field `SOFI` writer - No Description."]
pub type SOFI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDINT_SPEC, bool, O>;
#[doc = "Field `EORSTI` reader - No Description."]
pub type EORSTI_R = crate::BitReader<bool>;
#[doc = "Field `EORSTI` writer - No Description."]
pub type EORSTI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDINT_SPEC, bool, O>;
#[doc = "Field `WAKEUPI` reader - No Description."]
pub type WAKEUPI_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUPI` writer - No Description."]
pub type WAKEUPI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDINT_SPEC, bool, O>;
#[doc = "Field `EORSMI` reader - No Description."]
pub type EORSMI_R = crate::BitReader<bool>;
#[doc = "Field `EORSMI` writer - No Description."]
pub type EORSMI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDINT_SPEC, bool, O>;
#[doc = "Field `UPRSMI` reader - No Description."]
pub type UPRSMI_R = crate::BitReader<bool>;
#[doc = "Field `UPRSMI` writer - No Description."]
pub type UPRSMI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDINT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn suspi(&self) -> SUSPI_R {
        SUSPI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    pub fn sofi(&self) -> SOFI_R {
        SOFI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn eorsti(&self) -> EORSTI_R {
        EORSTI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    pub fn wakeupi(&self) -> WAKEUPI_R {
        WAKEUPI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn eorsmi(&self) -> EORSMI_R {
        EORSMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    pub fn uprsmi(&self) -> UPRSMI_R {
        UPRSMI_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn suspi(&mut self) -> SUSPI_W<0> {
        SUSPI_W::new(self)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn sofi(&mut self) -> SOFI_W<2> {
        SOFI_W::new(self)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn eorsti(&mut self) -> EORSTI_W<3> {
        EORSTI_W::new(self)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn wakeupi(&mut self) -> WAKEUPI_W<4> {
        WAKEUPI_W::new(self)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn eorsmi(&mut self) -> EORSMI_W<5> {
        EORSMI_W::new(self)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn uprsmi(&mut self) -> UPRSMI_W<6> {
        UPRSMI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udint](index.html) module"]
pub struct UDINT_SPEC;
impl crate::RegisterSpec for UDINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [udint::R](R) reader structure"]
impl crate::Readable for UDINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udint::W](W) writer structure"]
impl crate::Writable for UDINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDINT to value 0"]
impl crate::Resettable for UDINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
