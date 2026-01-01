#[doc = "Register `LINBRRH` reader"]
pub struct R(crate::R<LINBRRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINBRRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINBRRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINBRRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINBRRH` writer"]
pub struct W(crate::W<LINBRRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINBRRH_SPEC>;
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
impl From<crate::W<LINBRRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINBRRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDIV` reader - No Description."]
pub type LDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LDIV` writer - No Description."]
pub type LDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LINBRRH_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    pub fn ldiv(&self) -> LDIV_R {
        LDIV_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn ldiv(&mut self) -> LDIV_W<0> {
        LDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LIN Baud Rate High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linbrrh](index.html) module"]
pub struct LINBRRH_SPEC;
impl crate::RegisterSpec for LINBRRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [linbrrh::R](R) reader structure"]
impl crate::Readable for LINBRRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [linbrrh::W](W) writer structure"]
impl crate::Writable for LINBRRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINBRRH to value 0"]
impl crate::Resettable for LINBRRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
