#[doc = "Register `INTCTRL` reader"]
pub struct R(crate::R<INTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCTRL` writer"]
pub struct W(crate::W<INTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCTRL_SPEC>;
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
impl From<crate::W<INTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEREADY` reader - EEPROM Ready"]
pub type EEREADY_R = crate::BitReader<bool>;
#[doc = "Field `EEREADY` writer - EEPROM Ready"]
pub type EEREADY_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EEPROM Ready"]
    #[inline(always)]
    pub fn eeready(&self) -> EEREADY_R {
        EEREADY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Ready"]
    #[inline(always)]
    #[must_use]
    pub fn eeready(&mut self) -> EEREADY_W<0> {
        EEREADY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intctrl](index.html) module"]
pub struct INTCTRL_SPEC;
impl crate::RegisterSpec for INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intctrl::R](R) reader structure"]
impl crate::Readable for INTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intctrl::W](W) writer structure"]
impl crate::Writable for INTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::Resettable for INTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
