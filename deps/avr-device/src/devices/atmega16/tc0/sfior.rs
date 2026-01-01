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
#[doc = "Field `PSR10` reader - Prescaler Reset Timer/Counter1 and Timer/Counter0"]
pub type PSR10_R = crate::BitReader<bool>;
#[doc = "Field `PSR10` writer - Prescaler Reset Timer/Counter1 and Timer/Counter0"]
pub type PSR10_W<'a, const O: u8> = crate::BitWriter<'a, u8, SFIOR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Prescaler Reset Timer/Counter1 and Timer/Counter0"]
    #[inline(always)]
    pub fn psr10(&self) -> PSR10_R {
        PSR10_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Prescaler Reset Timer/Counter1 and Timer/Counter0"]
    #[inline(always)]
    #[must_use]
    pub fn psr10(&mut self) -> PSR10_W<0> {
        PSR10_W::new(self)
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
