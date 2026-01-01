#[doc = "Register `CALIB` reader"]
pub struct R(crate::R<CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALIB` writer"]
pub struct W(crate::W<CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIB_SPEC>;
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
impl From<crate::W<CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTYCYC` reader - Duty Cycle"]
pub type DUTYCYC_R = crate::BitReader<DUTYCYC_A>;
#[doc = "Duty Cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUTYCYC_A {
    #[doc = "0: 50% Duty cycle"]
    DUTY50 = 0,
    #[doc = "1: 25% Duty cycle"]
    DUTY25 = 1,
}
impl From<DUTYCYC_A> for bool {
    #[inline(always)]
    fn from(variant: DUTYCYC_A) -> Self {
        variant as u8 != 0
    }
}
impl DUTYCYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUTYCYC_A {
        match self.bits {
            false => DUTYCYC_A::DUTY50,
            true => DUTYCYC_A::DUTY25,
        }
    }
    #[doc = "Checks if the value of the field is `DUTY50`"]
    #[inline(always)]
    pub fn is_duty50(&self) -> bool {
        *self == DUTYCYC_A::DUTY50
    }
    #[doc = "Checks if the value of the field is `DUTY25`"]
    #[inline(always)]
    pub fn is_duty25(&self) -> bool {
        *self == DUTYCYC_A::DUTY25
    }
}
#[doc = "Field `DUTYCYC` writer - Duty Cycle"]
pub type DUTYCYC_W<'a, const O: u8> = crate::BitWriter<'a, u8, CALIB_SPEC, DUTYCYC_A, O>;
impl<'a, const O: u8> DUTYCYC_W<'a, O> {
    #[doc = "50% Duty cycle"]
    #[inline(always)]
    pub fn duty50(self) -> &'a mut W {
        self.variant(DUTYCYC_A::DUTY50)
    }
    #[doc = "25% Duty cycle"]
    #[inline(always)]
    pub fn duty25(self) -> &'a mut W {
        self.variant(DUTYCYC_A::DUTY25)
    }
}
impl R {
    #[doc = "Bit 0 - Duty Cycle"]
    #[inline(always)]
    pub fn dutycyc(&self) -> DUTYCYC_R {
        DUTYCYC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Duty Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn dutycyc(&mut self) -> DUTYCYC_W<0> {
        DUTYCYC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](index.html) module"]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [calib::R](R) reader structure"]
impl crate::Readable for CALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calib::W](W) writer structure"]
impl crate::Writable for CALIB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
