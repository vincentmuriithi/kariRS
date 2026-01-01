#[doc = "Register `AES_STATUS` reader"]
pub struct R(crate::R<AES_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_STATUS` writer"]
pub struct W(crate::W<AES_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_STATUS_SPEC>;
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
impl From<crate::W<AES_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_DONE` reader - AES Operation Finished with Success"]
pub type AES_DONE_R = crate::BitReader<bool>;
#[doc = "Field `AES_DONE` writer - AES Operation Finished with Success"]
pub type AES_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u8, AES_STATUS_SPEC, bool, O>;
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, AES_STATUS_SPEC, u8, u8, 6, O>;
#[doc = "Field `AES_ER` reader - AES Operation Finished with Error"]
pub type AES_ER_R = crate::BitReader<bool>;
#[doc = "Field `AES_ER` writer - AES Operation Finished with Error"]
pub type AES_ER_W<'a, const O: u8> = crate::BitWriter<'a, u8, AES_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - AES Operation Finished with Success"]
    #[inline(always)]
    pub fn aes_done(&self) -> AES_DONE_R {
        AES_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 1) & 0x3f)
    }
    #[doc = "Bit 7 - AES Operation Finished with Error"]
    #[inline(always)]
    pub fn aes_er(&self) -> AES_ER_R {
        AES_ER_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Operation Finished with Success"]
    #[inline(always)]
    #[must_use]
    pub fn aes_done(&mut self) -> AES_DONE_W<0> {
        AES_DONE_W::new(self)
    }
    #[doc = "Bits 1:6 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<1> {
        RES_W::new(self)
    }
    #[doc = "Bit 7 - AES Operation Finished with Error"]
    #[inline(always)]
    #[must_use]
    pub fn aes_er(&mut self) -> AES_ER_W<7> {
        AES_ER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_status](index.html) module"]
pub struct AES_STATUS_SPEC;
impl crate::RegisterSpec for AES_STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [aes_status::R](R) reader structure"]
impl crate::Readable for AES_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_status::W](W) writer structure"]
impl crate::Writable for AES_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_STATUS to value 0"]
impl crate::Resettable for AES_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
