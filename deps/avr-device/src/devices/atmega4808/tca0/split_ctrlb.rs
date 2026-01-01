#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<SPLIT_CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPLIT_CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPLIT_CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPLIT_CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<SPLIT_CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPLIT_CTRLB_SPEC>;
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
impl From<crate::W<SPLIT_CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPLIT_CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCMP0EN` reader - Low Compare 0 Enable"]
pub type LCMP0EN_R = crate::BitReader<bool>;
#[doc = "Field `LCMP0EN` writer - Low Compare 0 Enable"]
pub type LCMP0EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLB_SPEC, bool, O>;
#[doc = "Field `LCMP1EN` reader - Low Compare 1 Enable"]
pub type LCMP1EN_R = crate::BitReader<bool>;
#[doc = "Field `LCMP1EN` writer - Low Compare 1 Enable"]
pub type LCMP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLB_SPEC, bool, O>;
#[doc = "Field `LCMP2EN` reader - Low Compare 2 Enable"]
pub type LCMP2EN_R = crate::BitReader<bool>;
#[doc = "Field `LCMP2EN` writer - Low Compare 2 Enable"]
pub type LCMP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLB_SPEC, bool, O>;
#[doc = "Field `HCMP0EN` reader - High Compare 0 Enable"]
pub type HCMP0EN_R = crate::BitReader<bool>;
#[doc = "Field `HCMP0EN` writer - High Compare 0 Enable"]
pub type HCMP0EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLB_SPEC, bool, O>;
#[doc = "Field `HCMP1EN` reader - High Compare 1 Enable"]
pub type HCMP1EN_R = crate::BitReader<bool>;
#[doc = "Field `HCMP1EN` writer - High Compare 1 Enable"]
pub type HCMP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLB_SPEC, bool, O>;
#[doc = "Field `HCMP2EN` reader - High Compare 2 Enable"]
pub type HCMP2EN_R = crate::BitReader<bool>;
#[doc = "Field `HCMP2EN` writer - High Compare 2 Enable"]
pub type HCMP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Low Compare 0 Enable"]
    #[inline(always)]
    pub fn lcmp0en(&self) -> LCMP0EN_R {
        LCMP0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Compare 1 Enable"]
    #[inline(always)]
    pub fn lcmp1en(&self) -> LCMP1EN_R {
        LCMP1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Compare 2 Enable"]
    #[inline(always)]
    pub fn lcmp2en(&self) -> LCMP2EN_R {
        LCMP2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - High Compare 0 Enable"]
    #[inline(always)]
    pub fn hcmp0en(&self) -> HCMP0EN_R {
        HCMP0EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High Compare 1 Enable"]
    #[inline(always)]
    pub fn hcmp1en(&self) -> HCMP1EN_R {
        HCMP1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High Compare 2 Enable"]
    #[inline(always)]
    pub fn hcmp2en(&self) -> HCMP2EN_R {
        HCMP2EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Compare 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp0en(&mut self) -> LCMP0EN_W<0> {
        LCMP0EN_W::new(self)
    }
    #[doc = "Bit 1 - Low Compare 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp1en(&mut self) -> LCMP1EN_W<1> {
        LCMP1EN_W::new(self)
    }
    #[doc = "Bit 2 - Low Compare 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp2en(&mut self) -> LCMP2EN_W<2> {
        LCMP2EN_W::new(self)
    }
    #[doc = "Bit 4 - High Compare 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hcmp0en(&mut self) -> HCMP0EN_W<4> {
        HCMP0EN_W::new(self)
    }
    #[doc = "Bit 5 - High Compare 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hcmp1en(&mut self) -> HCMP1EN_W<5> {
        HCMP1EN_W::new(self)
    }
    #[doc = "Bit 6 - High Compare 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hcmp2en(&mut self) -> HCMP2EN_W<6> {
        HCMP2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [split_ctrlb](index.html) module"]
pub struct SPLIT_CTRLB_SPEC;
impl crate::RegisterSpec for SPLIT_CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [split_ctrlb::R](R) reader structure"]
impl crate::Readable for SPLIT_CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [split_ctrlb::W](W) writer structure"]
impl crate::Writable for SPLIT_CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for SPLIT_CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
