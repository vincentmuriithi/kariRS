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
#[doc = "Field `ICF0` reader - Timer/Counter0 Input Capture Flag"]
pub type ICF0_R = crate::BitReader<bool>;
#[doc = "Field `ICF0` writer - Timer/Counter0 Input Capture Flag"]
pub type ICF0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR_SPEC, bool, O>;
#[doc = "Field `TOV0` reader - Timer/Counter0 Overflow Flag"]
pub type TOV0_R = crate::BitReader<bool>;
#[doc = "Field `TOV0` writer - Timer/Counter0 Overflow Flag"]
pub type TOV0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR_SPEC, bool, O>;
#[doc = "Field `OCF0B` reader - Timer/Counter0 Output Compare Flag 0B"]
pub type OCF0B_R = crate::BitReader<bool>;
#[doc = "Field `OCF0B` writer - Timer/Counter0 Output Compare Flag 0B"]
pub type OCF0B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR_SPEC, bool, O>;
#[doc = "Field `OCF0A` reader - Timer/Counter0 Output Compare Flag 0A"]
pub type OCF0A_R = crate::BitReader<bool>;
#[doc = "Field `OCF0A` writer - Timer/Counter0 Output Compare Flag 0A"]
pub type OCF0A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter0 Input Capture Flag"]
    #[inline(always)]
    pub fn icf0(&self) -> ICF0_R {
        ICF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter0 Overflow Flag"]
    #[inline(always)]
    pub fn tov0(&self) -> TOV0_R {
        TOV0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter0 Output Compare Flag 0B"]
    #[inline(always)]
    pub fn ocf0b(&self) -> OCF0B_R {
        OCF0B_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer/Counter0 Output Compare Flag 0A"]
    #[inline(always)]
    pub fn ocf0a(&self) -> OCF0A_R {
        OCF0A_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter0 Input Capture Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icf0(&mut self) -> ICF0_W<0> {
        ICF0_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter0 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov0(&mut self) -> TOV0_W<1> {
        TOV0_W::new(self)
    }
    #[doc = "Bit 3 - Timer/Counter0 Output Compare Flag 0B"]
    #[inline(always)]
    #[must_use]
    pub fn ocf0b(&mut self) -> OCF0B_W<3> {
        OCF0B_W::new(self)
    }
    #[doc = "Bit 4 - Timer/Counter0 Output Compare Flag 0A"]
    #[inline(always)]
    #[must_use]
    pub fn ocf0a(&mut self) -> OCF0A_W<4> {
        OCF0A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter0 Interrupt Flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tifr](index.html) module"]
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
