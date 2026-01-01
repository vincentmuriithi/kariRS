#[doc = "Register `USERROW13` reader"]
pub struct R(crate::R<USERROW13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USERROW13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USERROW13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USERROW13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USERROW13` writer"]
pub struct W(crate::W<USERROW13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USERROW13_SPEC>;
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
impl From<crate::W<USERROW13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USERROW13_SPEC>) -> Self {
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
#[doc = "User Row Byte 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userrow13](index.html) module"]
pub struct USERROW13_SPEC;
impl crate::RegisterSpec for USERROW13_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [userrow13::R](R) reader structure"]
impl crate::Readable for USERROW13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [userrow13::W](W) writer structure"]
impl crate::Writable for USERROW13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USERROW13 to value 0"]
impl crate::Resettable for USERROW13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
