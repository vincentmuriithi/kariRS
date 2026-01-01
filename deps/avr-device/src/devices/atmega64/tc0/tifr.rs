#[doc = "Register `TIFR` reader"]
pub struct R(crate::R<TIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIFR` writer"]
pub struct W(crate::W<TIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIFR_SPEC>;
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
impl From<crate::W<TIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOV0` reader - Timer/Counter0 Overflow Flag"]
pub type TOV0_R = crate::BitReader<bool>;
#[doc = "Field `TOV0` writer - Timer/Counter0 Overflow Flag"]
pub type TOV0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR_SPEC, bool, O>;
#[doc = "Field `OCF0` reader - Output Compare Flag 0"]
pub type OCF0_R = crate::BitReader<bool>;
#[doc = "Field `OCF0` writer - Output Compare Flag 0"]
pub type OCF0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter0 Overflow Flag"]
    #[inline(always)]
    pub fn tov0(&self) -> TOV0_R {
        TOV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Compare Flag 0"]
    #[inline(always)]
    pub fn ocf0(&self) -> OCF0_R {
        OCF0_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter0 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov0(&mut self) -> TOV0_W<0> {
        TOV0_W::new(self)
    }
    #[doc = "Bit 1 - Output Compare Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn ocf0(&mut self) -> OCF0_W<1> {
        OCF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter Interrupt Flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tifr](index.html) module"]
pub struct TIFR_SPEC;
impl crate::RegisterSpec for TIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tifr::R](R) reader structure"]
impl crate::Readable for TIFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tifr::W](W) writer structure"]
impl crate::Writable for TIFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR to value 0"]
impl crate::Resettable for TIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
