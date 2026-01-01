#[doc = "Register `TCCR3C` reader"]
pub struct R(crate::R<TCCR3C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR3C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR3C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR3C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR3C` writer"]
pub struct W(crate::W<TCCR3C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR3C_SPEC>;
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
impl From<crate::W<TCCR3C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR3C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FOC3C` reader - Force Output Compare 3C"]
pub type FOC3C_R = crate::BitReader<bool>;
#[doc = "Field `FOC3C` writer - Force Output Compare 3C"]
pub type FOC3C_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR3C_SPEC, bool, O>;
#[doc = "Field `FOC3B` reader - Force Output Compare 3B"]
pub type FOC3B_R = crate::BitReader<bool>;
#[doc = "Field `FOC3B` writer - Force Output Compare 3B"]
pub type FOC3B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR3C_SPEC, bool, O>;
#[doc = "Field `FOC3A` reader - Force Output Compare 3A"]
pub type FOC3A_R = crate::BitReader<bool>;
#[doc = "Field `FOC3A` writer - Force Output Compare 3A"]
pub type FOC3A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR3C_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - Force Output Compare 3C"]
    #[inline(always)]
    pub fn foc3c(&self) -> FOC3C_R {
        FOC3C_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Output Compare 3B"]
    #[inline(always)]
    pub fn foc3b(&self) -> FOC3B_R {
        FOC3B_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Output Compare 3A"]
    #[inline(always)]
    pub fn foc3a(&self) -> FOC3A_R {
        FOC3A_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Force Output Compare 3C"]
    #[inline(always)]
    #[must_use]
    pub fn foc3c(&mut self) -> FOC3C_W<5> {
        FOC3C_W::new(self)
    }
    #[doc = "Bit 6 - Force Output Compare 3B"]
    #[inline(always)]
    #[must_use]
    pub fn foc3b(&mut self) -> FOC3B_W<6> {
        FOC3B_W::new(self)
    }
    #[doc = "Bit 7 - Force Output Compare 3A"]
    #[inline(always)]
    #[must_use]
    pub fn foc3a(&mut self) -> FOC3A_W<7> {
        FOC3A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter 3 Control Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr3c](index.html) module"]
pub struct TCCR3C_SPEC;
impl crate::RegisterSpec for TCCR3C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr3c::R](R) reader structure"]
impl crate::Readable for TCCR3C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr3c::W](W) writer structure"]
impl crate::Writable for TCCR3C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR3C to value 0"]
impl crate::Resettable for TCCR3C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
