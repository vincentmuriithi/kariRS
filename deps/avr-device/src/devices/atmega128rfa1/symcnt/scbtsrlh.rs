#[doc = "Register `SCBTSRLH` reader"]
pub struct R(crate::R<SCBTSRLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCBTSRLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCBTSRLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCBTSRLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCBTSRLH` writer"]
pub struct W(crate::W<SCBTSRLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCBTSRLH_SPEC>;
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
impl From<crate::W<SCBTSRLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCBTSRLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCBTSRLH` reader - Symbol Counter Beacon Timestamp Register LH-Byte"]
pub type SCBTSRLH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCBTSRLH` writer - Symbol Counter Beacon Timestamp Register LH-Byte"]
pub type SCBTSRLH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCBTSRLH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register LH-Byte"]
    #[inline(always)]
    pub fn scbtsrlh(&self) -> SCBTSRLH_R {
        SCBTSRLH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register LH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scbtsrlh(&mut self) -> SCBTSRLH_W<0> {
        SCBTSRLH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Symbol Counter Beacon Timestamp Register LH-Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scbtsrlh](index.html) module"]
pub struct SCBTSRLH_SPEC;
impl crate::RegisterSpec for SCBTSRLH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scbtsrlh::R](R) reader structure"]
impl crate::Readable for SCBTSRLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scbtsrlh::W](W) writer structure"]
impl crate::Writable for SCBTSRLH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCBTSRLH to value 0"]
impl crate::Resettable for SCBTSRLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
