#[doc = "Register `TCCR1C` reader"]
pub struct R(crate::R<TCCR1C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR1C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR1C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR1C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR1C` writer"]
pub struct W(crate::W<TCCR1C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR1C_SPEC>;
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
impl From<crate::W<TCCR1C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR1C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FOC1C` writer - Force Output Compare 1C"]
pub type FOC1C_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1C_SPEC, bool, O>;
#[doc = "Field `FOC1B` writer - Force Output Compare 1B"]
pub type FOC1B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1C_SPEC, bool, O>;
#[doc = "Field `FOC1A` writer - Force Output Compare 1A"]
pub type FOC1A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1C_SPEC, bool, O>;
impl W {
    #[doc = "Bit 5 - Force Output Compare 1C"]
    #[inline(always)]
    #[must_use]
    pub fn foc1c(&mut self) -> FOC1C_W<5> {
        FOC1C_W::new(self)
    }
    #[doc = "Bit 6 - Force Output Compare 1B"]
    #[inline(always)]
    #[must_use]
    pub fn foc1b(&mut self) -> FOC1B_W<6> {
        FOC1B_W::new(self)
    }
    #[doc = "Bit 7 - Force Output Compare 1A"]
    #[inline(always)]
    #[must_use]
    pub fn foc1a(&mut self) -> FOC1A_W<7> {
        FOC1A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter 1 Control Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr1c](index.html) module"]
pub struct TCCR1C_SPEC;
impl crate::RegisterSpec for TCCR1C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr1c::R](R) reader structure"]
impl crate::Readable for TCCR1C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr1c::W](W) writer structure"]
impl crate::Writable for TCCR1C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1C to value 0"]
impl crate::Resettable for TCCR1C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
