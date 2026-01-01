#[doc = "Register `TWBR` reader"]
pub struct R(crate::R<TWBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWBR` writer"]
pub struct W(crate::W<TWBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWBR_SPEC>;
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
impl From<crate::W<TWBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWBR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "TWI Bit Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twbr](index.html) module"]
pub struct TWBR_SPEC;
impl crate::RegisterSpec for TWBR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twbr::R](R) reader structure"]
impl crate::Readable for TWBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twbr::W](W) writer structure"]
impl crate::Writable for TWBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWBR to value 0"]
impl crate::Resettable for TWBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
