#[doc = "Register `TCCR2C` reader"]
pub struct R(crate::R<TCCR2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR2C` writer"]
pub struct W(crate::W<TCCR2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR2C_SPEC>;
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
impl From<crate::W<TCCR2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FOC2B` writer - Force Output Compare for Channel B"]
pub type FOC2B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR2C_SPEC, bool, O>;
#[doc = "Field `FOC2A` writer - Force Output Compare for Channel A"]
pub type FOC2A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR2C_SPEC, bool, O>;
impl W {
    #[doc = "Bit 6 - Force Output Compare for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn foc2b(&mut self) -> FOC2B_W<6> {
        FOC2B_W::new(self)
    }
    #[doc = "Bit 7 - Force Output Compare for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn foc2a(&mut self) -> FOC2A_W<7> {
        FOC2A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter2 Control Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr2c](index.html) module"]
pub struct TCCR2C_SPEC;
impl crate::RegisterSpec for TCCR2C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr2c::R](R) reader structure"]
impl crate::Readable for TCCR2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr2c::W](W) writer structure"]
impl crate::Writable for TCCR2C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR2C to value 0"]
impl crate::Resettable for TCCR2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
