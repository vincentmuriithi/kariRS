#[doc = "Register `PORTE` reader"]
pub struct R(crate::R<PORTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTE` writer"]
pub struct W(crate::W<PORTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTE_SPEC>;
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
impl From<crate::W<PORTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTE_SPEC>) -> Self {
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
#[doc = "Port E Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [porte](index.html) module"]
pub struct PORTE_SPEC;
impl crate::RegisterSpec for PORTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [porte::R](R) reader structure"]
impl crate::Readable for PORTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [porte::W](W) writer structure"]
impl crate::Writable for PORTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTE to value 0"]
impl crate::Resettable for PORTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
