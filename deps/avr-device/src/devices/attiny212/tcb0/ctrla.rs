#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: CLK_PER (No Prescaling)"]
    CLKDIV1 = 0,
    #[doc = "1: CLK_PER/2 (From Prescaler)"]
    CLKDIV2 = 1,
    #[doc = "2: Use Clock from TCA"]
    CLKTCA = 2,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::CLKDIV1),
            1 => Some(CLKSEL_A::CLKDIV2),
            2 => Some(CLKSEL_A::CLKTCA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLKDIV1`"]
    #[inline(always)]
    pub fn is_clkdiv1(&self) -> bool {
        *self == CLKSEL_A::CLKDIV1
    }
    #[doc = "Checks if the value of the field is `CLKDIV2`"]
    #[inline(always)]
    pub fn is_clkdiv2(&self) -> bool {
        *self == CLKSEL_A::CLKDIV2
    }
    #[doc = "Checks if the value of the field is `CLKTCA`"]
    #[inline(always)]
    pub fn is_clktca(&self) -> bool {
        *self == CLKSEL_A::CLKTCA
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLA_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "CLK_PER (No Prescaling)"]
    #[inline(always)]
    pub fn clkdiv1(self) -> &'a mut W {
        self.variant(CLKSEL_A::CLKDIV1)
    }
    #[doc = "CLK_PER/2 (From Prescaler)"]
    #[inline(always)]
    pub fn clkdiv2(self) -> &'a mut W {
        self.variant(CLKSEL_A::CLKDIV2)
    }
    #[doc = "Use Clock from TCA"]
    #[inline(always)]
    pub fn clktca(self) -> &'a mut W {
        self.variant(CLKSEL_A::CLKTCA)
    }
}
#[doc = "Field `SYNCUPD` reader - Synchronize Update"]
pub type SYNCUPD_R = crate::BitReader<bool>;
#[doc = "Field `SYNCUPD` writer - Synchronize Update"]
pub type SYNCUPD_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 4 - Synchronize Update"]
    #[inline(always)]
    pub fn syncupd(&self) -> SYNCUPD_R {
        SYNCUPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Run Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 1:2 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<1> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 4 - Synchronize Update"]
    #[inline(always)]
    #[must_use]
    pub fn syncupd(&mut self) -> SYNCUPD_W<4> {
        SYNCUPD_W::new(self)
    }
    #[doc = "Bit 6 - Run Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
