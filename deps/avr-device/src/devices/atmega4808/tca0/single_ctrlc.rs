#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<SINGLE_CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLE_CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLE_CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLE_CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<SINGLE_CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLE_CTRLC_SPEC>;
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
impl From<crate::W<SINGLE_CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLE_CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP0OV` reader - Compare 0 Waveform Output Value"]
pub type CMP0OV_R = crate::BitReader<bool>;
#[doc = "Field `CMP0OV` writer - Compare 0 Waveform Output Value"]
pub type CMP0OV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_CTRLC_SPEC, bool, O>;
#[doc = "Field `CMP1OV` reader - Compare 1 Waveform Output Value"]
pub type CMP1OV_R = crate::BitReader<bool>;
#[doc = "Field `CMP1OV` writer - Compare 1 Waveform Output Value"]
pub type CMP1OV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_CTRLC_SPEC, bool, O>;
#[doc = "Field `CMP2OV` reader - Compare 2 Waveform Output Value"]
pub type CMP2OV_R = crate::BitReader<bool>;
#[doc = "Field `CMP2OV` writer - Compare 2 Waveform Output Value"]
pub type CMP2OV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_CTRLC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Compare 0 Waveform Output Value"]
    #[inline(always)]
    pub fn cmp0ov(&self) -> CMP0OV_R {
        CMP0OV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1 Waveform Output Value"]
    #[inline(always)]
    pub fn cmp1ov(&self) -> CMP1OV_R {
        CMP1OV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 2 Waveform Output Value"]
    #[inline(always)]
    pub fn cmp2ov(&self) -> CMP2OV_R {
        CMP2OV_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare 0 Waveform Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0ov(&mut self) -> CMP0OV_W<0> {
        CMP0OV_W::new(self)
    }
    #[doc = "Bit 1 - Compare 1 Waveform Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1ov(&mut self) -> CMP1OV_W<1> {
        CMP1OV_W::new(self)
    }
    #[doc = "Bit 2 - Compare 2 Waveform Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2ov(&mut self) -> CMP2OV_W<2> {
        CMP2OV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [single_ctrlc](index.html) module"]
pub struct SINGLE_CTRLC_SPEC;
impl crate::RegisterSpec for SINGLE_CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [single_ctrlc::R](R) reader structure"]
impl crate::Readable for SINGLE_CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [single_ctrlc::W](W) writer structure"]
impl crate::Writable for SINGLE_CTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for SINGLE_CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
