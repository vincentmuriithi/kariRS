#[doc = "Register `TIFR4` reader"]
pub struct R(crate::R<TIFR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIFR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIFR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIFR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIFR4` writer"]
pub struct W(crate::W<TIFR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIFR4_SPEC>;
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
impl From<crate::W<TIFR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIFR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOV4` reader - Timer/Counter4 Overflow Flag"]
pub type TOV4_R = crate::BitReader<bool>;
#[doc = "Field `TOV4` writer - Timer/Counter4 Overflow Flag"]
pub type TOV4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR4_SPEC, bool, O>;
#[doc = "Field `OCF4A` reader - Output Compare Flag 4A"]
pub type OCF4A_R = crate::BitReader<bool>;
#[doc = "Field `OCF4A` writer - Output Compare Flag 4A"]
pub type OCF4A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR4_SPEC, bool, O>;
#[doc = "Field `OCF4B` reader - Output Compare Flag 4B"]
pub type OCF4B_R = crate::BitReader<bool>;
#[doc = "Field `OCF4B` writer - Output Compare Flag 4B"]
pub type OCF4B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR4_SPEC, bool, O>;
#[doc = "Field `OCF4C` reader - Output Compare Flag 4C"]
pub type OCF4C_R = crate::BitReader<bool>;
#[doc = "Field `OCF4C` writer - Output Compare Flag 4C"]
pub type OCF4C_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR4_SPEC, bool, O>;
#[doc = "Field `ICF4` reader - Input Capture Flag 4"]
pub type ICF4_R = crate::BitReader<bool>;
#[doc = "Field `ICF4` writer - Input Capture Flag 4"]
pub type ICF4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIFR4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter4 Overflow Flag"]
    #[inline(always)]
    pub fn tov4(&self) -> TOV4_R {
        TOV4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Compare Flag 4A"]
    #[inline(always)]
    pub fn ocf4a(&self) -> OCF4A_R {
        OCF4A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Compare Flag 4B"]
    #[inline(always)]
    pub fn ocf4b(&self) -> OCF4B_R {
        OCF4B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Compare Flag 4C"]
    #[inline(always)]
    pub fn ocf4c(&self) -> OCF4C_R {
        OCF4C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Input Capture Flag 4"]
    #[inline(always)]
    pub fn icf4(&self) -> ICF4_R {
        ICF4_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter4 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov4(&mut self) -> TOV4_W<0> {
        TOV4_W::new(self)
    }
    #[doc = "Bit 1 - Output Compare Flag 4A"]
    #[inline(always)]
    #[must_use]
    pub fn ocf4a(&mut self) -> OCF4A_W<1> {
        OCF4A_W::new(self)
    }
    #[doc = "Bit 2 - Output Compare Flag 4B"]
    #[inline(always)]
    #[must_use]
    pub fn ocf4b(&mut self) -> OCF4B_W<2> {
        OCF4B_W::new(self)
    }
    #[doc = "Bit 3 - Output Compare Flag 4C"]
    #[inline(always)]
    #[must_use]
    pub fn ocf4c(&mut self) -> OCF4C_W<3> {
        OCF4C_W::new(self)
    }
    #[doc = "Bit 5 - Input Capture Flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn icf4(&mut self) -> ICF4_W<5> {
        ICF4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter4 Interrupt Flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tifr4](index.html) module"]
pub struct TIFR4_SPEC;
impl crate::RegisterSpec for TIFR4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tifr4::R](R) reader structure"]
impl crate::Readable for TIFR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tifr4::W](W) writer structure"]
impl crate::Writable for TIFR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR4 to value 0"]
impl crate::Resettable for TIFR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
