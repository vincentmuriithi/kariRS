#[doc = "Register `ASSR` reader"]
pub struct R(crate::R<ASSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASSR` writer"]
pub struct W(crate::W<ASSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASSR_SPEC>;
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
impl From<crate::W<ASSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCR2BUB` reader - Timer/Counter2 Control Register B Update Busy"]
pub type TCR2BUB_R = crate::BitReader<bool>;
#[doc = "Field `TCR2BUB` writer - Timer/Counter2 Control Register B Update Busy"]
pub type TCR2BUB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `TCR2AUB` reader - Timer/Counter2 Control Register A Update Busy"]
pub type TCR2AUB_R = crate::BitReader<bool>;
#[doc = "Field `TCR2AUB` writer - Timer/Counter2 Control Register A Update Busy"]
pub type TCR2AUB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `OCR2BUB` reader - Timer/Counter2 Output Compare Register B Update Busy"]
pub type OCR2BUB_R = crate::BitReader<bool>;
#[doc = "Field `OCR2BUB` writer - Timer/Counter2 Output Compare Register B Update Busy"]
pub type OCR2BUB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `OCR2AUB` reader - Timer/Counter2 Output Compare Register A Update Busy"]
pub type OCR2AUB_R = crate::BitReader<bool>;
#[doc = "Field `OCR2AUB` writer - Timer/Counter2 Output Compare Register A Update Busy"]
pub type OCR2AUB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `TCN2UB` reader - Timer/Counter2 Update Busy"]
pub type TCN2UB_R = crate::BitReader<bool>;
#[doc = "Field `TCN2UB` writer - Timer/Counter2 Update Busy"]
pub type TCN2UB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `AS2` reader - Timer/Counter2 Asynchronous Mode"]
pub type AS2_R = crate::BitReader<bool>;
#[doc = "Field `AS2` writer - Timer/Counter2 Asynchronous Mode"]
pub type AS2_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `EXCLK` reader - Enable External Clock Input"]
pub type EXCLK_R = crate::BitReader<bool>;
#[doc = "Field `EXCLK` writer - Enable External Clock Input"]
pub type EXCLK_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `EXCLKAMR` reader - Enable External Clock Input for AMR"]
pub type EXCLKAMR_R = crate::BitReader<bool>;
#[doc = "Field `EXCLKAMR` writer - Enable External Clock Input for AMR"]
pub type EXCLKAMR_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter2 Control Register B Update Busy"]
    #[inline(always)]
    pub fn tcr2bub(&self) -> TCR2BUB_R {
        TCR2BUB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter2 Control Register A Update Busy"]
    #[inline(always)]
    pub fn tcr2aub(&self) -> TCR2AUB_R {
        TCR2AUB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter2 Output Compare Register B Update Busy"]
    #[inline(always)]
    pub fn ocr2bub(&self) -> OCR2BUB_R {
        OCR2BUB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter2 Output Compare Register A Update Busy"]
    #[inline(always)]
    pub fn ocr2aub(&self) -> OCR2AUB_R {
        OCR2AUB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer/Counter2 Update Busy"]
    #[inline(always)]
    pub fn tcn2ub(&self) -> TCN2UB_R {
        TCN2UB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter2 Asynchronous Mode"]
    #[inline(always)]
    pub fn as2(&self) -> AS2_R {
        AS2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable External Clock Input"]
    #[inline(always)]
    pub fn exclk(&self) -> EXCLK_R {
        EXCLK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable External Clock Input for AMR"]
    #[inline(always)]
    pub fn exclkamr(&self) -> EXCLKAMR_R {
        EXCLKAMR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter2 Control Register B Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcr2bub(&mut self) -> TCR2BUB_W<0> {
        TCR2BUB_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter2 Control Register A Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcr2aub(&mut self) -> TCR2AUB_W<1> {
        TCR2AUB_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter2 Output Compare Register B Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn ocr2bub(&mut self) -> OCR2BUB_W<2> {
        OCR2BUB_W::new(self)
    }
    #[doc = "Bit 3 - Timer/Counter2 Output Compare Register A Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn ocr2aub(&mut self) -> OCR2AUB_W<3> {
        OCR2AUB_W::new(self)
    }
    #[doc = "Bit 4 - Timer/Counter2 Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcn2ub(&mut self) -> TCN2UB_W<4> {
        TCN2UB_W::new(self)
    }
    #[doc = "Bit 5 - Timer/Counter2 Asynchronous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn as2(&mut self) -> AS2_W<5> {
        AS2_W::new(self)
    }
    #[doc = "Bit 6 - Enable External Clock Input"]
    #[inline(always)]
    #[must_use]
    pub fn exclk(&mut self) -> EXCLK_W<6> {
        EXCLK_W::new(self)
    }
    #[doc = "Bit 7 - Enable External Clock Input for AMR"]
    #[inline(always)]
    #[must_use]
    pub fn exclkamr(&mut self) -> EXCLKAMR_W<7> {
        EXCLKAMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assr](index.html) module"]
pub struct ASSR_SPEC;
impl crate::RegisterSpec for ASSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [assr::R](R) reader structure"]
impl crate::Readable for ASSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [assr::W](W) writer structure"]
impl crate::Writable for ASSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASSR to value 0"]
impl crate::Resettable for ASSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
