#[doc = "Register `PAN_ID_1` reader"]
pub struct R(crate::R<PAN_ID_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAN_ID_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAN_ID_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAN_ID_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAN_ID_1` writer"]
pub struct W(crate::W<PAN_ID_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAN_ID_1_SPEC>;
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
impl From<crate::W<PAN_ID_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAN_ID_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAN_ID_` reader - MAC Personal Area Network ID"]
pub type PAN_ID__R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAN_ID_` writer - MAC Personal Area Network ID"]
pub type PAN_ID__W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PAN_ID_1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - MAC Personal Area Network ID"]
    #[inline(always)]
    pub fn pan_id_(&self) -> PAN_ID__R {
        PAN_ID__R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - MAC Personal Area Network ID"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id_(&mut self) -> PAN_ID__W<0> {
        PAN_ID__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Transceiver Personal Area Network ID Register (High Byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pan_id_1](index.html) module"]
pub struct PAN_ID_1_SPEC;
impl crate::RegisterSpec for PAN_ID_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pan_id_1::R](R) reader structure"]
impl crate::Readable for PAN_ID_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pan_id_1::W](W) writer structure"]
impl crate::Writable for PAN_ID_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAN_ID_1 to value 0"]
impl crate::Resettable for PAN_ID_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
