#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLC_SPEC>;
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
impl From<crate::W<CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPOVR` reader - Compare output value override"]
pub type CMPOVR_R = crate::BitReader<bool>;
#[doc = "Field `CMPOVR` writer - Compare output value override"]
pub type CMPOVR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, bool, O>;
#[doc = "Field `AUPDATE` reader - Auto update"]
pub type AUPDATE_R = crate::BitReader<bool>;
#[doc = "Field `AUPDATE` writer - Auto update"]
pub type AUPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, bool, O>;
#[doc = "Field `FIFTY` reader - Fifty percent waveform"]
pub type FIFTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFTY` writer - Fifty percent waveform"]
pub type FIFTY_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, bool, O>;
#[doc = "Field `CMPCSEL` reader - Compare C output select"]
pub type CMPCSEL_R = crate::BitReader<CMPCSEL_A>;
#[doc = "Compare C output select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCSEL_A {
    #[doc = "0: PWM A output"]
    PWMA = 0,
    #[doc = "1: PWM B output"]
    PWMB = 1,
}
impl From<CMPCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCSEL_A {
        match self.bits {
            false => CMPCSEL_A::PWMA,
            true => CMPCSEL_A::PWMB,
        }
    }
    #[doc = "Checks if the value of the field is `PWMA`"]
    #[inline(always)]
    pub fn is_pwma(&self) -> bool {
        *self == CMPCSEL_A::PWMA
    }
    #[doc = "Checks if the value of the field is `PWMB`"]
    #[inline(always)]
    pub fn is_pwmb(&self) -> bool {
        *self == CMPCSEL_A::PWMB
    }
}
#[doc = "Field `CMPCSEL` writer - Compare C output select"]
pub type CMPCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, CMPCSEL_A, O>;
impl<'a, const O: u8> CMPCSEL_W<'a, O> {
    #[doc = "PWM A output"]
    #[inline(always)]
    pub fn pwma(self) -> &'a mut W {
        self.variant(CMPCSEL_A::PWMA)
    }
    #[doc = "PWM B output"]
    #[inline(always)]
    pub fn pwmb(self) -> &'a mut W {
        self.variant(CMPCSEL_A::PWMB)
    }
}
#[doc = "Field `CMPDSEL` reader - Compare D output select"]
pub type CMPDSEL_R = crate::BitReader<CMPDSEL_A>;
#[doc = "Compare D output select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPDSEL_A {
    #[doc = "0: PWM A output"]
    PWMA = 0,
    #[doc = "1: PWM B output"]
    PWMB = 1,
}
impl From<CMPDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CMPDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDSEL_A {
        match self.bits {
            false => CMPDSEL_A::PWMA,
            true => CMPDSEL_A::PWMB,
        }
    }
    #[doc = "Checks if the value of the field is `PWMA`"]
    #[inline(always)]
    pub fn is_pwma(&self) -> bool {
        *self == CMPDSEL_A::PWMA
    }
    #[doc = "Checks if the value of the field is `PWMB`"]
    #[inline(always)]
    pub fn is_pwmb(&self) -> bool {
        *self == CMPDSEL_A::PWMB
    }
}
#[doc = "Field `CMPDSEL` writer - Compare D output select"]
pub type CMPDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, CMPDSEL_A, O>;
impl<'a, const O: u8> CMPDSEL_W<'a, O> {
    #[doc = "PWM A output"]
    #[inline(always)]
    pub fn pwma(self) -> &'a mut W {
        self.variant(CMPDSEL_A::PWMA)
    }
    #[doc = "PWM B output"]
    #[inline(always)]
    pub fn pwmb(self) -> &'a mut W {
        self.variant(CMPDSEL_A::PWMB)
    }
}
impl R {
    #[doc = "Bit 0 - Compare output value override"]
    #[inline(always)]
    pub fn cmpovr(&self) -> CMPOVR_R {
        CMPOVR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto update"]
    #[inline(always)]
    pub fn aupdate(&self) -> AUPDATE_R {
        AUPDATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Fifty percent waveform"]
    #[inline(always)]
    pub fn fifty(&self) -> FIFTY_R {
        FIFTY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare C output select"]
    #[inline(always)]
    pub fn cmpcsel(&self) -> CMPCSEL_R {
        CMPCSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare D output select"]
    #[inline(always)]
    pub fn cmpdsel(&self) -> CMPDSEL_R {
        CMPDSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare output value override"]
    #[inline(always)]
    #[must_use]
    pub fn cmpovr(&mut self) -> CMPOVR_W<0> {
        CMPOVR_W::new(self)
    }
    #[doc = "Bit 1 - Auto update"]
    #[inline(always)]
    #[must_use]
    pub fn aupdate(&mut self) -> AUPDATE_W<1> {
        AUPDATE_W::new(self)
    }
    #[doc = "Bit 3 - Fifty percent waveform"]
    #[inline(always)]
    #[must_use]
    pub fn fifty(&mut self) -> FIFTY_W<3> {
        FIFTY_W::new(self)
    }
    #[doc = "Bit 6 - Compare C output select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcsel(&mut self) -> CMPCSEL_W<6> {
        CMPCSEL_W::new(self)
    }
    #[doc = "Bit 7 - Compare D output select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpdsel(&mut self) -> CMPDSEL_W<7> {
        CMPDSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](index.html) module"]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlc::R](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
