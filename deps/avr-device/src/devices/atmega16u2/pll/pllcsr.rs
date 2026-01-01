#[doc = "Register `PLLCSR` reader"]
pub struct R(crate::R<PLLCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCSR` writer"]
pub struct W(crate::W<PLLCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCSR_SPEC>;
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
impl From<crate::W<PLLCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLOCK` reader - PLL Lock Status Bit"]
pub type PLOCK_R = crate::BitReader<bool>;
#[doc = "Field `PLLE` reader - PLL Enable Bit"]
pub type PLLE_R = crate::BitReader<bool>;
#[doc = "Field `PLLE` writer - PLL Enable Bit"]
pub type PLLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, PLLCSR_SPEC, bool, O>;
#[doc = "Field `PLLP` reader - PLL prescaler Bits"]
pub type PLLP_R = crate::FieldReader<u8, PLLP_A>;
#[doc = "PLL prescaler Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLP_A {
    #[doc = "3: Clock/4"]
    VAL_0X03 = 3,
    #[doc = "5: Clock/8"]
    VAL_0X05 = 5,
}
impl From<PLLP_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLP_A) -> Self {
        variant as _
    }
}
impl PLLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLP_A> {
        match self.bits {
            3 => Some(PLLP_A::VAL_0X03),
            5 => Some(PLLP_A::VAL_0X05),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X03`"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == PLLP_A::VAL_0X03
    }
    #[doc = "Checks if the value of the field is `VAL_0X05`"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == PLLP_A::VAL_0X05
    }
}
#[doc = "Field `PLLP` writer - PLL prescaler Bits"]
pub type PLLP_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PLLCSR_SPEC, u8, PLLP_A, 3, O>;
impl<'a, const O: u8> PLLP_W<'a, O> {
    #[doc = "Clock/4"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut W {
        self.variant(PLLP_A::VAL_0X03)
    }
    #[doc = "Clock/8"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut W {
        self.variant(PLLP_A::VAL_0X05)
    }
}
impl R {
    #[doc = "Bit 0 - PLL Lock Status Bit"]
    #[inline(always)]
    pub fn plock(&self) -> PLOCK_R {
        PLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Enable Bit"]
    #[inline(always)]
    pub fn plle(&self) -> PLLE_R {
        PLLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - PLL prescaler Bits"]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new((self.bits >> 2) & 7)
    }
}
impl W {
    #[doc = "Bit 1 - PLL Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn plle(&mut self) -> PLLE_W<1> {
        PLLE_W::new(self)
    }
    #[doc = "Bits 2:4 - PLL prescaler Bits"]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<2> {
        PLLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Status and Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcsr](index.html) module"]
pub struct PLLCSR_SPEC;
impl crate::RegisterSpec for PLLCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pllcsr::R](R) reader structure"]
impl crate::Readable for PLLCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcsr::W](W) writer structure"]
impl crate::Writable for PLLCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCSR to value 0"]
impl crate::Resettable for PLLCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
