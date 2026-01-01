#[doc = "Register `SFIOR` reader"]
pub struct R(crate::R<SFIOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFIOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFIOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFIOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFIOR` writer"]
pub struct W(crate::W<SFIOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFIOR_SPEC>;
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
impl From<crate::W<SFIOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFIOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSR321` reader - Prescaler Reset Timer/Counter3, Timer/Counter2, and Timer/Counter1"]
pub type PSR321_R = crate::BitReader<bool>;
#[doc = "Field `PSR321` writer - Prescaler Reset Timer/Counter3, Timer/Counter2, and Timer/Counter1"]
pub type PSR321_W<'a, const O: u8> = crate::BitWriter<'a, u8, SFIOR_SPEC, bool, O>;
#[doc = "Field `PSR0` reader - Prescaler Reset Timer/Counter0"]
pub type PSR0_R = crate::BitReader<bool>;
#[doc = "Field `PSR0` writer - Prescaler Reset Timer/Counter0"]
pub type PSR0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SFIOR_SPEC, bool, O>;
#[doc = "Field `PUD` reader - Pull Up Disable"]
pub type PUD_R = crate::BitReader<bool>;
#[doc = "Field `PUD` writer - Pull Up Disable"]
pub type PUD_W<'a, const O: u8> = crate::BitWriter<'a, u8, SFIOR_SPEC, bool, O>;
#[doc = "Field `ACME` reader - Analog Comparator Multiplexer Enable"]
pub type ACME_R = crate::BitReader<bool>;
#[doc = "Field `ACME` writer - Analog Comparator Multiplexer Enable"]
pub type ACME_W<'a, const O: u8> = crate::BitWriter<'a, u8, SFIOR_SPEC, bool, O>;
#[doc = "Field `TSM` reader - Timer/Counter Synchronization Mode"]
pub type TSM_R = crate::BitReader<bool>;
#[doc = "Field `TSM` writer - Timer/Counter Synchronization Mode"]
pub type TSM_W<'a, const O: u8> = crate::BitWriter<'a, u8, SFIOR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Prescaler Reset Timer/Counter3, Timer/Counter2, and Timer/Counter1"]
    #[inline(always)]
    pub fn psr321(&self) -> PSR321_R {
        PSR321_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prescaler Reset Timer/Counter0"]
    #[inline(always)]
    pub fn psr0(&self) -> PSR0_R {
        PSR0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull Up Disable"]
    #[inline(always)]
    pub fn pud(&self) -> PUD_R {
        PUD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator Multiplexer Enable"]
    #[inline(always)]
    pub fn acme(&self) -> ACME_R {
        ACME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer/Counter Synchronization Mode"]
    #[inline(always)]
    pub fn tsm(&self) -> TSM_R {
        TSM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Prescaler Reset Timer/Counter3, Timer/Counter2, and Timer/Counter1"]
    #[inline(always)]
    #[must_use]
    pub fn psr321(&mut self) -> PSR321_W<0> {
        PSR321_W::new(self)
    }
    #[doc = "Bit 1 - Prescaler Reset Timer/Counter0"]
    #[inline(always)]
    #[must_use]
    pub fn psr0(&mut self) -> PSR0_W<1> {
        PSR0_W::new(self)
    }
    #[doc = "Bit 2 - Pull Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pud(&mut self) -> PUD_W<2> {
        PUD_W::new(self)
    }
    #[doc = "Bit 3 - Analog Comparator Multiplexer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acme(&mut self) -> ACME_W<3> {
        ACME_W::new(self)
    }
    #[doc = "Bit 7 - Timer/Counter Synchronization Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tsm(&mut self) -> TSM_W<7> {
        TSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Special Function IO Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfior](index.html) module"]
pub struct SFIOR_SPEC;
impl crate::RegisterSpec for SFIOR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sfior::R](R) reader structure"]
impl crate::Readable for SFIOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfior::W](W) writer structure"]
impl crate::Writable for SFIOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFIOR to value 0"]
impl crate::Resettable for SFIOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
