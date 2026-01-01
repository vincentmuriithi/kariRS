#[doc = "Register `UESTA0X` reader"]
pub struct R(crate::R<UESTA0X_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UESTA0X_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UESTA0X_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UESTA0X_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UESTA0X` writer"]
pub struct W(crate::W<UESTA0X_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UESTA0X_SPEC>;
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
impl From<crate::W<UESTA0X_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UESTA0X_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBUSYBK` reader - No Description."]
pub type NBUSYBK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBUSYBK` writer - No Description."]
pub type NBUSYBK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UESTA0X_SPEC, u8, u8, 2, O>;
#[doc = "Field `DTSEQ` reader - No Description."]
pub type DTSEQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTSEQ` writer - No Description."]
pub type DTSEQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UESTA0X_SPEC, u8, u8, 2, O>;
#[doc = "Field `UNDERFI` reader - No Description."]
pub type UNDERFI_R = crate::BitReader<bool>;
#[doc = "Field `UNDERFI` writer - No Description."]
pub type UNDERFI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UESTA0X_SPEC, bool, O>;
#[doc = "Field `OVERFI` reader - No Description."]
pub type OVERFI_R = crate::BitReader<bool>;
#[doc = "Field `OVERFI` writer - No Description."]
pub type OVERFI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UESTA0X_SPEC, bool, O>;
#[doc = "Field `CFGOK` reader - No Description."]
pub type CFGOK_R = crate::BitReader<bool>;
#[doc = "Field `CFGOK` writer - No Description."]
pub type CFGOK_W<'a, const O: u8> = crate::BitWriter<'a, u8, UESTA0X_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - No Description."]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - No Description."]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn underfi(&self) -> UNDERFI_R {
        UNDERFI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    pub fn overfi(&self) -> OVERFI_R {
        OVERFI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    pub fn cfgok(&self) -> CFGOK_R {
        CFGOK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn nbusybk(&mut self) -> NBUSYBK_W<0> {
        NBUSYBK_W::new(self)
    }
    #[doc = "Bits 2:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn dtseq(&mut self) -> DTSEQ_W<2> {
        DTSEQ_W::new(self)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn underfi(&mut self) -> UNDERFI_W<5> {
        UNDERFI_W::new(self)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn overfi(&mut self) -> OVERFI_W<6> {
        OVERFI_W::new(self)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn cfgok(&mut self) -> CFGOK_W<7> {
        CFGOK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta0x](index.html) module"]
pub struct UESTA0X_SPEC;
impl crate::RegisterSpec for UESTA0X_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uesta0x::R](R) reader structure"]
impl crate::Readable for UESTA0X_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uesta0x::W](W) writer structure"]
impl crate::Writable for UESTA0X_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UESTA0X to value 0"]
impl crate::Resettable for UESTA0X_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
