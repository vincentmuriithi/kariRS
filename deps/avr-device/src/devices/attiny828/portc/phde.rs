#[doc = "Register `PHDE` reader"]
pub struct R(crate::R<PHDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHDE` writer"]
pub struct W(crate::W<PHDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHDE_SPEC>;
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
impl From<crate::W<PHDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHDEC` reader - Port C High Drive Enable"]
pub type PHDEC_R = crate::BitReader<bool>;
#[doc = "Field `PHDEC` writer - Port C High Drive Enable"]
pub type PHDEC_W<'a, const O: u8> = crate::BitWriter<'a, u8, PHDE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Port C High Drive Enable"]
    #[inline(always)]
    pub fn phdec(&self) -> PHDEC_R {
        PHDEC_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Port C High Drive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn phdec(&mut self) -> PHDEC_W<2> {
        PHDEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port High Drive Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phde](index.html) module"]
pub struct PHDE_SPEC;
impl crate::RegisterSpec for PHDE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [phde::R](R) reader structure"]
impl crate::Readable for PHDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phde::W](W) writer structure"]
impl crate::Writable for PHDE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHDE to value 0"]
impl crate::Resettable for PHDE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
