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
#[doc = "Field `ICF1` reader - Input Capture Flag 1"]
pub type ICF1_R = crate::BitReader<bool>;
#[doc = "Field `ICF1` writer - Input Capture Flag 1"]
pub type ICF1_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR_SPEC, bool, O>;
#[doc = "Field `OCF1B` reader - Output Compare Flag 1B"]
pub type OCF1B_R = crate::BitReader<bool>;
#[doc = "Field `OCF1B` writer - Output Compare Flag 1B"]
pub type OCF1B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR_SPEC, bool, O>;
#[doc = "Field `OCF1A` reader - Output Compare Flag 1A"]
pub type OCF1A_R = crate::BitReader<bool>;
#[doc = "Field `OCF1A` writer - Output Compare Flag 1A"]
pub type OCF1A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR_SPEC, bool, O>;
#[doc = "Field `TOV1` reader - Timer/Counter1 Overflow Flag"]
pub type TOV1_R = crate::BitReader<bool>;
#[doc = "Field `TOV1` writer - Timer/Counter1 Overflow Flag"]
pub type TOV1_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - Input Capture Flag 1"]
    #[inline(always)]
    pub fn icf1(&self) -> ICF1_R {
        ICF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Output Compare Flag 1B"]
    #[inline(always)]
    pub fn ocf1b(&self) -> OCF1B_R {
        OCF1B_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output Compare Flag 1A"]
    #[inline(always)]
    pub fn ocf1a(&self) -> OCF1A_R {
        OCF1A_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer/Counter1 Overflow Flag"]
    #[inline(always)]
    pub fn tov1(&self) -> TOV1_R {
        TOV1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Input Capture Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn icf1(&mut self) -> ICF1_W<3> {
        ICF1_W::new(self)
    }
    #[doc = "Bit 5 - Output Compare Flag 1B"]
    #[inline(always)]
    #[must_use]
    pub fn ocf1b(&mut self) -> OCF1B_W<5> {
        OCF1B_W::new(self)
    }
    #[doc = "Bit 6 - Output Compare Flag 1A"]
    #[inline(always)]
    #[must_use]
    pub fn ocf1a(&mut self) -> OCF1A_W<6> {
        OCF1A_W::new(self)
    }
    #[doc = "Bit 7 - Timer/Counter1 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov1(&mut self) -> TOV1_W<7> {
        TOV1_W::new(self)
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
