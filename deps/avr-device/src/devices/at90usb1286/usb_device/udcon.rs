#[doc = "Register `UDCON` reader"]
pub struct R(crate::R<UDCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDCON` writer"]
pub struct W(crate::W<UDCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDCON_SPEC>;
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
impl From<crate::W<UDCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DETACH` reader - No Description."]
pub type DETACH_R = crate::BitReader<bool>;
#[doc = "Field `DETACH` writer - No Description."]
pub type DETACH_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDCON_SPEC, bool, O>;
#[doc = "Field `RMWKUP` reader - No Description."]
pub type RMWKUP_R = crate::BitReader<bool>;
#[doc = "Field `RMWKUP` writer - No Description."]
pub type RMWKUP_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDCON_SPEC, bool, O>;
#[doc = "Field `LSM` reader - No Description."]
pub type LSM_R = crate::BitReader<bool>;
#[doc = "Field `LSM` writer - No Description."]
pub type LSM_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDCON_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn detach(&self) -> DETACH_R {
        DETACH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn rmwkup(&self) -> RMWKUP_R {
        RMWKUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn detach(&mut self) -> DETACH_W<0> {
        DETACH_W::new(self)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rmwkup(&mut self) -> RMWKUP_W<1> {
        RMWKUP_W::new(self)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<2> {
        LSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udcon](index.html) module"]
pub struct UDCON_SPEC;
impl crate::RegisterSpec for UDCON_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [udcon::R](R) reader structure"]
impl crate::Readable for UDCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udcon::W](W) writer structure"]
impl crate::Writable for UDCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDCON to value 0"]
impl crate::Resettable for UDCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
