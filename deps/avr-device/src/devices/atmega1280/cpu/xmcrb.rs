#[doc = "Register `XMCRB` reader"]
pub struct R(crate::R<XMCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XMCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XMCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XMCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XMCRB` writer"]
pub struct W(crate::W<XMCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XMCRB_SPEC>;
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
impl From<crate::W<XMCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XMCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XMM` reader - External Memory High Mask"]
pub type XMM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XMM` writer - External Memory High Mask"]
pub type XMM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, XMCRB_SPEC, u8, u8, 3, O>;
#[doc = "Field `XMBK` reader - External Memory Bus Keeper Enable"]
pub type XMBK_R = crate::BitReader<bool>;
#[doc = "Field `XMBK` writer - External Memory Bus Keeper Enable"]
pub type XMBK_W<'a, const O: u8> = crate::BitWriter<'a, u8, XMCRB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - External Memory High Mask"]
    #[inline(always)]
    pub fn xmm(&self) -> XMM_R {
        XMM_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - External Memory Bus Keeper Enable"]
    #[inline(always)]
    pub fn xmbk(&self) -> XMBK_R {
        XMBK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Memory High Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xmm(&mut self) -> XMM_W<0> {
        XMM_W::new(self)
    }
    #[doc = "Bit 7 - External Memory Bus Keeper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xmbk(&mut self) -> XMBK_W<7> {
        XMBK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Memory Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xmcrb](index.html) module"]
pub struct XMCRB_SPEC;
impl crate::RegisterSpec for XMCRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [xmcrb::R](R) reader structure"]
impl crate::Readable for XMCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xmcrb::W](W) writer structure"]
impl crate::Writable for XMCRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XMCRB to value 0"]
impl crate::Resettable for XMCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
