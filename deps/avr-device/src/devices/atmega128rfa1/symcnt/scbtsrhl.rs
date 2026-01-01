#[doc = "Register `SCBTSRHL` reader"]
pub struct R(crate::R<SCBTSRHL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCBTSRHL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCBTSRHL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCBTSRHL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCBTSRHL` writer"]
pub struct W(crate::W<SCBTSRHL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCBTSRHL_SPEC>;
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
impl From<crate::W<SCBTSRHL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCBTSRHL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCBTSRHL` reader - Symbol Counter Beacon Timestamp Register HL-Byte"]
pub type SCBTSRHL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCBTSRHL` writer - Symbol Counter Beacon Timestamp Register HL-Byte"]
pub type SCBTSRHL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCBTSRHL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register HL-Byte"]
    #[inline(always)]
    pub fn scbtsrhl(&self) -> SCBTSRHL_R {
        SCBTSRHL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register HL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scbtsrhl(&mut self) -> SCBTSRHL_W<0> {
        SCBTSRHL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Symbol Counter Beacon Timestamp Register HL-Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scbtsrhl](index.html) module"]
pub struct SCBTSRHL_SPEC;
impl crate::RegisterSpec for SCBTSRHL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scbtsrhl::R](R) reader structure"]
impl crate::Readable for SCBTSRHL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scbtsrhl::W](W) writer structure"]
impl crate::Writable for SCBTSRHL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCBTSRHL to value 0"]
impl crate::Resettable for SCBTSRHL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
