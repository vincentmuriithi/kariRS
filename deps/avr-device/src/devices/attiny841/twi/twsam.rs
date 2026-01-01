#[doc = "Register `TWSAM` reader"]
pub struct R(crate::R<TWSAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWSAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWSAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWSAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWSAM` writer"]
pub struct W(crate::W<TWSAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWSAM_SPEC>;
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
impl From<crate::W<TWSAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWSAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWAE` reader - TWI Address Enable"]
pub type TWAE_R = crate::BitReader<bool>;
#[doc = "Field `TWAE` writer - TWI Address Enable"]
pub type TWAE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWSAM_SPEC, bool, O>;
#[doc = "Field `TWSAM` reader - TWI Address Mask Bits"]
pub type TWSAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWSAM` writer - TWI Address Mask Bits"]
pub type TWSAM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TWSAM_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - TWI Address Enable"]
    #[inline(always)]
    pub fn twae(&self) -> TWAE_R {
        TWAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - TWI Address Mask Bits"]
    #[inline(always)]
    pub fn twsam(&self) -> TWSAM_R {
        TWSAM_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - TWI Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twae(&mut self) -> TWAE_W<0> {
        TWAE_W::new(self)
    }
    #[doc = "Bits 1:7 - TWI Address Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn twsam(&mut self) -> TWSAM_W<1> {
        TWSAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Slave Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twsam](index.html) module"]
pub struct TWSAM_SPEC;
impl crate::RegisterSpec for TWSAM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twsam::R](R) reader structure"]
impl crate::Readable for TWSAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twsam::W](W) writer structure"]
impl crate::Writable for TWSAM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWSAM to value 0"]
impl crate::Resettable for TWSAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
