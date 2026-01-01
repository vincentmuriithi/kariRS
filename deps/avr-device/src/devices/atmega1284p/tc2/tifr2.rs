#[doc = "Register `TIFR2` reader"]
pub struct R(crate::R<TIFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIFR2` writer"]
pub struct W(crate::W<TIFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIFR2_SPEC>;
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
impl From<crate::W<TIFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOV2` reader - Timer/Counter2 Overflow Flag"]
pub type TOV2_R = crate::BitReader<bool>;
#[doc = "Field `TOV2` writer - Timer/Counter2 Overflow Flag"]
pub type TOV2_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR2_SPEC, bool, O>;
#[doc = "Field `OCF2A` reader - Output Compare Flag 2A"]
pub type OCF2A_R = crate::BitReader<bool>;
#[doc = "Field `OCF2A` writer - Output Compare Flag 2A"]
pub type OCF2A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR2_SPEC, bool, O>;
#[doc = "Field `OCF2B` reader - Output Compare Flag 2B"]
pub type OCF2B_R = crate::BitReader<bool>;
#[doc = "Field `OCF2B` writer - Output Compare Flag 2B"]
pub type OCF2B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter2 Overflow Flag"]
    #[inline(always)]
    pub fn tov2(&self) -> TOV2_R {
        TOV2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Compare Flag 2A"]
    #[inline(always)]
    pub fn ocf2a(&self) -> OCF2A_R {
        OCF2A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Compare Flag 2B"]
    #[inline(always)]
    pub fn ocf2b(&self) -> OCF2B_R {
        OCF2B_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter2 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov2(&mut self) -> TOV2_W<0> {
        TOV2_W::new(self)
    }
    #[doc = "Bit 1 - Output Compare Flag 2A"]
    #[inline(always)]
    #[must_use]
    pub fn ocf2a(&mut self) -> OCF2A_W<1> {
        OCF2A_W::new(self)
    }
    #[doc = "Bit 2 - Output Compare Flag 2B"]
    #[inline(always)]
    #[must_use]
    pub fn ocf2b(&mut self) -> OCF2B_W<2> {
        OCF2B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tifr2](index.html) module"]
pub struct TIFR2_SPEC;
impl crate::RegisterSpec for TIFR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tifr2::R](R) reader structure"]
impl crate::Readable for TIFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tifr2::W](W) writer structure"]
impl crate::Writable for TIFR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR2 to value 0"]
impl crate::Resettable for TIFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
