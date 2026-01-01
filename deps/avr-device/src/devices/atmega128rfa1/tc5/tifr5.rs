#[doc = "Register `TIFR5` reader"]
pub struct R(crate::R<TIFR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIFR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIFR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIFR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIFR5` writer"]
pub struct W(crate::W<TIFR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIFR5_SPEC>;
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
impl From<crate::W<TIFR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIFR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOV5` reader - Timer/Counter5 Overflow Flag"]
pub type TOV5_R = crate::BitReader<bool>;
#[doc = "Field `TOV5` writer - Timer/Counter5 Overflow Flag"]
pub type TOV5_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR5_SPEC, bool, O>;
#[doc = "Field `OCF5A` reader - Timer/Counter5 Output Compare A Match Flag"]
pub type OCF5A_R = crate::BitReader<bool>;
#[doc = "Field `OCF5A` writer - Timer/Counter5 Output Compare A Match Flag"]
pub type OCF5A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR5_SPEC, bool, O>;
#[doc = "Field `OCF5B` reader - Timer/Counter5 Output Compare B Match Flag"]
pub type OCF5B_R = crate::BitReader<bool>;
#[doc = "Field `OCF5B` writer - Timer/Counter5 Output Compare B Match Flag"]
pub type OCF5B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR5_SPEC, bool, O>;
#[doc = "Field `OCF5C` reader - Timer/Counter5 Output Compare C Match Flag"]
pub type OCF5C_R = crate::BitReader<bool>;
#[doc = "Field `OCF5C` writer - Timer/Counter5 Output Compare C Match Flag"]
pub type OCF5C_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR5_SPEC, bool, O>;
#[doc = "Field `ICF5` reader - Timer/Counter5 Input Capture Flag"]
pub type ICF5_R = crate::BitReader<bool>;
#[doc = "Field `ICF5` writer - Timer/Counter5 Input Capture Flag"]
pub type ICF5_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR5_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter5 Overflow Flag"]
    #[inline(always)]
    pub fn tov5(&self) -> TOV5_R {
        TOV5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter5 Output Compare A Match Flag"]
    #[inline(always)]
    pub fn ocf5a(&self) -> OCF5A_R {
        OCF5A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter5 Output Compare B Match Flag"]
    #[inline(always)]
    pub fn ocf5b(&self) -> OCF5B_R {
        OCF5B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter5 Output Compare C Match Flag"]
    #[inline(always)]
    pub fn ocf5c(&self) -> OCF5C_R {
        OCF5C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter5 Input Capture Flag"]
    #[inline(always)]
    pub fn icf5(&self) -> ICF5_R {
        ICF5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter5 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov5(&mut self) -> TOV5_W<0> {
        TOV5_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter5 Output Compare A Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf5a(&mut self) -> OCF5A_W<1> {
        OCF5A_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter5 Output Compare B Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf5b(&mut self) -> OCF5B_W<2> {
        OCF5B_W::new(self)
    }
    #[doc = "Bit 3 - Timer/Counter5 Output Compare C Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf5c(&mut self) -> OCF5C_W<3> {
        OCF5C_W::new(self)
    }
    #[doc = "Bit 5 - Timer/Counter5 Input Capture Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icf5(&mut self) -> ICF5_W<5> {
        ICF5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter5 Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tifr5](index.html) module"]
pub struct TIFR5_SPEC;
impl crate::RegisterSpec for TIFR5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tifr5::R](R) reader structure"]
impl crate::Readable for TIFR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tifr5::W](W) writer structure"]
impl crate::Writable for TIFR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR5 to value 0"]
impl crate::Resettable for TIFR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
