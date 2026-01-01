#[doc = "Register `TIFR1` reader"]
pub struct R(crate::R<TIFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIFR1` writer"]
pub struct W(crate::W<TIFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIFR1_SPEC>;
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
impl From<crate::W<TIFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIFR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOV1` reader - Timer/Counter1 Overflow Flag"]
pub type TOV1_R = crate::BitReader<bool>;
#[doc = "Field `TOV1` writer - Timer/Counter1 Overflow Flag"]
pub type TOV1_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR1_SPEC, bool, O>;
#[doc = "Field `OCF1A` reader - Timer/Counter1 Output Compare A Match Flag"]
pub type OCF1A_R = crate::BitReader<bool>;
#[doc = "Field `OCF1A` writer - Timer/Counter1 Output Compare A Match Flag"]
pub type OCF1A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR1_SPEC, bool, O>;
#[doc = "Field `OCF1B` reader - Timer/Counter1 Output Compare B Match Flag"]
pub type OCF1B_R = crate::BitReader<bool>;
#[doc = "Field `OCF1B` writer - Timer/Counter1 Output Compare B Match Flag"]
pub type OCF1B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR1_SPEC, bool, O>;
#[doc = "Field `ICF1` reader - Timer/Counter1 Input Capture Flag"]
pub type ICF1_R = crate::BitReader<bool>;
#[doc = "Field `ICF1` writer - Timer/Counter1 Input Capture Flag"]
pub type ICF1_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter1 Overflow Flag"]
    #[inline(always)]
    pub fn tov1(&self) -> TOV1_R {
        TOV1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter1 Output Compare A Match Flag"]
    #[inline(always)]
    pub fn ocf1a(&self) -> OCF1A_R {
        OCF1A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter1 Output Compare B Match Flag"]
    #[inline(always)]
    pub fn ocf1b(&self) -> OCF1B_R {
        OCF1B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter1 Input Capture Flag"]
    #[inline(always)]
    pub fn icf1(&self) -> ICF1_R {
        ICF1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter1 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov1(&mut self) -> TOV1_W<0> {
        TOV1_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter1 Output Compare A Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf1a(&mut self) -> OCF1A_W<1> {
        OCF1A_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter1 Output Compare B Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf1b(&mut self) -> OCF1B_W<2> {
        OCF1B_W::new(self)
    }
    #[doc = "Bit 5 - Timer/Counter1 Input Capture Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icf1(&mut self) -> ICF1_W<5> {
        ICF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter1 Interrupt Flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tifr1](index.html) module"]
pub struct TIFR1_SPEC;
impl crate::RegisterSpec for TIFR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tifr1::R](R) reader structure"]
impl crate::Readable for TIFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tifr1::W](W) writer structure"]
impl crate::Writable for TIFR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR1 to value 0"]
impl crate::Resettable for TIFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
