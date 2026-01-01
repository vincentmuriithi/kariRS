#[doc = "Register `TWDR` reader"]
pub struct R(crate::R<TWDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWDR` writer"]
pub struct W(crate::W<TWDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWDR_SPEC>;
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
impl From<crate::W<TWDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWDR_SPEC>) -> Self {
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
#[doc = "TWI Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twdr](index.html) module"]
pub struct TWDR_SPEC;
impl crate::RegisterSpec for TWDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twdr::R](R) reader structure"]
impl crate::Readable for TWDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twdr::W](W) writer structure"]
impl crate::Writable for TWDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWDR to value 0"]
impl crate::Resettable for TWDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
