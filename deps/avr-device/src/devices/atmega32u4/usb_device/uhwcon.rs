#[doc = "Register `UHWCON` reader"]
pub struct R(crate::R<UHWCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHWCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHWCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHWCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UHWCON` writer"]
pub struct W(crate::W<UHWCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHWCON_SPEC>;
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
impl From<crate::W<UHWCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHWCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UVREGE` reader - No Description."]
pub type UVREGE_R = crate::BitReader<bool>;
#[doc = "Field `UVREGE` writer - No Description."]
pub type UVREGE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UHWCON_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn uvrege(&self) -> UVREGE_R {
        UVREGE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn uvrege(&mut self) -> UVREGE_W<0> {
        UVREGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhwcon](index.html) module"]
pub struct UHWCON_SPEC;
impl crate::RegisterSpec for UHWCON_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uhwcon::R](R) reader structure"]
impl crate::Readable for UHWCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhwcon::W](W) writer structure"]
impl crate::Writable for UHWCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UHWCON to value 0"]
impl crate::Resettable for UHWCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
