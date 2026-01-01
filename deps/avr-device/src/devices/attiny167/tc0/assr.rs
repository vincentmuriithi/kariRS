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
#[doc = "Field `TCR0BUB` reader - Timer/Counter0 Control Register B Update Busy"]
pub type TCR0BUB_R = crate::BitReader<bool>;
#[doc = "Field `TCR0BUB` writer - Timer/Counter0 Control Register B Update Busy"]
pub type TCR0BUB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `TCR0AUB` reader - Timer/Counter0 Control Register A Update Busy"]
pub type TCR0AUB_R = crate::BitReader<bool>;
#[doc = "Field `TCR0AUB` writer - Timer/Counter0 Control Register A Update Busy"]
pub type TCR0AUB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `OCR0AUB` reader - Output Compare Register 0A Update Busy"]
pub type OCR0AUB_R = crate::BitReader<bool>;
#[doc = "Field `OCR0AUB` writer - Output Compare Register 0A Update Busy"]
pub type OCR0AUB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `TCN0UB` reader - Timer/Counter0 Update Busy"]
pub type TCN0UB_R = crate::BitReader<bool>;
#[doc = "Field `TCN0UB` writer - Timer/Counter0 Update Busy"]
pub type TCN0UB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `AS0` reader - Asynchronous Timer/Counter0"]
pub type AS0_R = crate::BitReader<bool>;
#[doc = "Field `AS0` writer - Asynchronous Timer/Counter0"]
pub type AS0_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `EXCLK` reader - Enable External Clock Input"]
pub type EXCLK_R = crate::BitReader<bool>;
#[doc = "Field `EXCLK` writer - Enable External Clock Input"]
pub type EXCLK_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter0 Control Register B Update Busy"]
    #[inline(always)]
    pub fn tcr0bub(&self) -> TCR0BUB_R {
        TCR0BUB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter0 Control Register A Update Busy"]
    #[inline(always)]
    pub fn tcr0aub(&self) -> TCR0AUB_R {
        TCR0AUB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Compare Register 0A Update Busy"]
    #[inline(always)]
    pub fn ocr0aub(&self) -> OCR0AUB_R {
        OCR0AUB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer/Counter0 Update Busy"]
    #[inline(always)]
    pub fn tcn0ub(&self) -> TCN0UB_R {
        TCN0UB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Timer/Counter0"]
    #[inline(always)]
    pub fn as0(&self) -> AS0_R {
        AS0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable External Clock Input"]
    #[inline(always)]
    pub fn exclk(&self) -> EXCLK_R {
        EXCLK_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter0 Control Register B Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcr0bub(&mut self) -> TCR0BUB_W<0> {
        TCR0BUB_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter0 Control Register A Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcr0aub(&mut self) -> TCR0AUB_W<1> {
        TCR0AUB_W::new(self)
    }
    #[doc = "Bit 3 - Output Compare Register 0A Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn ocr0aub(&mut self) -> OCR0AUB_W<3> {
        OCR0AUB_W::new(self)
    }
    #[doc = "Bit 4 - Timer/Counter0 Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcn0ub(&mut self) -> TCN0UB_W<4> {
        TCN0UB_W::new(self)
    }
    #[doc = "Bit 5 - Asynchronous Timer/Counter0"]
    #[inline(always)]
    #[must_use]
    pub fn as0(&mut self) -> AS0_W<5> {
        AS0_W::new(self)
    }
    #[doc = "Bit 6 - Enable External Clock Input"]
    #[inline(always)]
    #[must_use]
    pub fn exclk(&mut self) -> EXCLK_W<6> {
        EXCLK_W::new(self)
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
