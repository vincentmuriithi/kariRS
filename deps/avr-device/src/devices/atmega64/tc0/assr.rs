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
#[doc = "Field `TCR0UB` reader - Timer/Counter Control Register 0 Update Busy"]
pub type TCR0UB_R = crate::BitReader<bool>;
#[doc = "Field `TCR0UB` writer - Timer/Counter Control Register 0 Update Busy"]
pub type TCR0UB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `OCR0UB` reader - Output Compare register 0 Busy"]
pub type OCR0UB_R = crate::BitReader<bool>;
#[doc = "Field `OCR0UB` writer - Output Compare register 0 Busy"]
pub type OCR0UB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `TCN0UB` reader - Timer/Counter0 Update Busy"]
pub type TCN0UB_R = crate::BitReader<bool>;
#[doc = "Field `TCN0UB` writer - Timer/Counter0 Update Busy"]
pub type TCN0UB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
#[doc = "Field `AS0` reader - Asynchronus Timer/Counter 0"]
pub type AS0_R = crate::BitReader<bool>;
#[doc = "Field `AS0` writer - Asynchronus Timer/Counter 0"]
pub type AS0_W<'a, const O: u8> = crate::BitWriter<'a, u8, ASSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter Control Register 0 Update Busy"]
    #[inline(always)]
    pub fn tcr0ub(&self) -> TCR0UB_R {
        TCR0UB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Compare register 0 Busy"]
    #[inline(always)]
    pub fn ocr0ub(&self) -> OCR0UB_R {
        OCR0UB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter0 Update Busy"]
    #[inline(always)]
    pub fn tcn0ub(&self) -> TCN0UB_R {
        TCN0UB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Asynchronus Timer/Counter 0"]
    #[inline(always)]
    pub fn as0(&self) -> AS0_R {
        AS0_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter Control Register 0 Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcr0ub(&mut self) -> TCR0UB_W<0> {
        TCR0UB_W::new(self)
    }
    #[doc = "Bit 1 - Output Compare register 0 Busy"]
    #[inline(always)]
    #[must_use]
    pub fn ocr0ub(&mut self) -> OCR0UB_W<1> {
        OCR0UB_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter0 Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcn0ub(&mut self) -> TCN0UB_W<2> {
        TCN0UB_W::new(self)
    }
    #[doc = "Bit 3 - Asynchronus Timer/Counter 0"]
    #[inline(always)]
    #[must_use]
    pub fn as0(&mut self) -> AS0_W<3> {
        AS0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronus Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assr](index.html) module"]
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
