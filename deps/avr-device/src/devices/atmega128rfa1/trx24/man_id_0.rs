#[doc = "Register `MAN_ID_0` reader"]
pub struct R(crate::R<MAN_ID_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAN_ID_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAN_ID_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAN_ID_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAN_ID_0` writer"]
pub struct W(crate::W<MAN_ID_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAN_ID_0_SPEC>;
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
impl From<crate::W<MAN_ID_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAN_ID_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAN_ID_0` reader - Manufacturer ID (Low Byte)"]
pub type MAN_ID_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAN_ID_0` writer - Manufacturer ID (Low Byte)"]
pub type MAN_ID_0_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MAN_ID_0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Manufacturer ID (Low Byte)"]
    #[inline(always)]
    pub fn man_id_0(&self) -> MAN_ID_0_R {
        MAN_ID_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Manufacturer ID (Low Byte)"]
    #[inline(always)]
    #[must_use]
    pub fn man_id_0(&mut self) -> MAN_ID_0_W<0> {
        MAN_ID_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Identification Register (Manufacture ID Low Byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man_id_0](index.html) module"]
pub struct MAN_ID_0_SPEC;
impl crate::RegisterSpec for MAN_ID_0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [man_id_0::R](R) reader structure"]
impl crate::Readable for MAN_ID_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [man_id_0::W](W) writer structure"]
impl crate::Writable for MAN_ID_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAN_ID_0 to value 0"]
impl crate::Resettable for MAN_ID_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
