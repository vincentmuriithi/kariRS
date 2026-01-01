#[doc = "Register `MCUSR` reader"]
pub struct R(crate::R<MCUSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUSR` writer"]
pub struct W(crate::W<MCUSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUSR_SPEC>;
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
impl From<crate::W<MCUSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JTRF` reader - JTAG Reset Flag"]
pub type JTRF_R = crate::BitReader<bool>;
#[doc = "Field `JTRF` writer - JTAG Reset Flag"]
pub type JTRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - JTAG Reset Flag"]
    #[inline(always)]
    pub fn jtrf(&self) -> JTRF_R {
        JTRF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - JTAG Reset Flag"]
    #[inline(always)]
    #[must_use]
    pub fn jtrf(&mut self) -> JTRF_W<4> {
        JTRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcusr](index.html) module"]
pub struct MCUSR_SPEC;
impl crate::RegisterSpec for MCUSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mcusr::R](R) reader structure"]
impl crate::Readable for MCUSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcusr::W](W) writer structure"]
impl crate::Writable for MCUSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUSR to value 0"]
impl crate::Resettable for MCUSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
