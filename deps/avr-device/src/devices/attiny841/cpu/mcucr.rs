#[doc = "Register `MCUCR` reader"]
pub struct R(crate::R<MCUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUCR` writer"]
pub struct W(crate::W<MCUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUCR_SPEC>;
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
impl From<crate::W<MCUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISC0` reader - Interrupt Sense Control 0 bits"]
pub type ISC0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISC0` writer - Interrupt Sense Control 0 bits"]
pub type ISC0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MCUCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SM` reader - Sleep Mode Select Bits"]
pub type SM_R = crate::FieldReader<u8, SM_A>;
#[doc = "Sleep Mode Select Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM_A {
    #[doc = "0: Idle"]
    IDLE = 0,
    #[doc = "1: ADC Noise Reduction (If Available)"]
    ADC = 1,
    #[doc = "2: Power Down"]
    PDOWN = 2,
    #[doc = "3: Standby"]
    STDBY = 3,
}
impl From<SM_A> for u8 {
    #[inline(always)]
    fn from(variant: SM_A) -> Self {
        variant as _
    }
}
impl SM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM_A {
        match self.bits {
            0 => SM_A::IDLE,
            1 => SM_A::ADC,
            2 => SM_A::PDOWN,
            3 => SM_A::STDBY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SM_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == SM_A::ADC
    }
    #[doc = "Checks if the value of the field is `PDOWN`"]
    #[inline(always)]
    pub fn is_pdown(&self) -> bool {
        *self == SM_A::PDOWN
    }
    #[doc = "Checks if the value of the field is `STDBY`"]
    #[inline(always)]
    pub fn is_stdby(&self) -> bool {
        *self == SM_A::STDBY
    }
}
#[doc = "Field `SM` writer - Sleep Mode Select Bits"]
pub type SM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MCUCR_SPEC, u8, SM_A, 2, O>;
impl<'a, const O: u8> SM_W<'a, O> {
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(SM_A::IDLE)
    }
    #[doc = "ADC Noise Reduction (If Available)"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut W {
        self.variant(SM_A::ADC)
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub fn pdown(self) -> &'a mut W {
        self.variant(SM_A::PDOWN)
    }
    #[doc = "Standby"]
    #[inline(always)]
    pub fn stdby(self) -> &'a mut W {
        self.variant(SM_A::STDBY)
    }
}
#[doc = "Field `SE` reader - Sleep Enable"]
pub type SE_R = crate::BitReader<bool>;
#[doc = "Field `SE` writer - Sleep Enable"]
pub type SE_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Interrupt Sense Control 0 bits"]
    #[inline(always)]
    pub fn isc0(&self) -> ISC0_R {
        ISC0_R::new(self.bits & 3)
    }
    #[doc = "Bits 3:4 - Sleep Mode Select Bits"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - Sleep Enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt Sense Control 0 bits"]
    #[inline(always)]
    #[must_use]
    pub fn isc0(&mut self) -> ISC0_W<0> {
        ISC0_W::new(self)
    }
    #[doc = "Bits 3:4 - Sleep Mode Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<3> {
        SM_W::new(self)
    }
    #[doc = "Bit 5 - Sleep Enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<5> {
        SE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcucr](index.html) module"]
pub struct MCUCR_SPEC;
impl crate::RegisterSpec for MCUCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mcucr::R](R) reader structure"]
impl crate::Readable for MCUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcucr::W](W) writer structure"]
impl crate::Writable for MCUCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCR to value 0"]
impl crate::Resettable for MCUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
