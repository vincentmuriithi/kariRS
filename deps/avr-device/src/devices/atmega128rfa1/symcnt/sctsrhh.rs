#[doc = "Register `SCTSRHH` reader"]
pub struct R(crate::R<SCTSRHH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTSRHH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTSRHH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTSRHH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTSRHH` writer"]
pub struct W(crate::W<SCTSRHH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTSRHH_SPEC>;
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
impl From<crate::W<SCTSRHH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTSRHH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCTSRHH` reader - Symbol Counter Frame Timestamp Register HH-Byte"]
pub type SCTSRHH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCTSRHH` writer - Symbol Counter Frame Timestamp Register HH-Byte"]
pub type SCTSRHH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCTSRHH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Frame Timestamp Register HH-Byte"]
    #[inline(always)]
    pub fn sctsrhh(&self) -> SCTSRHH_R {
        SCTSRHH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Frame Timestamp Register HH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sctsrhh(&mut self) -> SCTSRHH_W<0> {
        SCTSRHH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Symbol Counter Frame Timestamp Register HH-Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctsrhh](index.html) module"]
pub struct SCTSRHH_SPEC;
impl crate::RegisterSpec for SCTSRHH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sctsrhh::R](R) reader structure"]
impl crate::Readable for SCTSRHH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctsrhh::W](W) writer structure"]
impl crate::Writable for SCTSRHH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTSRHH to value 0"]
impl crate::Resettable for SCTSRHH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
