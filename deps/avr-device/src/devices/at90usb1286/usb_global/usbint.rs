#[doc = "Register `USBINT` reader"]
pub struct R(crate::R<USBINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBINT` writer"]
pub struct W(crate::W<USBINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBINT_SPEC>;
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
impl From<crate::W<USBINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUSTI` reader - No Description."]
pub type VBUSTI_R = crate::BitReader<bool>;
#[doc = "Field `VBUSTI` writer - No Description."]
pub type VBUSTI_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBINT_SPEC, bool, O>;
#[doc = "Field `IDTI` reader - No Description."]
pub type IDTI_R = crate::BitReader<bool>;
#[doc = "Field `IDTI` writer - No Description."]
pub type IDTI_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBINT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn vbusti(&self) -> VBUSTI_R {
        VBUSTI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn idti(&self) -> IDTI_R {
        IDTI_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn vbusti(&mut self) -> VBUSTI_W<0> {
        VBUSTI_W::new(self)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn idti(&mut self) -> IDTI_W<1> {
        IDTI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbint](index.html) module"]
pub struct USBINT_SPEC;
impl crate::RegisterSpec for USBINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbint::R](R) reader structure"]
impl crate::Readable for USBINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbint::W](W) writer structure"]
impl crate::Writable for USBINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBINT to value 0"]
impl crate::Resettable for USBINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
