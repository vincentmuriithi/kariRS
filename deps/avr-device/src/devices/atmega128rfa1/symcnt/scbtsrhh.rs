#[doc = "Register `SCBTSRHH` reader"]
pub struct R(crate::R<SCBTSRHH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCBTSRHH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCBTSRHH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCBTSRHH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCBTSRHH` writer"]
pub struct W(crate::W<SCBTSRHH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCBTSRHH_SPEC>;
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
impl From<crate::W<SCBTSRHH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCBTSRHH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCBTSRHH` reader - Symbol Counter Beacon Timestamp Register HH-Byte"]
pub type SCBTSRHH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCBTSRHH` writer - Symbol Counter Beacon Timestamp Register HH-Byte"]
pub type SCBTSRHH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCBTSRHH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register HH-Byte"]
    #[inline(always)]
    pub fn scbtsrhh(&self) -> SCBTSRHH_R {
        SCBTSRHH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register HH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scbtsrhh(&mut self) -> SCBTSRHH_W<0> {
        SCBTSRHH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Symbol Counter Beacon Timestamp Register HH-Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scbtsrhh](index.html) module"]
pub struct SCBTSRHH_SPEC;
impl crate::RegisterSpec for SCBTSRHH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scbtsrhh::R](R) reader structure"]
impl crate::Readable for SCBTSRHH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scbtsrhh::W](W) writer structure"]
impl crate::Writable for SCBTSRHH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCBTSRHH to value 0"]
impl crate::Resettable for SCBTSRHH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
