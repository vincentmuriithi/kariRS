#[doc = "Register `TIFR3` reader"]
pub struct R(crate::R<TIFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIFR3` writer"]
pub struct W(crate::W<TIFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIFR3_SPEC>;
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
impl From<crate::W<TIFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIFR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOV3` reader - Timer/Counter3 Overflow Flag"]
pub type TOV3_R = crate::BitReader<bool>;
#[doc = "Field `TOV3` writer - Timer/Counter3 Overflow Flag"]
pub type TOV3_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR3_SPEC, bool, O>;
#[doc = "Field `OCF3A` reader - Timer/Counter3 Output Compare A Match Flag"]
pub type OCF3A_R = crate::BitReader<bool>;
#[doc = "Field `OCF3A` writer - Timer/Counter3 Output Compare A Match Flag"]
pub type OCF3A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR3_SPEC, bool, O>;
#[doc = "Field `OCF3B` reader - Timer/Counter3 Output Compare B Match Flag"]
pub type OCF3B_R = crate::BitReader<bool>;
#[doc = "Field `OCF3B` writer - Timer/Counter3 Output Compare B Match Flag"]
pub type OCF3B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR3_SPEC, bool, O>;
#[doc = "Field `OCF3C` reader - Timer/Counter3 Output Compare C Match Flag"]
pub type OCF3C_R = crate::BitReader<bool>;
#[doc = "Field `OCF3C` writer - Timer/Counter3 Output Compare C Match Flag"]
pub type OCF3C_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR3_SPEC, bool, O>;
#[doc = "Field `ICF3` reader - Timer/Counter3 Input Capture Flag"]
pub type ICF3_R = crate::BitReader<bool>;
#[doc = "Field `ICF3` writer - Timer/Counter3 Input Capture Flag"]
pub type ICF3_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter3 Overflow Flag"]
    #[inline(always)]
    pub fn tov3(&self) -> TOV3_R {
        TOV3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter3 Output Compare A Match Flag"]
    #[inline(always)]
    pub fn ocf3a(&self) -> OCF3A_R {
        OCF3A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter3 Output Compare B Match Flag"]
    #[inline(always)]
    pub fn ocf3b(&self) -> OCF3B_R {
        OCF3B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter3 Output Compare C Match Flag"]
    #[inline(always)]
    pub fn ocf3c(&self) -> OCF3C_R {
        OCF3C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter3 Input Capture Flag"]
    #[inline(always)]
    pub fn icf3(&self) -> ICF3_R {
        ICF3_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter3 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov3(&mut self) -> TOV3_W<0> {
        TOV3_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter3 Output Compare A Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf3a(&mut self) -> OCF3A_W<1> {
        OCF3A_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter3 Output Compare B Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf3b(&mut self) -> OCF3B_W<2> {
        OCF3B_W::new(self)
    }
    #[doc = "Bit 3 - Timer/Counter3 Output Compare C Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf3c(&mut self) -> OCF3C_W<3> {
        OCF3C_W::new(self)
    }
    #[doc = "Bit 5 - Timer/Counter3 Input Capture Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icf3(&mut self) -> ICF3_W<5> {
        ICF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter3 Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tifr3](index.html) module"]
pub struct TIFR3_SPEC;
impl crate::RegisterSpec for TIFR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tifr3::R](R) reader structure"]
impl crate::Readable for TIFR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tifr3::W](W) writer structure"]
impl crate::Writable for TIFR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR3 to value 0"]
impl crate::Resettable for TIFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
