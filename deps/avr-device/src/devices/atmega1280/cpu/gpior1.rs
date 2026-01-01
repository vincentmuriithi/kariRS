#[doc = "Register `GPIOR1` reader"]
pub struct R(crate::R<GPIOR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOR1` writer"]
pub struct W(crate::W<GPIOR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOR1_SPEC>;
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
impl From<crate::W<GPIOR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOR` reader - General Purpose IO Register 1 bis"]
pub type GPIOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIOR` writer - General Purpose IO Register 1 bis"]
pub type GPIOR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, GPIOR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - General Purpose IO Register 1 bis"]
    #[inline(always)]
    pub fn gpior(&self) -> GPIOR_R {
        GPIOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - General Purpose IO Register 1 bis"]
    #[inline(always)]
    #[must_use]
    pub fn gpior(&mut self) -> GPIOR_W<0> {
        GPIOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "General Purpose IO Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpior1](index.html) module"]
pub struct GPIOR1_SPEC;
impl crate::RegisterSpec for GPIOR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gpior1::R](R) reader structure"]
impl crate::Readable for GPIOR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpior1::W](W) writer structure"]
impl crate::Writable for GPIOR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOR1 to value 0"]
impl crate::Resettable for GPIOR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
