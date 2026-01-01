#[doc = "Register `USERROW52` reader"]
pub struct R(crate::R<USERROW52_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USERROW52_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USERROW52_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USERROW52_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USERROW52` writer"]
pub struct W(crate::W<USERROW52_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USERROW52_SPEC>;
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
impl From<crate::W<USERROW52_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USERROW52_SPEC>) -> Self {
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
#[doc = "User Row Byte 52\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userrow52](index.html) module"]
pub struct USERROW52_SPEC;
impl crate::RegisterSpec for USERROW52_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [userrow52::R](R) reader structure"]
impl crate::Readable for USERROW52_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [userrow52::W](W) writer structure"]
impl crate::Writable for USERROW52_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USERROW52 to value 0"]
impl crate::Resettable for USERROW52_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
