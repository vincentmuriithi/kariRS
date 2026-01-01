#[doc = "Register `TCCR4C` reader"]
pub struct R(crate::R<TCCR4C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR4C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR4C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR4C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR4C` writer"]
pub struct W(crate::W<TCCR4C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR4C_SPEC>;
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
impl From<crate::W<TCCR4C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR4C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FOC4C` writer - Force Output Compare 4C"]
pub type FOC4C_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4C_SPEC, bool, O>;
#[doc = "Field `FOC4B` writer - Force Output Compare 4B"]
pub type FOC4B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4C_SPEC, bool, O>;
#[doc = "Field `FOC4A` writer - Force Output Compare 4A"]
pub type FOC4A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4C_SPEC, bool, O>;
impl W {
    #[doc = "Bit 5 - Force Output Compare 4C"]
    #[inline(always)]
    #[must_use]
    pub fn foc4c(&mut self) -> FOC4C_W<5> {
        FOC4C_W::new(self)
    }
    #[doc = "Bit 6 - Force Output Compare 4B"]
    #[inline(always)]
    #[must_use]
    pub fn foc4b(&mut self) -> FOC4B_W<6> {
        FOC4B_W::new(self)
    }
    #[doc = "Bit 7 - Force Output Compare 4A"]
    #[inline(always)]
    #[must_use]
    pub fn foc4a(&mut self) -> FOC4A_W<7> {
        FOC4A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter 4 Control Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr4c](index.html) module"]
pub struct TCCR4C_SPEC;
impl crate::RegisterSpec for TCCR4C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr4c::R](R) reader structure"]
impl crate::Readable for TCCR4C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr4c::W](W) writer structure"]
impl crate::Writable for TCCR4C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4C to value 0"]
impl crate::Resettable for TCCR4C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
