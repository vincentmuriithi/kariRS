#[doc = "Register `LINDAT` reader"]
pub struct R(crate::R<LINDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINDAT` writer"]
pub struct W(crate::W<LINDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINDAT_SPEC>;
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
impl From<crate::W<LINDAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDATA` reader - No Description."]
pub type LDATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LDATA` writer - No Description."]
pub type LDATA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LINDAT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - No Description."]
    #[inline(always)]
    pub fn ldata(&self) -> LDATA_R {
        LDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn ldata(&mut self) -> LDATA_W<0> {
        LDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "LIN Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lindat](index.html) module"]
pub struct LINDAT_SPEC;
impl crate::RegisterSpec for LINDAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lindat::R](R) reader structure"]
impl crate::Readable for LINDAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lindat::W](W) writer structure"]
impl crate::Writable for LINDAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINDAT to value 0"]
impl crate::Resettable for LINDAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
