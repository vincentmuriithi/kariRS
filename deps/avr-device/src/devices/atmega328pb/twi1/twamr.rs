#[doc = "Register `TWAMR` reader"]
pub struct R(crate::R<TWAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWAMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWAMR` writer"]
pub struct W(crate::W<TWAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWAMR_SPEC>;
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
impl From<crate::W<TWAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWAMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWAM1` reader - No Description."]
pub type TWAM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWAM1` writer - No Description."]
pub type TWAM1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TWAMR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 1:7 - No Description."]
    #[inline(always)]
    pub fn twam1(&self) -> TWAM1_R {
        TWAM1_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bits 1:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn twam1(&mut self) -> TWAM1_W<1> {
        TWAM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI (Slave) Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twamr](index.html) module"]
pub struct TWAMR_SPEC;
impl crate::RegisterSpec for TWAMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twamr::R](R) reader structure"]
impl crate::Readable for TWAMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twamr::W](W) writer structure"]
impl crate::Writable for TWAMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWAMR to value 0"]
impl crate::Resettable for TWAMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
