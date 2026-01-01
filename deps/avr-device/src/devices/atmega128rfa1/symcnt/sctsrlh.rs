#[doc = "Register `SCTSRLH` reader"]
pub struct R(crate::R<SCTSRLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTSRLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTSRLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTSRLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTSRLH` writer"]
pub struct W(crate::W<SCTSRLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTSRLH_SPEC>;
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
impl From<crate::W<SCTSRLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTSRLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCTSRLH` reader - Symbol Counter Frame Timestamp Register LH-Byte"]
pub type SCTSRLH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCTSRLH` writer - Symbol Counter Frame Timestamp Register LH-Byte"]
pub type SCTSRLH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCTSRLH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Frame Timestamp Register LH-Byte"]
    #[inline(always)]
    pub fn sctsrlh(&self) -> SCTSRLH_R {
        SCTSRLH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Frame Timestamp Register LH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sctsrlh(&mut self) -> SCTSRLH_W<0> {
        SCTSRLH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Symbol Counter Frame Timestamp Register LH-Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctsrlh](index.html) module"]
pub struct SCTSRLH_SPEC;
impl crate::RegisterSpec for SCTSRLH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sctsrlh::R](R) reader structure"]
impl crate::Readable for SCTSRLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctsrlh::W](W) writer structure"]
impl crate::Writable for SCTSRLH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTSRLH to value 0"]
impl crate::Resettable for SCTSRLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
