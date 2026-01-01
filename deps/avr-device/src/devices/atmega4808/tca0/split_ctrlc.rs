#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<SPLIT_CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPLIT_CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPLIT_CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPLIT_CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<SPLIT_CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPLIT_CTRLC_SPEC>;
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
impl From<crate::W<SPLIT_CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPLIT_CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCMP0OV` reader - Low Compare 0 Output Value"]
pub type LCMP0OV_R = crate::BitReader<bool>;
#[doc = "Field `LCMP0OV` writer - Low Compare 0 Output Value"]
pub type LCMP0OV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLC_SPEC, bool, O>;
#[doc = "Field `LCMP1OV` reader - Low Compare 1 Output Value"]
pub type LCMP1OV_R = crate::BitReader<bool>;
#[doc = "Field `LCMP1OV` writer - Low Compare 1 Output Value"]
pub type LCMP1OV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLC_SPEC, bool, O>;
#[doc = "Field `LCMP2OV` reader - Low Compare 2 Output Value"]
pub type LCMP2OV_R = crate::BitReader<bool>;
#[doc = "Field `LCMP2OV` writer - Low Compare 2 Output Value"]
pub type LCMP2OV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLC_SPEC, bool, O>;
#[doc = "Field `HCMP0OV` reader - High Compare 0 Output Value"]
pub type HCMP0OV_R = crate::BitReader<bool>;
#[doc = "Field `HCMP0OV` writer - High Compare 0 Output Value"]
pub type HCMP0OV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLC_SPEC, bool, O>;
#[doc = "Field `HCMP1OV` reader - High Compare 1 Output Value"]
pub type HCMP1OV_R = crate::BitReader<bool>;
#[doc = "Field `HCMP1OV` writer - High Compare 1 Output Value"]
pub type HCMP1OV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLC_SPEC, bool, O>;
#[doc = "Field `HCMP2OV` reader - High Compare 2 Output Value"]
pub type HCMP2OV_R = crate::BitReader<bool>;
#[doc = "Field `HCMP2OV` writer - High Compare 2 Output Value"]
pub type HCMP2OV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Low Compare 0 Output Value"]
    #[inline(always)]
    pub fn lcmp0ov(&self) -> LCMP0OV_R {
        LCMP0OV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Compare 1 Output Value"]
    #[inline(always)]
    pub fn lcmp1ov(&self) -> LCMP1OV_R {
        LCMP1OV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Compare 2 Output Value"]
    #[inline(always)]
    pub fn lcmp2ov(&self) -> LCMP2OV_R {
        LCMP2OV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - High Compare 0 Output Value"]
    #[inline(always)]
    pub fn hcmp0ov(&self) -> HCMP0OV_R {
        HCMP0OV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High Compare 1 Output Value"]
    #[inline(always)]
    pub fn hcmp1ov(&self) -> HCMP1OV_R {
        HCMP1OV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High Compare 2 Output Value"]
    #[inline(always)]
    pub fn hcmp2ov(&self) -> HCMP2OV_R {
        HCMP2OV_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Compare 0 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp0ov(&mut self) -> LCMP0OV_W<0> {
        LCMP0OV_W::new(self)
    }
    #[doc = "Bit 1 - Low Compare 1 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp1ov(&mut self) -> LCMP1OV_W<1> {
        LCMP1OV_W::new(self)
    }
    #[doc = "Bit 2 - Low Compare 2 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp2ov(&mut self) -> LCMP2OV_W<2> {
        LCMP2OV_W::new(self)
    }
    #[doc = "Bit 4 - High Compare 0 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn hcmp0ov(&mut self) -> HCMP0OV_W<4> {
        HCMP0OV_W::new(self)
    }
    #[doc = "Bit 5 - High Compare 1 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn hcmp1ov(&mut self) -> HCMP1OV_W<5> {
        HCMP1OV_W::new(self)
    }
    #[doc = "Bit 6 - High Compare 2 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn hcmp2ov(&mut self) -> HCMP2OV_W<6> {
        HCMP2OV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [split_ctrlc](index.html) module"]
pub struct SPLIT_CTRLC_SPEC;
impl crate::RegisterSpec for SPLIT_CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [split_ctrlc::R](R) reader structure"]
impl crate::Readable for SPLIT_CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [split_ctrlc::W](W) writer structure"]
impl crate::Writable for SPLIT_CTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for SPLIT_CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
